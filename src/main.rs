mod app;
mod ascii;
mod cli;
mod config;
mod render;

use config::Config;
use glob::{glob_with, MatchOptions};
use std::io::{BufRead, IsTerminal};

fn visit_dirs(dir: &std::path::Path, files: &mut Vec<String>) -> std::io::Result<()> {
    if dir.is_dir() {
        let mut entries = std::fs::read_dir(dir)?
            .collect::<Result<Vec<_>, _>>()?;
        entries.sort_by_key(|e| e.path());
        for entry in entries {
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, files)?;
            } else if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
                let ext_lower = ext.to_lowercase();
                if ["png", "jpg", "jpeg", "gif", "webp", "bmp", "ico", "tiff", "tga"].contains(&ext_lower.as_str()) {
                    files.push(path.to_string_lossy().to_string());
                }
            }
        }
    }
    Ok(())
}

fn contains_glob_chars(s: &str) -> bool {
    s.contains('*') || s.contains('?') || s.contains('[')
}

#[cfg(windows)]
fn get_glob_options() -> MatchOptions {
    MatchOptions {
        case_sensitive: false,
        require_literal_separator: false,
        require_literal_leading_dot: false,
    }
}

#[cfg(not(windows))]
fn get_glob_options() -> MatchOptions {
    MatchOptions::default()
}

fn main() {
    let matches = cli::build_cli().get_matches();
    let conf = Config::new(&matches);

    let mut raw_files: Vec<String> = matches
        .get_many::<String>("file")
        .unwrap_or_default()
        .map(|s| s.to_string())
        .collect();

    if !std::io::stdin().is_terminal() {
        for line in std::io::stdin().lock().lines().map_while(Result::ok) {
            let trimmed = line.trim();
            if !trimmed.is_empty() {
                raw_files.push(trimmed.to_string());
            }
        }
    }

    if raw_files.is_empty() {
        eprintln!("Error: no image files specified");
        std::process::exit(1);
    }

    let mut processed_files = Vec::new();
    let mut temp_files = Vec::new();

    for f in raw_files {
        let is_url = f.starts_with("http://") || f.starts_with("https://");
        if is_url {
            println!("Downloading {} ...", f);
            match ureq::get(&f).call() {
                Ok(response) => {
                    let mut reader = response.into_reader();
                    let temp_res = tempfile::Builder::new()
                        .prefix("viua_")
                        .suffix(".png")
                        .tempfile();
                    match temp_res {
                        Ok(mut temp_file) => {
                            if let Err(e) = std::io::copy(&mut reader, &mut temp_file) {
                                eprintln!("warning: failed to write downloaded file: {}", e);
                            } else {
                                processed_files.push(temp_file.path().to_string_lossy().to_string());
                                temp_files.push(temp_file);
                            }
                        }
                        Err(e) => {
                            eprintln!("warning: failed to create temporary file: {}", e);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("warning: failed to download URL {}: {}", f, e);
                }
            }
        } else {
            let path = std::path::Path::new(&f);
            if path.exists() {
                if path.is_dir() {
                    if conf.recursive {
                        if let Err(e) = visit_dirs(path, &mut processed_files) {
                            eprintln!("warning: failed to read directory {}: {}", f, e);
                        }
                    } else {
                        eprintln!("warning: {} is a directory (use -r/--recursive to traverse)", f);
                    }
                } else {
                    processed_files.push(f);
                }
            } else if contains_glob_chars(&f) {
                // 路径不存在且包含glob通配符，尝试展开（Windows下cmd/powershell不会自动展开通配符）
                match glob_with(&f, get_glob_options()) {
                    Ok(entries) => {
                        let mut found = false;
                        for entry in entries {
                            match entry {
                                Ok(matched_path) => {
                                    if matched_path.is_file() {
                                        processed_files.push(matched_path.to_string_lossy().to_string());
                                        found = true;
                                    } else if matched_path.is_dir() {
                                        if conf.recursive {
                                            if let Err(e) = visit_dirs(&matched_path, &mut processed_files) {
                                                eprintln!("warning: failed to read directory {}: {}", matched_path.display(), e);
                                            }
                                            found = true;
                                        } else {
                                            eprintln!("warning: {} is a directory (use -r/--recursive to traverse)", matched_path.display());
                                        }
                                    }
                                }
                                Err(e) => {
                                    eprintln!("warning: failed to process glob entry for {}: {}", f, e);
                                }
                            }
                        }
                        if !found {
                            // 没有匹配到任何有效文件，保留原路径以触发文件不存在错误
                            processed_files.push(f);
                        }
                    }
                    Err(e) => {
                        eprintln!("warning: invalid glob pattern '{}': {}", f, e);
                        processed_files.push(f);
                    }
                }
            } else {
                // 普通不存在的文件，保留原路径触发后续错误提示
                processed_files.push(f);
            }
        }
    }

    if processed_files.is_empty() {
        eprintln!("Error: no valid image files found to process");
        std::process::exit(1);
    }

    if let Err(e) = app::run(&conf, &processed_files) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

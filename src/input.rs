use std::io::{BufRead, IsTerminal};

#[cfg(windows)]
use glob::MatchOptions;

#[allow(dead_code)]
pub fn contains_glob_chars(s: &str) -> bool {
    s.contains('*') || s.contains('?') || s.contains('[')
}

#[allow(dead_code)]
#[cfg(windows)]
pub fn get_glob_options() -> MatchOptions {
    MatchOptions {
        case_sensitive: false,
        require_literal_separator: false,
        require_literal_leading_dot: false,
    }
}

#[allow(dead_code)]
#[cfg(not(windows))]
pub fn get_glob_options() -> MatchOptions {
    MatchOptions::default()
}

pub fn collect_files(matches: &clap::ArgMatches) -> Vec<String> {
    let mut files: Vec<String> = Vec::new();

    if let Some((_, sub)) = matches.subcommand() {
        if let Some(f) = sub.get_many::<String>("file") {
            for x in f {
                files.push(x.to_string());
            }
        }
    } else if let Some(f) = matches.get_many::<String>("file") {
        for x in f {
            files.push(x.to_string());
        }
    }

    if !std::io::stdin().is_terminal() {
        for line in std::io::stdin().lock().lines().map_while(Result::ok) {
            let trimmed = line.trim();
            if !trimmed.is_empty() {
                files.push(trimmed.to_string());
            }
        }
    }

    files
}

fn download_url(url: &str, processed: &mut Vec<String>, _tmp: &mut Vec<tempfile::NamedTempFile>) {
    println!("Downloading {} ...", url);
    match ureq::get(url).call() {
        Ok(response) => {
            let mut reader = response.into_reader();
            match tempfile::Builder::new()
                .prefix("viua_")
                .suffix(".png")
                .tempfile()
            {
                Ok(mut tmp) => {
                    if let Err(e) = std::io::copy(&mut reader, &mut tmp) {
                        eprintln!("warning: failed to write downloaded file: {}", e);
                    } else {
                        processed.push(tmp.path().to_string_lossy().to_string());
                        _tmp.push(tmp);
                    }
                }
                Err(e) => eprintln!("warning: failed to create temporary file: {}", e),
            }
        }
        Err(e) => eprintln!("warning: failed to download URL {}: {}", url, e),
    }
}

pub fn resolve_files(raw: Vec<String>, recursive: bool) -> Vec<String> {
    let mut processed = Vec::new();
    let mut _temp_files = Vec::new();

    for f in raw {
        if f.starts_with("http://") || f.starts_with("https://") {
            download_url(&f, &mut processed, &mut _temp_files);
        } else {
            let path = std::path::Path::new(&f);
            if path.is_dir() {
                if recursive {
                    if let Err(e) = visit_dirs(path, &mut processed) {
                        eprintln!("warning: failed to read directory {}: {}", f, e);
                    }
                } else {
                    eprintln!(
                        "warning: {} is a directory (use -r/--recursive to traverse)",
                        f
                    );
                }
            } else {
                processed.push(f);
            }
        }
    }

    processed
}

fn visit_dirs(dir: &std::path::Path, files: &mut Vec<String>) -> std::io::Result<()> {
    if dir.is_dir() {
        let mut entries = std::fs::read_dir(dir)?.collect::<Result<Vec<_>, _>>()?;
        entries.sort_by_key(|e| e.path());
        for entry in entries {
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, files)?;
            } else if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
                let ext_lower = ext.to_lowercase();
                if [
                    "png", "jpg", "jpeg", "gif", "webp", "bmp", "ico", "tiff", "tga",
                ]
                .contains(&ext_lower.as_str())
                {
                    files.push(path.to_string_lossy().to_string());
                }
            }
        }
    }
    Ok(())
}

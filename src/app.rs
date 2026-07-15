use crate::ascii::{self, Algorithm};
use crate::config::{Config, ViewMode};
use crate::render::{Ansi, Renderer};
use crossterm::terminal::size;
use image::GenericImageView;
use std::io::stdout;

fn resolve_width(conf: &Config) -> u32 {
    let term_w = size().map(|(w, _)| w as u32).unwrap_or(80);

    if conf.width == 0 {
        return term_w;
    }
    if conf.width > term_w {
        eprintln!(
            "warning: width {} exceeds terminal ({}), clamping",
            conf.width, term_w
        );
        return term_w;
    }
    conf.width
}

fn ensure_iterm_detection() {
    if std::env::var("TERM_PROGRAM").is_ok() {
        return;
    }
    if std::env::var("WT_SESSION").is_ok() {
        std::env::set_var("TERM_PROGRAM", "iTerm");
    }
}

fn human_size(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB"];
    let mut size = bytes as f64;
    let mut unit = 0;
    while size >= 1024.0 && unit < UNITS.len() - 1 {
        size /= 1024.0;
        unit += 1;
    }
    if unit == 0 {
        format!("{} {}", bytes, UNITS[unit])
    } else {
        format!("{:.1} {}", size, UNITS[unit])
    }
}

fn print_footer(
    file: &str,
    orig_w: u32,
    orig_h: u32,
    format: &str,
    file_size: u64,
    show_info: bool,
    is_ascii: bool,
) {
    let term_w = size().map(|(w, _)| w as usize).unwrap_or(80);
    if term_w < 4 {
        return;
    }

    let text = if show_info {
        let base = format!(
            "{} · {}×{} · {} · {}",
            file,
            orig_w,
            orig_h,
            format,
            human_size(file_size)
        );
        if is_ascii {
            format!("{} · ascii by viua", base)
        } else {
            base
        }
    } else if is_ascii {
        "ascii by viua".to_string()
    } else {
        return;
    };

    let inner_w = term_w.saturating_sub(2);
    let text_w = text.chars().count();
    let pad_left = (inner_w.saturating_sub(text_w)) / 2;
    let pad_right = inner_w.saturating_sub(text_w) - pad_left;
    let fill = "─".repeat(inner_w);

    println!("╭{}╮", fill);
    println!(
        "│{}{}{}│",
        " ".repeat(pad_left),
        text,
        " ".repeat(pad_right)
    );
    println!("╰{}╯", fill);
}

fn print_separator() {
    let term_w = size().map(|(w, _)| w as usize).unwrap_or(80);
    println!("{}", "─".repeat(term_w));
}

pub fn run(conf: &Config, files: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    let actual_width = resolve_width(conf);
    ensure_iterm_detection();

    #[cfg(debug_assertions)]
    eprintln!(
        "viua: iterm={} sixel={}",
        viuer::is_iterm_supported(),
        viuer::is_sixel_supported()
    );

    let char_set: Vec<char> = if conf.charset.is_empty() {
        ascii::charset::DEFAULT_CHARSET.chars().collect()
    } else {
        conf.charset.chars().collect()
    };
    assert!(!char_set.is_empty(), "Charset must not be empty");

    for (i, file) in files.iter().enumerate() {
        let mut img = image::open(file)?;
        let (orig_w, orig_h) = img.dimensions();
        let file_size = std::fs::metadata(file).map(|m| m.len()).unwrap_or(0);
        let fmt = std::path::Path::new(file)
            .extension()
            .and_then(|e| e.to_str())
            .map(|e| e.to_uppercase())
            .unwrap_or_default();
        let is_ascii = matches!(conf.mode, ViewMode::Ascii);

        if conf.monochrome {
            img = img.grayscale();
        }

        match conf.mode {
            ViewMode::Ascii => {
                let art = ascii::convert(&img, actual_width, &char_set, Algorithm::Luminance);
                let renderer = Ansi {
                    color: !conf.monochrome,
                    monochrome: conf.monochrome,
                };
                renderer.render(&mut stdout(), &art)?;
            }
            ViewMode::Image => {
                let vcfg = viuer::Config {
                    width: Some(actual_width),
                    transparent: true,
                    absolute_offset: false,
                    ..Default::default()
                };
                if conf.monochrome {
                    viuer::print(&img, &vcfg)?;
                } else {
                    viuer::print_from_file(file, &vcfg)?;
                }
            }
            ViewMode::HalfBlock => {
                let vcfg = viuer::Config {
                    width: Some(actual_width),
                    use_kitty: false,
                    use_iterm: false,
                    use_sixel: false,
                    transparent: true,
                    absolute_offset: false,
                    ..Default::default()
                };
                if conf.monochrome {
                    viuer::print(&img, &vcfg)?;
                } else {
                    viuer::print_from_file(file, &vcfg)?;
                }
            }
        }

        print_footer(file, orig_w, orig_h, &fmt, file_size, conf.info, is_ascii);
        if i < files.len() - 1 {
            print_separator();
        }
        println!();
    }

    Ok(())
}

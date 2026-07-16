use crate::ascii::{self, Algorithm};
use crate::config::{Config, ViewMode};
use crate::render::{Ansi, Renderer};
use crossterm::terminal::size;
use image::GenericImageView;
use std::io::stdout;

fn resolve_dimensions(conf: &Config, orig_w: u32, orig_h: u32) -> (u32, u32) {
    let (term_w, _term_h) = size().map(|(w, h)| (w as u32, h as u32)).unwrap_or((80, 24));
    let aspect = orig_h as f64 / orig_w as f64;

    let (mut w, mut h) = if conf.width == 0 && conf.height == 0 {
        let new_w = term_w;
        let new_h = (new_w as f64 * aspect * 0.5).round() as u32;
        (new_w, new_h)
    } else if conf.width > 0 && conf.height == 0 {
        let new_w = conf.width;
        let new_h = (new_w as f64 * aspect * 0.5).round() as u32;
        (new_w, new_h)
    } else if conf.width == 0 && conf.height > 0 {
        let new_h = conf.height;
        let new_w = (new_h as f64 / (aspect * 0.5)).round() as u32;
        (new_w, new_h)
    } else {
        (conf.width, conf.height)
    };

    if w > term_w {
        eprintln!(
            "warning: width {} exceeds terminal ({}), clamping",
            w, term_w
        );
        let ratio = term_w as f64 / w as f64;
        w = term_w;
        h = (h as f64 * ratio).round() as u32;
    }

    (w.max(1), h.max(1))
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

fn viuer_print(
    file: &str,
    img: &image::DynamicImage,
    conf: &Config,
    actual_width: u32,
    actual_height: Option<u32>,
    use_image_protocols: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let vcfg = viuer::Config {
        width: Some(actual_width),
        height: actual_height,
        use_kitty: use_image_protocols,
        use_iterm: use_image_protocols,
        use_sixel: use_image_protocols,
        transparent: true,
        absolute_offset: false,
        ..Default::default()
    };
    if conf.monochrome {
        viuer::print(img, &vcfg)?;
    } else {
        viuer::print_from_file(file, &vcfg)?;
    }
    Ok(())
}

pub fn run(conf: &Config, files: &[String]) -> Result<(), Box<dyn std::error::Error>> {
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
        if !std::path::Path::new(file).exists() {
            eprintln!("warning: file not found — {file}");
            continue;
        }
        let mut img = match image::open(file) {
            Ok(img) => img,
            Err(e) => {
                eprintln!("warning: cannot open {file}: {e}");
                continue;
            }
        };
        let (orig_w, orig_h) = img.dimensions();
        let (w, h) = resolve_dimensions(conf, orig_w, orig_h);

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
                let art = ascii::convert(&img, w, h, &char_set, Algorithm::Luminance);
                let renderer = Ansi {
                    color: !conf.monochrome,
                    monochrome: conf.monochrome,
                };
                renderer.render(&mut stdout(), &art)?;
            }
            ViewMode::Image => viuer_print(
                file,
                &img,
                conf,
                w,
                if conf.height > 0 { Some(h) } else { None },
                true,
            )?,
            ViewMode::HalfBlock => viuer_print(
                file,
                &img,
                conf,
                w,
                if conf.height > 0 { Some(h) } else { None },
                false,
            )?,
        }

        print_footer(file, orig_w, orig_h, &fmt, file_size, conf.info, is_ascii);
        if i < files.len() - 1 {
            print_separator();
        }
        println!();
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_human_size_zero() {
        assert_eq!(human_size(0), "0 B");
    }

    #[test]
    fn test_human_size_bytes() {
        assert_eq!(human_size(1), "1 B");
        assert_eq!(human_size(1023), "1023 B");
    }

    #[test]
    fn test_human_size_kilobytes() {
        assert_eq!(human_size(1024), "1.0 KB");
        assert_eq!(human_size(1536), "1.5 KB");
        assert_eq!(human_size(1024 * 100), "100.0 KB");
    }

    #[test]
    fn test_human_size_megabytes() {
        assert_eq!(human_size(1024 * 1024), "1.0 MB");
        assert_eq!(human_size(1024 * 1024 * 5), "5.0 MB");
    }

    #[test]
    fn test_human_size_gigabytes() {
        assert_eq!(human_size(1024 * 1024 * 1024), "1.0 GB");
    }
}

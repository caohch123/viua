use crate::ascii::{self};
use crate::config::{Config, ViewMode};
use crate::render::{Ansi, Renderer};
use crossterm::terminal::size;
use crossterm::{cursor, execute};
use image::codecs::gif::GifDecoder;
use image::{AnimationDecoder, GenericImageView};
use std::io::{stdout, BufReader};
use std::time::{Duration, Instant};

fn resolve_dimensions(conf: &Config, orig_w: u32, orig_h: u32) -> (u32, u32) {
    let (term_w, _term_h) = size()
        .map(|(w, h)| (w as u32, h as u32))
        .unwrap_or((80, 24));
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
        #[cfg(any(feature = "sixel", feature = "icy_sixel"))]
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

fn play_gif(
    file: &str,
    conf: &Config,
    actual_width: u32,
    actual_height: Option<u32>,
    use_image_protocols: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let repeat = {
        let file_reader = std::fs::File::open(file)?;
        let mut options = gif::DecodeOptions::new();
        options.set_color_output(gif::ColorOutput::RGBA);
        let decoder = options.read_info(BufReader::new(file_reader))?;
        decoder.repeat()
    };
    let loop_count = match repeat {
        gif::Repeat::Infinite if conf.gif_once => 1,
        gif::Repeat::Infinite => usize::MAX,
        gif::Repeat::Finite(n) => n as usize,
    };

    let file_reader = std::fs::File::open(file)?;
    let frames: Vec<(u64, image::RgbaImage)> = GifDecoder::new(BufReader::new(file_reader))?
        .into_frames()
        .collect_frames()?
        .into_iter()
        .map(|f| {
            let delay = Duration::from(f.delay()).as_millis() as u64;
            let delay_ms = if delay < 20 { 100 } else { delay };
            (delay_ms, f.into_buffer())
        })
        .collect();

    if frames.is_empty() {
        return Ok(());
    }

    let (frame_w, frame_h) = frames[0].1.dimensions();
    let (term_w, term_h) = size()
        .map(|(w, h)| (w as u32, h as u32))
        .unwrap_or((80, 24));
    let mut width = actual_width.min(term_w).max(1);
    let mut height = actual_height
        .unwrap_or_else(|| {
            (width as f64 * frame_h as f64 / frame_w as f64 * 0.5).round() as u32
        })
        .max(1);
    let max_h = term_h.saturating_sub(1).max(1);
    if height > max_h {
        width = ((width as f64 * max_h as f64 / height as f64).round() as u32).max(1);
        height = max_h;
    }

    let vcfg = viuer::Config {
        width: Some(width),
        height: Some(height),
        use_kitty: use_image_protocols,
        use_iterm: use_image_protocols,
        #[cfg(any(feature = "sixel", feature = "icy_sixel"))]
        use_sixel: false,
        transparent: true,
        absolute_offset: false,
        ..Default::default()
    };

    for loop_i in 0..loop_count {
        let mut frame_start = Instant::now();
        for (frame_i, (delay_ms, img)) in frames.iter().enumerate() {
            let dyn_img = image::DynamicImage::ImageRgba8(img.clone());
            let (_print_w, print_h) = if conf.monochrome {
                viuer::print(&dyn_img.grayscale(), &vcfg)?
            } else {
                viuer::print(&dyn_img, &vcfg)?
            };

            let elapsed = frame_start.elapsed().as_millis() as u64;
            if elapsed < *delay_ms {
                std::thread::sleep(Duration::from_millis(delay_ms - elapsed));
            }
            frame_start = Instant::now();

            let is_last = loop_i + 1 == loop_count && frame_i + 1 == frames.len();
            if !is_last {
                execute!(stdout(), cursor::MoveUp(print_h as u16))?;
            }
        }
    }

    Ok(())
}

pub fn run(conf: &Config, files: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(all(debug_assertions, feature = "sixel"))]
    eprintln!(
        "viua: iterm={} sixel={}",
        viuer::is_iterm_supported(),
        viuer::is_sixel_supported()
    );
    #[cfg(all(debug_assertions, not(feature = "sixel")))]
    eprintln!("viua: iterm={}", viuer::is_iterm_supported());

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
                let art = ascii::convert(&img, w, h, &char_set, conf.algorithm);
                let renderer = Ansi {
                    color: !conf.monochrome,
                    monochrome: conf.monochrome,
                };
                renderer.render(&mut stdout(), &art)?;
            }
            ViewMode::Image => {
                if fmt == "GIF" {
                    play_gif(
                        file,
                        conf,
                        w,
                        if conf.height > 0 { Some(h) } else { None },
                        true,
                    )?;
                } else {
                    viuer_print(
                        file,
                        &img,
                        conf,
                        w,
                        if conf.height > 0 { Some(h) } else { None },
                        true,
                    )?;
                }
            }
            ViewMode::HalfBlock => {
                if fmt == "GIF" {
                    play_gif(
                        file,
                        conf,
                        w,
                        if conf.height > 0 { Some(h) } else { None },
                        false,
                    )?;
                } else {
                    viuer_print(
                        file,
                        &img,
                        conf,
                        w,
                        if conf.height > 0 { Some(h) } else { None },
                        false,
                    )?;
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

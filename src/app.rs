use crate::ascii::{self, Algorithm};
use crate::config::{Config, ViewMode};
use crate::render::{Ansi, Renderer};
use crossterm::terminal::size;
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

pub fn run(conf: &Config, files: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    let actual_width = resolve_width(conf);

    let char_set: Vec<char> = if conf.charset.is_empty() {
        ascii::charset::DEFAULT_CHARSET.chars().collect()
    } else {
        conf.charset.chars().collect()
    };
    assert!(!char_set.is_empty(), "Charset must not be empty");

    for file in files {
        let img = image::open(file)?;
        if conf.name {
            println!("{}:", file);
        }

        match conf.mode {
            ViewMode::Ascii => {
                let art = ascii::convert(&img, actual_width, &char_set, Algorithm::Luminance);
                let renderer = Ansi {
                    color: conf.color,
                    monochrome: conf.monochrome,
                };
                renderer.render(&mut stdout(), &art)?;
            }
            ViewMode::Image => {
                let vcfg = viuer::Config {
                    width: Some(actual_width),
                    transparent: true,
                    ..Default::default()
                };
                viuer::print(&img, &vcfg)?;
            }
            ViewMode::HalfBlock => {
                let vcfg = viuer::Config {
                    width: Some(actual_width),
                    use_kitty: false,
                    use_iterm: false,
                    transparent: true,
                    ..Default::default()
                };
                viuer::print(&img, &vcfg)?;
            }
        }

        if conf.caption {
            println!("{}", file);
        }
    }

    Ok(())
}

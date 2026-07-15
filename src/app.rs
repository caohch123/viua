use crate::ascii::{self, Algorithm};
use crate::config::{Config, ViewMode};
use crate::render::{Ansi, Renderer};
use std::io::stdout;

pub fn run(conf: &Config, files: &[String]) -> Result<(), Box<dyn std::error::Error>> {
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
                let art = ascii::convert(&img, conf.width, &char_set, Algorithm::Luminance);
                let renderer = Ansi {
                    color: conf.color,
                    monochrome: conf.monochrome,
                };
                renderer.render(&mut stdout(), &art)?;
            }
            ViewMode::Image => {
                let vcfg = viuer::Config {
                    width: Some(conf.width),
                    transparent: true,
                    ..Default::default()
                };
                viuer::print(&img, &vcfg)?;
            }
            ViewMode::HalfBlock => {
                let vcfg = viuer::Config {
                    width: Some(conf.width),
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

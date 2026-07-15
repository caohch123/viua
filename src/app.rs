use crate::ascii::convert;
use crate::charset;
use crate::config::Config;
use crate::render::{render, render_html};
use std::fs::File;
use std::io::{stdout, BufWriter};

pub fn run(conf: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let char_set = if conf.charset.is_empty() {
        charset::DEFAULT_CHARSET.chars().collect::<Vec<_>>()
    } else {
        conf.charset.chars().collect::<Vec<_>>()
    };
    assert!(!char_set.is_empty(), "Charset must not be empty");

    let mut out = stdout();
    for file in &conf.files {
        let img = image::open(file)?;
        if conf.name {
            println!("{}:", file);
        }
        let art = convert(&img, conf.width, &char_set);

        render(&mut out, &art, conf.color, conf.monochrome)?;

        if let Some(ref path) = conf.output {
            let f = File::create(path)?;
            let mut writer = BufWriter::new(f);
            render(&mut writer, &art, false, true)?;
        }

        if let Some(ref path) = conf.html {
            let f = File::create(path)?;
            let mut writer = BufWriter::new(f);
            render_html(&mut writer, &art, file)?;
        }

        if conf.caption {
            println!("{}", file);
        }
    }

    Ok(())
}

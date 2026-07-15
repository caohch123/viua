use clap::ArgMatches;

use crate::ascii::Algorithm;

pub enum ViewMode {
    Ascii,
    Image,
    HalfBlock,
}

pub struct Config {
    pub mode: ViewMode,
    #[allow(dead_code)]
    pub algorithm: Algorithm,
    pub width: u32,
    pub color: bool,
    pub monochrome: bool,
    pub charset: String,
    pub name: bool,
    pub caption: bool,
    // pub output: Option<String>,
    // pub html: Option<String>,
}

impl Config {
    pub fn new(matches: &ArgMatches) -> Self {
        let width = *matches.get_one::<u32>("width").expect("width has default");
        let color = matches.get_flag("color");
        let monochrome = matches.get_flag("monochrome");
        let charset = matches
            .get_one::<String>("charset")
            .cloned()
            .unwrap_or_default();
        let name = matches.get_flag("name");
        let caption = matches.get_flag("caption");
        let mode = match matches
            .get_one::<String>("mode")
            .map(|s| s.as_str())
            .unwrap_or("image")
        {
            "halfblock" => ViewMode::HalfBlock,
            "ascii" => ViewMode::Ascii,
            _ => ViewMode::Image,
        };
        // let output = matches.get_one::<String>("output").cloned();
        // let html = matches.get_one::<String>("html").cloned();

        Config {
            mode,
            algorithm: Algorithm::Luminance,
            width,
            color,
            monochrome,
            charset,
            name,
            caption,
        }
    }

    #[cfg(test)]
    #[allow(unused)]
    pub fn test_config() -> Self {
        Config {
            mode: ViewMode::Ascii,
            algorithm: Algorithm::Luminance,
            width: 80,
            color: false,
            monochrome: false,
            charset: String::new(),
            name: false,
            caption: false,
        }
    }
}

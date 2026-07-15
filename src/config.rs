use clap::ArgMatches;

use crate::ascii::Algorithm;

#[derive(Debug)]
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
    pub monochrome: bool,
    pub charset: String,
    pub info: bool,
    // pub output: Option<String>,
    // pub html: Option<String>,
}

impl Config {
    pub fn new(matches: &ArgMatches) -> Self {
        let width = *matches.get_one::<u32>("width").expect("width has default");
        let monochrome = matches.get_flag("monochrome");
        let charset = matches
            .get_one::<String>("charset")
            .cloned()
            .unwrap_or_default();
        let info = matches.get_flag("info");
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
            monochrome,
            charset,
            info,
        }
    }

    #[cfg(test)]
    #[allow(unused)]
    pub fn test_config() -> Self {
        Config {
            mode: ViewMode::Ascii,
            algorithm: Algorithm::Luminance,
            width: 80,
            monochrome: false,
            charset: String::new(),
            info: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli;

    fn parse_config(args: &[&str]) -> Config {
        let matches = cli::build_cli().try_get_matches_from(args).unwrap();
        Config::new(&matches)
    }

    #[test]
    fn test_defaults() {
        let conf = parse_config(&["viua", "img.png"]);
        assert!(matches!(conf.mode, ViewMode::Image));
        assert_eq!(conf.width, 0);
        assert!(!conf.monochrome);
        assert!(!conf.info);
        assert_eq!(conf.charset, " .:-=+*#%@");
    }

    #[test]
    fn test_mode_ascii() {
        let conf = parse_config(&["viua", "-M", "ascii", "img.png"]);
        assert!(matches!(conf.mode, ViewMode::Ascii));
    }

    #[test]
    fn test_mode_halfblock() {
        let conf = parse_config(&["viua", "-M", "halfblock", "img.png"]);
        assert!(matches!(conf.mode, ViewMode::HalfBlock));
    }

    #[test]
    fn test_monochrome() {
        let conf = parse_config(&["viua", "-m", "img.png"]);
        assert!(conf.monochrome);
    }

    #[test]
    fn test_info() {
        let conf = parse_config(&["viua", "-i", "img.png"]);
        assert!(conf.info);
    }

    #[test]
    fn test_width_custom() {
        let conf = parse_config(&["viua", "-w", "60", "img.png"]);
        assert_eq!(conf.width, 60);
    }

    #[test]
    fn test_charset_custom() {
        let conf = parse_config(&["viua", "-s", " .-+#", "img.png"]);
        assert_eq!(conf.charset, " .-+#");
    }
}

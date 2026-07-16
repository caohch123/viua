use clap::ArgMatches;

use crate::ascii::Algorithm;

#[derive(Debug, Clone, Copy)]
pub enum ViewMode {
    Ascii,
    Image,
    HalfBlock,
}

pub struct Config {
    pub mode: ViewMode,
    pub algorithm: Algorithm,
    pub width: u32,
    pub height: u32,
    pub monochrome: bool,
    pub charset: String,
    pub info: bool,
    pub recursive: bool,
    pub gif_once: bool,
}

impl Config {
    pub fn new(matches: &ArgMatches, mode: ViewMode, ascii_matches: Option<&ArgMatches>) -> Self {
        let width = *matches.get_one::<u32>("width").expect("width has default");
        let height = *matches
            .get_one::<u32>("height")
            .expect("height has default");
        let recursive = matches.get_flag("recursive");
        let monochrome = matches.get_flag("monochrome");
        let info = matches.get_flag("info");
        let gif_once = matches.get_flag("once");

        let (charset, algorithm) = if let Some(am) = ascii_matches {
            let cs = am.get_one::<String>("charset").cloned().unwrap_or_default();
            let algo = match am
                .get_one::<String>("algorithm")
                .map(|s| s.as_str())
                .unwrap_or("lum")
            {
                "lum-clahe" => Algorithm::LuminanceClahe,
                "sobel" => Algorithm::Sobel,
                _ => Algorithm::Luminance,
            };
            (cs, algo)
        } else {
            (String::new(), Algorithm::Luminance)
        };

        Config {
            mode,
            algorithm,
            width,
            height,
            monochrome,
            charset,
            info,
            recursive,
            gif_once,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli;

    #[test]
    fn test_default_mode_is_image() {
        let m = cli::build_cli()
            .try_get_matches_from(["viua", "img.png"])
            .unwrap();
        let conf = Config::new(&m, ViewMode::Image, None);
        assert!(matches!(conf.mode, ViewMode::Image));
        assert_eq!(conf.width, 0);
        assert_eq!(conf.height, 0);
        assert!(!conf.monochrome);
        assert!(!conf.info);
        assert!(!conf.recursive);
        assert_eq!(conf.charset, "");
    }

    #[test]
    fn test_ascii_mode() {
        let m = cli::build_cli()
            .try_get_matches_from(["viua", "ascii", "img.png"])
            .unwrap();
        let (_, sub) = m.subcommand().unwrap();
        let conf = Config::new(&m, ViewMode::Ascii, Some(sub));
        assert!(matches!(conf.mode, ViewMode::Ascii));
        assert_eq!(conf.charset, " .:-=+*#%@");
        assert!(matches!(conf.algorithm, Algorithm::Luminance));
    }

    #[test]
    fn test_ascii_algorithm_clahe() {
        let m = cli::build_cli()
            .try_get_matches_from(["viua", "ascii", "-a", "lum-clahe", "img.png"])
            .unwrap();
        let (_, sub) = m.subcommand().unwrap();
        let conf = Config::new(&m, ViewMode::Ascii, Some(sub));
        assert!(matches!(conf.algorithm, Algorithm::LuminanceClahe));
    }

    #[test]
    fn test_ascii_charset_custom() {
        let m = cli::build_cli()
            .try_get_matches_from(["viua", "ascii", "-s", " .-+#", "img.png"])
            .unwrap();
        let (_, sub) = m.subcommand().unwrap();
        let conf = Config::new(&m, ViewMode::Ascii, Some(sub));
        assert_eq!(conf.charset, " .-+#");
    }

    #[test]
    fn test_ascii_algorithm_sobel() {
        let m = cli::build_cli()
            .try_get_matches_from(["viua", "ascii", "-a", "sobel", "img.png"])
            .unwrap();
        let (_, sub) = m.subcommand().unwrap();
        let conf = Config::new(&m, ViewMode::Ascii, Some(sub));
        assert!(matches!(conf.algorithm, Algorithm::Sobel));
    }

    #[test]
    fn test_halfblock_mode() {
        let m = cli::build_cli()
            .try_get_matches_from(["viua", "halfblock", "img.png"])
            .unwrap();
        assert!(m.subcommand().is_some());
        let conf = Config::new(&m, ViewMode::HalfBlock, None);
        assert!(matches!(conf.mode, ViewMode::HalfBlock));
        assert_eq!(conf.charset, "");
    }

    #[test]
    fn test_monochrome() {
        let m = cli::build_cli()
            .try_get_matches_from(["viua", "-m", "img.png"])
            .unwrap();
        let conf = Config::new(&m, ViewMode::Image, None);
        assert!(conf.monochrome);
    }

    #[test]
    fn test_info() {
        let m = cli::build_cli()
            .try_get_matches_from(["viua", "-i", "img.png"])
            .unwrap();
        let conf = Config::new(&m, ViewMode::Image, None);
        assert!(conf.info);
    }

    #[test]
    fn test_width_custom() {
        let m = cli::build_cli()
            .try_get_matches_from(["viua", "-w", "60", "img.png"])
            .unwrap();
        let conf = Config::new(&m, ViewMode::Image, None);
        assert_eq!(conf.width, 60);
    }

    #[test]
    fn test_height_custom() {
        let m = cli::build_cli()
            .try_get_matches_from(["viua", "-H", "40", "img.png"])
            .unwrap();
        let conf = Config::new(&m, ViewMode::Image, None);
        assert_eq!(conf.height, 40);
    }

    #[test]
    fn test_recursive() {
        let m = cli::build_cli()
            .try_get_matches_from(["viua", "-r", "img.png"])
            .unwrap();
        let conf = Config::new(&m, ViewMode::Image, None);
        assert!(conf.recursive);
    }
}

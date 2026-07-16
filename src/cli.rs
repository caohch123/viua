use clap::{crate_description, crate_name, crate_version, value_parser, Arg, ArgAction, Command};

pub fn build_cli() -> Command {
    Command::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .arg_required_else_help(true)
        .args_override_self(true)
        .arg(
            Arg::new("file")
                .help("Image file(s) to convert")
                .action(ArgAction::Append),
        )
        .arg(
            Arg::new("width")
                .short('w')
                .long("width")
                .default_value("0")
                .value_parser(value_parser!(u32))
                .global(true)
                .help("Output width in characters (0 = auto, fit terminal)"),
        )
        .arg(
            Arg::new("height")
                .short('H')
                .long("height")
                .default_value("0")
                .value_parser(value_parser!(u32))
                .global(true)
                .help("Output height in characters (0 = auto)"),
        )
        .arg(
            Arg::new("recursive")
                .short('r')
                .long("recursive")
                .action(ArgAction::SetTrue)
                .global(true)
                .help("Recursively traverse directories"),
        )
        .arg(
            Arg::new("monochrome")
                .short('m')
                .long("monochrome")
                .action(ArgAction::SetTrue)
                .global(true)
                .help("Force monochrome output"),
        )
        .arg(
            Arg::new("info")
                .short('i')
                .long("info")
                .action(ArgAction::SetTrue)
                .global(true)
                .help("Show file info footer after each image"),
        )
        .subcommand(
            Command::new("ascii")
                .about("Convert images to ASCII art")
                .arg(
                    Arg::new("file")
                        .help("Image file(s) to convert")
                        .action(ArgAction::Append),
                )
                .arg(
                    Arg::new("algorithm")
                        .short('a')
                        .long("algorithm")
                        .default_value("lum")
                        .value_parser(["lum", "lum-clahe", "sobel"])
                        .help("Conversion algorithm: lum, lum-clahe, or sobel"),
                )
                .arg(
                    Arg::new("charset")
                        .short('s')
                        .long("charset")
                        .default_value(" .:-=+*#%@")
                        .help("Characters used for ASCII art (dark to light)"),
                ),
        )
        .subcommand(
            Command::new("image")
                .about("Display images in terminal (default)")
                .arg(
                    Arg::new("file")
                        .help("Image file(s) to display")
                        .action(ArgAction::Append),
                ),
        )
        .subcommand(
            Command::new("halfblock")
                .about("Display images using half-block characters")
                .arg(
                    Arg::new("file")
                        .help("Image file(s) to display")
                        .action(ArgAction::Append),
                ),
        )
        .after_help(
            "EXAMPLES:\n  \
             viua img.png                        Display image (default)\n  \
             viua -w 60 -i img.png               Custom width + info footer\n  \
             viua ascii img.png                  Convert to ASCII art\n  \
              viua ascii -a lum-clahe img.png     ASCII with CLAHE enhancement\n  \
              viua ascii -a sobel img.png       ASCII with Sobel edge detection\n  \
             viua halfblock -m img.png           Half-block, grayscale\n  \
             find . -name '*.png' | viua ascii   Pipe filenames from stdin",
        )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_mode_is_image() {
        let m = build_cli()
            .try_get_matches_from(["viua", "img.png"])
            .unwrap();
        assert!(m.subcommand().is_none());
    }

    #[test]
    fn test_ascii_subcommand() {
        let m = build_cli()
            .try_get_matches_from(["viua", "ascii", "img.png"])
            .unwrap();
        let (name, sub) = m.subcommand().unwrap();
        assert_eq!(name, "ascii");
        assert_eq!(sub.get_one::<String>("algorithm").unwrap(), "lum");
    }

    #[test]
    fn test_ascii_algorithm() {
        let m = build_cli()
            .try_get_matches_from(["viua", "ascii", "-a", "lum-clahe", "img.png"])
            .unwrap();
        let (_, sub) = m.subcommand().unwrap();
        assert_eq!(sub.get_one::<String>("algorithm").unwrap(), "lum-clahe");
    }

    #[test]
    fn test_image_subcommand() {
        let m = build_cli()
            .try_get_matches_from(["viua", "image", "img.png"])
            .unwrap();
        assert_eq!(m.subcommand().unwrap().0, "image");
    }

    #[test]
    fn test_ascii_algorithm_sobel() {
        let m = build_cli()
            .try_get_matches_from(["viua", "ascii", "-a", "sobel", "img.png"])
            .unwrap();
        let (_, sub) = m.subcommand().unwrap();
        assert_eq!(sub.get_one::<String>("algorithm").unwrap(), "sobel");
    }

    #[test]
    fn test_halfblock_subcommand() {
        let m = build_cli()
            .try_get_matches_from(["viua", "halfblock", "img.png"])
            .unwrap();
        assert_eq!(m.subcommand().unwrap().0, "halfblock");
    }

    #[test]
    fn test_default_width() {
        let m = build_cli()
            .try_get_matches_from(["viua", "img.png"])
            .unwrap();
        assert_eq!(*m.get_one::<u32>("width").unwrap(), 0);
    }

    #[test]
    fn test_width_flag() {
        let m = build_cli()
            .try_get_matches_from(["viua", "-w", "60", "img.png"])
            .unwrap();
        assert_eq!(*m.get_one::<u32>("width").unwrap(), 60);
    }

    #[test]
    fn test_monochrome_flag() {
        let m = build_cli()
            .try_get_matches_from(["viua", "-m", "img.png"])
            .unwrap();
        assert!(m.get_flag("monochrome"));
    }

    #[test]
    fn test_info_flag() {
        let m = build_cli()
            .try_get_matches_from(["viua", "-i", "img.png"])
            .unwrap();
        assert!(m.get_flag("info"));
    }

    #[test]
    fn test_multiple_files() {
        let m = build_cli()
            .try_get_matches_from(["viua", "a.png", "b.png", "c.jpg"])
            .unwrap();
        let files: Vec<&str> = m
            .get_many::<String>("file")
            .unwrap()
            .map(|s| s.as_str())
            .collect();
        assert_eq!(files, vec!["a.png", "b.png", "c.jpg"]);
    }

    #[test]
    fn test_default_height() {
        let m = build_cli()
            .try_get_matches_from(["viua", "img.png"])
            .unwrap();
        assert_eq!(*m.get_one::<u32>("height").unwrap(), 0);
    }

    #[test]
    fn test_height_flag() {
        let m = build_cli()
            .try_get_matches_from(["viua", "-H", "40", "img.png"])
            .unwrap();
        assert_eq!(*m.get_one::<u32>("height").unwrap(), 40);
    }

    #[test]
    fn test_recursive_flag() {
        let m = build_cli()
            .try_get_matches_from(["viua", "-r", "img.png"])
            .unwrap();
        assert!(m.get_flag("recursive"));
    }
}

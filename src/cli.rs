use clap::{crate_description, crate_name, crate_version, value_parser, Arg, ArgAction, Command};

pub fn build_cli() -> Command {
    Command::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .arg_required_else_help(true)
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
                .help("Output width in characters (0 = auto, fit terminal)"),
        )
        .arg(
            Arg::new("height")
                .short('H')
                .long("height")
                .default_value("0")
                .value_parser(value_parser!(u32))
                .help("Output height in characters (0 = auto)"),
        )
        .arg(
            Arg::new("recursive")
                .short('r')
                .long("recursive")
                .action(ArgAction::SetTrue)
                .help("Recursively traverse directories"),
        )
        .arg(
            Arg::new("monochrome")
                .short('m')
                .long("monochrome")
                .action(ArgAction::SetTrue)
                .help("Force monochrome output"),
        )
        .arg(
            Arg::new("charset")
                .short('s')
                .long("charset")
                .default_value(" .:-=+*#%@")
                .help("Characters used for ASCII art (dark to light)"),
        )
        .arg(
            Arg::new("info")
                .short('i')
                .long("info")
                .action(ArgAction::SetTrue)
                .help("Show file info footer after each image"),
        )
        .arg(
            Arg::new("algorithm")
                .short('a')
                .long("algorithm")
                .default_value("lum")
                .value_parser(["lum", "lum-clahe"])
                .help("ASCII algorithm: lum or lum-clahe"),
        )
        .arg(
            Arg::new("mode")
                .short('M')
                .long("mode")
                .default_value("image")
                .value_parser(["ascii", "image", "halfblock"])
                .help("Render mode: ascii, image, or halfblock"),
        )
    // .arg(
    //     Arg::new("output")
    //         .short('o')
    //         .long("output")
    //         .value_name("FILE")
    //         .help("Save ASCII art as plain text to FILE"),
    // )
    // .arg(
    //     Arg::new("html")
    //         .long("html")
    //         .value_name("FILE")
    //         .help("Save ASCII art as colored HTML to FILE"),
    // )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(args: &[&str]) -> clap::ArgMatches {
        build_cli().try_get_matches_from(args).unwrap()
    }

    #[test]
    fn test_default_width() {
        let m = parse(&["viua", "img.png"]);
        assert_eq!(*m.get_one::<u32>("width").unwrap(), 0);
    }

    #[test]
    fn test_width_flag() {
        let m = parse(&["viua", "-w", "60", "img.png"]);
        assert_eq!(*m.get_one::<u32>("width").unwrap(), 60);
    }

    #[test]
    fn test_default_mode() {
        let m = parse(&["viua", "img.png"]);
        assert_eq!(m.get_one::<String>("mode").unwrap(), "image");
    }

    #[test]
    fn test_mode_ascii() {
        let m = parse(&["viua", "-M", "ascii", "img.png"]);
        assert_eq!(m.get_one::<String>("mode").unwrap(), "ascii");
    }

    #[test]
    fn test_mode_halfblock() {
        let m = parse(&["viua", "-M", "halfblock", "img.png"]);
        assert_eq!(m.get_one::<String>("mode").unwrap(), "halfblock");
    }

    #[test]
    fn test_monochrome_flag() {
        let m = parse(&["viua", "-m", "img.png"]);
        assert!(m.get_flag("monochrome"));
    }

    #[test]
    fn test_info_flag() {
        let m = parse(&["viua", "-i", "img.png"]);
        assert!(m.get_flag("info"));
    }

    #[test]
    fn test_multiple_files() {
        let m = parse(&["viua", "a.png", "b.png", "c.jpg"]);
        let files: Vec<&str> = m
            .get_many::<String>("file")
            .unwrap()
            .map(|s| s.as_str())
            .collect();
        assert_eq!(files, vec!["a.png", "b.png", "c.jpg"]);
    }

    #[test]
    fn test_default_height() {
        let m = parse(&["viua", "img.png"]);
        assert_eq!(*m.get_one::<u32>("height").unwrap(), 0);
    }

    #[test]
    fn test_height_flag() {
        let m = parse(&["viua", "-H", "40", "img.png"]);
        assert_eq!(*m.get_one::<u32>("height").unwrap(), 40);
    }

    #[test]
    fn test_recursive_flag() {
        let m = parse(&["viua", "-r", "img.png"]);
        assert!(m.get_flag("recursive"));
    }
}

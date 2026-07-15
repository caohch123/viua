mod app;
mod ascii;
mod charset;
mod config;
mod render;

use clap::{crate_description, crate_name, crate_version, value_parser, Arg, ArgAction, Command};
use config::Config;

fn main() {
    let matches = Command::new(crate_name!())
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
                .default_value("80")
                .value_parser(value_parser!(u32))
                .help("Output width in characters"),
        )
        .arg(
            Arg::new("color")
                .short('c')
                .long("color")
                .action(ArgAction::SetTrue)
                .help("Output with ANSI truecolor"),
        )
        .arg(
            Arg::new("monochrome")
                .short('m')
                .long("monochrome")
                .action(ArgAction::SetTrue)
                .help("Force monochrome output even with --color"),
        )
        .arg(
            Arg::new("charset")
                .short('s')
                .long("charset")
                .default_value(" .:-=+*#%@")
                .help("Characters used for ASCII art (dark to light)"),
        )
        .arg(
            Arg::new("name")
                .short('n')
                .long("name")
                .action(ArgAction::SetTrue)
                .help("Print filename before each image"),
        )
        .arg(
            Arg::new("caption")
                .short('t')
                .long("caption")
                .action(ArgAction::SetTrue)
                .help("Print filename after each image"),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .value_name("FILE")
                .help("Save ASCII art as plain text to FILE"),
        )
        .arg(
            Arg::new("html")
                .long("html")
                .value_name("FILE")
                .help("Save ASCII art as colored HTML to FILE"),
        )
        .get_matches();

    let conf = Config::new(&matches);

    if let Err(e) = app::run(&conf) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

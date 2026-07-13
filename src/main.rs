mod ascii;
mod charset;
mod config;
mod render;

use ascii::convert;
use clap::{crate_description, crate_name, crate_version, value_parser, Arg, ArgAction, Command};
use config::Config;
use render::render;
use std::io::stdout;

fn main() -> Result<(), Box<dyn std::error::Error>> {
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
        .get_matches();

    let conf = Config::new(&matches);

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
        if conf.caption {
            println!("{}", file);
        }
    }

    Ok(())
}

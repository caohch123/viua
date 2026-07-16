mod app;
mod ascii;
mod cli;
mod config;
mod glob;
mod input;
mod render;

use config::{Config, ViewMode};

fn main() {
    let matches = cli::build_cli().get_matches();

    let (mode, sub_matches) = match matches.subcommand() {
        Some(("ascii", m)) => (ViewMode::Ascii, Some(m)),
        Some(("image", _)) => (ViewMode::Image, None),
        Some(("halfblock", _)) => (ViewMode::HalfBlock, None),
        _ => (ViewMode::Image, None),
    };

    let conf = Config::new(&matches, mode, sub_matches);
    let raw_files = input::collect_files(&matches);

    if raw_files.is_empty() {
        eprintln!("Error: no image files specified");
        std::process::exit(1);
    }

    let files = input::resolve_files(raw_files, conf.recursive);

    if files.is_empty() {
        eprintln!("Error: no valid image files found to process");
        std::process::exit(1);
    }

    if let Err(e) = app::run(&conf, &files) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

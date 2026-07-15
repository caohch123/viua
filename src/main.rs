mod app;
mod ascii;
mod cli;
mod config;
mod render;

use config::Config;
use std::io::{BufRead, IsTerminal};

fn main() {
    let matches = cli::build_cli().get_matches();
    let conf = Config::new(&matches);

    let mut files: Vec<String> = matches
        .get_many::<String>("file")
        .unwrap_or_default()
        .map(|s| s.to_string())
        .collect();

    if !std::io::stdin().is_terminal() {
        for line in std::io::stdin().lock().lines().map_while(Result::ok) {
            let trimmed = line.trim();
            if !trimmed.is_empty() {
                files.push(trimmed.to_string());
            }
        }
    }

    if files.is_empty() {
        eprintln!("Error: no image files specified");
        std::process::exit(1);
    }

    if let Err(e) = app::run(&conf, &files) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

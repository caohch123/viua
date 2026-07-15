mod app;
mod ascii;
mod cli;
mod config;
mod render;

use config::Config;

fn main() {
    let matches = cli::build_cli().get_matches();
    let conf = Config::new(&matches);

    let files: Vec<String> = matches
        .get_many::<String>("file")
        .unwrap_or_default()
        .map(|s| s.to_string())
        .collect();

    if files.is_empty() {
        eprintln!("Error: no image files specified");
        std::process::exit(1);
    }

    if let Err(e) = app::run(&conf, &files) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

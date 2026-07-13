use clap::ArgMatches;

pub struct Config {
    pub files: Vec<String>,
    pub width: u32,
    pub color: bool,
    pub monochrome: bool,
    pub charset: String,
    pub name: bool,
    pub caption: bool,
}

impl Config {
    pub fn new(matches: &ArgMatches) -> Self {
        let files: Vec<String> = matches
            .get_many::<String>("file")
            .unwrap_or_default()
            .map(|s| s.to_string())
            .collect();

        let width = *matches.get_one::<u32>("width").expect("width has default");
        let color = matches.get_flag("color");
        let monochrome = matches.get_flag("monochrome");
        let charset = matches
            .get_one::<String>("charset")
            .cloned()
            .unwrap_or_default();
        let name = matches.get_flag("name");
        let caption = matches.get_flag("caption");

        Config {
            files,
            width,
            color,
            monochrome,
            charset,
            name,
            caption,
        }
    }

    #[cfg(test)]
    pub fn test_config() -> Self {
        Config {
            files: vec![],
            width: 80,
            color: false,
            monochrome: false,
            charset: String::new(),
            name: false,
            caption: false,
        }
    }
}

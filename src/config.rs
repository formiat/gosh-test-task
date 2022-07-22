use clap::{Arg, ArgMatches, Command};

#[derive(Clone, Debug)]
pub struct Config {
    pub words: Vec<String>,
}

impl Config {
    pub fn new() -> Self {
        let matches = Self::make_matches();

        let words: &String = matches.get_one("words").unwrap();
        let words = words.split(',').map(|v| v.to_string()).collect();

        Self { words }
    }

    /// Call only once
    fn make_matches() -> ArgMatches {
        Command::new("Eth crawler")
            .version("1.0")
            .arg(
                Arg::new("words")
                    .long("words")
                    .value_name("WORDS")
                    .help("Comma-separated words")
                    .required(true),
            )
            .get_matches()
    }
}

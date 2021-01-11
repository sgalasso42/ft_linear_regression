use clap::{Arg, App};

#[derive(Debug, PartialEq)]
pub struct Config {
    pub file: String,
}

impl Config {
    pub fn new() -> Self {
        let matches = App::new("predict")
            .version("0.1.0")
            .author("Simon Galasso <simon.galasso@hotmail.fr>")
            .about("Use save from the learn program to predict the price of a car")
            .arg(Arg::with_name("file")
                .required(true)
                .index(1)
                .help("Path to the save file"))
            .get_matches();
        return Config {
            file: matches.value_of("file").unwrap_or("").to_string(),
        };
    }
}
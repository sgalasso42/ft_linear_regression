use clap::{Arg, App};

#[derive(Debug, PartialEq)]
pub enum Algo {
	Sgradient, Gradient, Ols
}

#[derive(Debug, PartialEq)]
pub struct Config {
    pub file: String,
    pub algo: Algo,
}

impl Config {
    pub fn new() -> Self {
        let matches = App::new("learn")
            .version("0.1.0")
            .author("Simon Galasso <simon.galasso@hotmail.fr>")
            .about("Perform a linear regression on a given dataset")
            .arg(Arg::with_name("file")
                .required(true)
                .index(1)
                .help("Path to the data file"))
            .arg(Arg::with_name("algo")
                .required(false)
                .short("a")
                .long("algo")
                .takes_value(true)
                .help("Algo selection, choose from 'ols' or 'gradient' or 'sgradient"))
            .get_matches();
        return Config {
            file: matches.value_of("file").unwrap_or("").to_string(),
            algo: match matches.value_of("algo").unwrap_or("conflict") {
                "gradient" => Algo::Gradient,
                "sgradient" => Algo::Sgradient,
                "ols" => Algo::Ols,
                _ => Algo::Gradient
            }
        };
    }
}
use std::fs::File;
use std::fs::metadata;
use std::io::prelude::*;
use std::io::BufReader;

use crate::parsing::args::*;

pub fn parse_file(config: &Config) -> (f64, f64) {
	if !metadata(&config.file).expect("error: A problem occured with the file").is_file() {
        panic!("error: The file should be a file, I mean a real one, not a directory, hum... guess you got it");
	}
	let file = File::open(&config.file).expect("error: file not found");
	let lines: Vec<_> = BufReader::new(file).lines().collect();
	if lines.len() != 2 {
		panic!("error: Bad file format");
	}
	let content: Vec<&str> = lines[1].as_ref().expect("error: bad file format").split(",").collect();
	let m: f64 = content[0].parse::<f64>().expect("error: bad character");
	let b: f64 = content[1].parse::<f64>().expect("error: bad character");
	return (m, b);
}
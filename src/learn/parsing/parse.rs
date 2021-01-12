use std::fs::File;
use std::fs::metadata;
use std::io::prelude::*;
use std::io::BufReader;

use crate::game::game::*;
use crate::parsing::args::*;
use crate::maths::scale::*;

pub fn parse_file(config: &Config) -> (Vec<Pos>, Pos) {
	if !metadata(&config.file).expect("error: A problem occured with the file").is_file() {
        panic!("error: The file should be a file, I mean a real one, not a directory, hum... guess you got it");
	}
	let file = File::open(&config.file).expect("error: file not found");
	let lines: Vec<_> = BufReader::new(file).lines().collect();
	// securiser parsing
	let mut dataset: Vec<Pos> = Vec::new();
	let mut max_values: Pos = Pos::new(0.0, 0.0);
	for (i, line) in lines.into_iter().enumerate() {
    	if i > 0 {
			if let Ok(content) = line {
				let values: Vec<&str> = content.split(",").collect();
				let km: f64 = values[0].parse::<f64>().expect("error: bad character");
				let price: f64 = values[1].parse::<f64>().expect("error: bad character");
				if km > max_values.x {
					max_values.x = km;
				}
				if price > max_values.y {
					max_values.y = price;
				}
				dataset.push(Pos::new(km, price));
			}
		}
	}
	return (dataset.iter().map(|value| Pos {
		x: scale(value.x, 0.0, max_values.x, 0.0, 1.0),
		y: scale(value.y, 0.0, max_values.y, 0.0, 1.0),
	}).collect(), max_values);
}
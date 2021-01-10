use std::env;
use std::io::{BufReader};
use std::io::prelude::*;
use std::fs::File;

use crate::graphics::render::*;
use crate::maths::scale::*;

pub fn load_file() -> Vec<Pos> {
	let mut max_values: Pos = Pos::new(0.0, 0.0);
	let args: Vec<String> = env::args().collect();
	if args.len() != 2 {
		panic!("error: bad args number")
	}
	let file = File::open(&args[1]).expect("error: file not found");
	let lines: Vec<_> = BufReader::new(file).lines().collect();
	let mut dataset: Vec<Pos> = Vec::new();
	for (i, line) in lines.into_iter().enumerate() {
        if i == 0 {
			continue
		}
        else if let Ok(content) = line {
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
	let scaled_dataset = dataset.iter().map(|value| Pos {
		x: scale(value.x, 0.0, max_values.x, 0.0, 1.0),
		y: scale(value.y, 0.0, max_values.y, 0.0, 1.0),
	}).collect();
	return scaled_dataset;
}
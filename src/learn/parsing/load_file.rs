use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::env;

use crate::graphics::render::*;
use crate::maths::scale::*;

#[derive(Debug)]
pub enum Algo {
	Gradient, Ols
}

fn parse_file(file: File) -> Vec<Pos> {
	let lines: Vec<_> = BufReader::new(file).lines().collect();
	let mut dataset: Vec<Pos> = Vec::new();
	let mut max_values: Pos = Pos::new(0.0, 0.0);
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
	let scaled_dataset: Vec<Pos> = dataset.iter().map(|value| Pos {
		x: scale(value.x, 0.0, max_values.x, 0.0, 1.0),
		y: scale(value.y, 0.0, max_values.y, 0.0, 1.0),
	}).collect();
	return scaled_dataset;
}

pub fn load_file() -> (Vec<Pos>, Algo, String) {
	let args: Vec<String> = env::args().collect();
	let mut algo: Algo = Algo::Gradient;
	if args.len() < 2 {
		panic!("error: bad args number")	
	} else if args.len() == 3 && args[2] == "Ols" {
		algo = Algo::Ols;
	}
	let file = File::open(&args[1]).expect("error: can't open the file");
	return (parse_file(file), algo, args[1].clone());
}
use std::io::{self, Write, BufRead};

mod parsing;

use crate::parsing::parse::*;
use crate::parsing::args::*;

// A securiser
fn main() {
    let config: Config = Config::new();
    let (m, b): (f64, f64)= parse_file(&config);
    println!("Data file: {}", config.file);
    println!("m: {}\nb: {}", m, b);

    print!("Enter the km value: ");
    io::stdout().flush().unwrap();
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();
    let line = iterator.next().unwrap().unwrap();
    let km: u32 = line.parse().unwrap();

    let estimated_price: f64 = b + (m * km as f64);

    println!("Estimated price: {}", estimated_price as u32);
}
extern crate math;
use math::round;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

fn calc_fuel_for_mass(module_mass: f64) -> f64 {
    round::floor(module_mass / 3.0, 0) - 2.0
}

fn main() -> io::Result<()> {
    let f = File::open("./src/input")?;
    let f = BufReader::new(f);
    let mut fuel_needed: f64 = 0.0;
    for line in f.lines() {
        let line_string: String = line.unwrap();
        let module_mass: f64 = line_string.parse().unwrap();
        fuel_needed += calc_fuel_for_mass(module_mass)
    }
    println!("{}", fuel_needed);
    Ok(())
}

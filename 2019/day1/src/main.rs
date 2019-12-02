extern crate math;
use math::round;
use std::fs::File;
use std::io::BufRead;
use std::io::{self, BufReader};

fn create_file_reader(file_path: String) -> io::Result<BufReader<File>> {
    let file = File::open(file_path)?;
    Ok(BufReader::new(file))
}

fn calc_fuel_for_mass(module_mass: f64) -> f64 {
    round::floor(module_mass / 3.0, 0) - 2.0
}

fn part_one(file_path: String) -> io::Result<(f64)> {
    let reader = create_file_reader(file_path).unwrap();
    let mut fuel_needed: f64 = 0.0;

    for line in reader.lines() {
        let module_mass: f64 = line.unwrap().parse().unwrap();
        fuel_needed += calc_fuel_for_mass(module_mass);
    }

    Ok(fuel_needed)
}

fn part_two(module_fuel: f64) -> io::Result<(f64)> {
    let mut extra_fuel_required = calc_fuel_for_mass(module_fuel);
    let mut fuel_required_total = module_fuel;
    loop {
        fuel_required_total += extra_fuel_required;
        extra_fuel_required = calc_fuel_for_mass(extra_fuel_required);
        if extra_fuel_required <= 0.0 {
            break;
        }
    }
    Ok(fuel_required_total)
}

fn main() {
    let file_path: String = String::from("./src/input");
    let first_result = part_one(file_path).unwrap();
    println!("Part one: {}", first_result);

    let second_result = part_two(first_result).unwrap();
    println!("Part two: {}", second_result);
}

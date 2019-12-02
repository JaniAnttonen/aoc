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
    let mut module_mass: f64;
    let mut fuel_needed: f64 = 0.0;

    for line in reader.lines() {
        module_mass = line.unwrap().parse().unwrap();
        fuel_needed += calc_fuel_for_mass(module_mass);
    }

    Ok(fuel_needed)
}

fn part_two(file_path: String) -> io::Result<(f64)> {
    let reader = create_file_reader(file_path).unwrap();
    let mut module_mass: f64;
    let mut fuel_required: f64;
    let mut extra_fuel_required: f64;
    let mut fuel_required_total: f64 = 0.0;

    for line in reader.lines() {
        module_mass = line.unwrap().parse().unwrap();
        fuel_required = calc_fuel_for_mass(module_mass);
        extra_fuel_required = calc_fuel_for_mass(fuel_required);
        loop {
            if extra_fuel_required <= 0.0 {
                break;
            }
            fuel_required += extra_fuel_required;
            extra_fuel_required = calc_fuel_for_mass(extra_fuel_required);
        }
        fuel_required_total += fuel_required;
    }

    Ok(fuel_required_total)
}

fn main() {
    let file_path: String = String::from("./src/input");
    let first_result = part_one(file_path.clone()).unwrap();
    println!("Part one: {}", first_result);

    let second_result = part_two(file_path.clone()).unwrap();
    println!("Part two: {}", second_result);
}

use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Unable to read file");
    let masses: Vec<i32> = contents.lines().map(|s|  s.parse().unwrap()).collect();
    let total_fuel = masses.iter().fold(0, |acc, &i| acc + calculate_fuel(i));
    println!("Total fuel = {}", total_fuel)
}

fn calculate_fuel(mass: i32) -> i32 {
    let fuel_needed = mass_to_fuel(mass);
    if fuel_needed <= 0 {
        0
    } else {
        fuel_needed + calculate_fuel(fuel_needed)
    }
}

fn mass_to_fuel(mass: i32) -> i32 {
    mass / 3 - 2
}

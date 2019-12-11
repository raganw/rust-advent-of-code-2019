use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Unable to read file");
    let lines: Vec<i32> = contents.lines().map(|s|  s.parse().unwrap()).collect();
    let total_fuel = lines.iter().fold(0, |acc, &i| acc + calculate_fuel(i));

    println!("Total fuel = {}", total_fuel)
}

fn calculate_fuel(mass: i32) -> i32 {
    mass / 3 - 2
}

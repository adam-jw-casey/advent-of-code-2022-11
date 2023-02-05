use std::env;
use std::fs;
use advent_of_code_2022_11::monkey_business;

fn main() {
    let args = env::args().collect::<Vec<_>>();
    let file_path = &args[1];
    let contents = fs::read_to_string(file_path).expect("Should have been able to read {file_path}");

    println!("The level of monkey business is {}", monkey_business(&contents));
}

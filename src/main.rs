
use std::fs;


mod year_2015;



fn main() {
    println!("Advent of Code in Rust!");
    advent_of_code_rust::command_line::validate_args(advent_of_code_rust::run);
}


fn _input_source(path: &str) -> Vec<String> {
    fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

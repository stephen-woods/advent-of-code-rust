pub mod command_line;
mod year_2015;

pub fn run(year: u32, day: u8) {
    println!("Advent of Code {} in Rust!", year);
    match (year, day) {
        (2015, 1) => year_2015::day_01::run(),
        (2015, 2) => year_2015::day_02::run(),
        (2015, 3) => year_2015::day_03::run(),
        (2015, 4) => year_2015::day_04::run(),
        (2015, 5) => year_2015::day_05::run(),
        (2015, 6) => year_2015::day_06::run(),
        _ => println!("--- Day {}: Not implemented!", day),
    }
}
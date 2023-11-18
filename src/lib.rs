pub mod algorithm;
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
        (2015, 7) => year_2015::day_07::run(),
        (2015, 8) => year_2015::day_08::run(),
        (2015, 9) => year_2015::day_09::run(),
        (2015, 10) => year_2015::day_10::run(),
        (2015, 11) => year_2015::day_11::run(),
        (2015, 12) => year_2015::day_12::run(),
        (2015, 13) => year_2015::day_13::run(),
        (2015, 14) => year_2015::day_14::run(),
        _ => println!("--- Day {}: Not implemented!", day),
    }
}

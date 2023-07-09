pub mod command_line;
mod year_2015;

pub fn run(year: u32, day: u8) {
    match (year, day) {
        (2015, 1) => {
            println!("Year {} day {}", year, day);
            year_2015::day_01::run()
        },
        _ => println!("Year {} day {} not implemented.", year, day),
    }
}

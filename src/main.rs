use frunk::hlist_pat;
use frunk::prelude::*;
use std::fs;

use std::env;

mod year_2015;

fn main() {
    println!("Advent of Code in Ruest!");

    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("Usage cargo run -- <year> <day>");
        return;
    } else {
        let year = args[1]
            .parse::<u32>()
            .map_err(|_| "Year must be a number");

        let day = args[2]
            .parse::<u8>()
            .map_err(|_| "Day must be a number");

        let vs = (year.into_validated() + day.into_validated())
            .into_result()
            .map(|hlist_pat!(year, day)| (year, day ));

        match vs {
            Err(errs) => {
                for err in errs {
                    println!("{}", err);
                }
            }
            Ok((2015, 1)) => year_2015::day01::run(),
            Ok((year, day )) => {
                println!("Year {} day {} not implemented.", year, day)
            }
        }
    }
}


fn _input_source(path: &str) -> Vec<String> {
    fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

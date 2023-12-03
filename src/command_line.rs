use frunk::hlist_pat;
use frunk::prelude::*;
use std::env;

pub fn validate_args<F>(f: F)
where
    F: FnOnce(u32, u8) -> (),
{
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("Usage cargo run -- <year> <day>");
        return;
    } else {
        let year = args[1].parse::<u32>().map_err(|_| "Year must be a number");

        let day = args[2].parse::<u8>().map_err(|_| "Day must be a number");

        let vs = (year.into_validated() + day)
            .into_result()
            .map(|hlist_pat!(year, day)| (year, day));

        match vs {
            Err(errs) => {
                for err in errs {
                    println!("{}", err);
                }
            }

            Ok((year, day)) => f(year, day),
        }
    }
}

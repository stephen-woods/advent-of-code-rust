use std::borrow::ToOwned;

pub fn run() {
    println!("--- Day 4: The Ideal Stocking Stuffer ---");

    let answer_a = part_a();
    println!("To mine AdventCoins, you must find Santa the lowest positive number (no leading zeroes: 1, 2, 3, ...) that produces such a hash.\n {}", answer_a);

    let answer_b = part_b();
    println!("Now find one that starts with six zeroes.\n {}", answer_b);
}

fn part_a() -> u64 {
    mine("00000")
}

fn part_b() -> u64 {
    mine("000000")
}

fn mine(prefix: &str) -> u64 {
    let mut found = false;
    let mut x: u64 = 0;
    while !found {
        x += 1;
        let key = INPUT_A.to_owned() + format!("{}", x).as_str();
        let digest = md5::compute(key);
        let hash = format!("{:x}", digest);

        found = hash.starts_with(prefix);
    }
    return x
}

const INPUT_A: &str = "yzbqklnj";
use crypto::digest::Digest;
use crypto::md5::Md5;
use std::time::SystemTime;

pub fn run() {
    println!("--- Day 4: The Ideal Stocking Stuffer ---");

    let now = SystemTime::now();
    let answer_a = part_a();
    let duration = now.elapsed().expect("Elapsed failed");
    println!("To mine AdventCoins, you must find Santa the lowest positive number (no leading zeroes: 1, 2, 3, ...) that produces such a hash.");
    println!(" {}", answer_a);
    println!(" in {}ms", duration.as_millis());

    let now = SystemTime::now();
    let answer_b = part_b();
    let duration = now.elapsed().expect("Elapsed failed");
    println!("Now find one that starts with six zeroes.");
    println!(" {}", answer_b);
    println!(" in {}ms", duration.as_millis());
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
    let mut md5 = Md5::new();
    while !found {
        md5.reset();
        x += 1;

        let key = format!("{}{}", INPUT_A, x);
        md5.input_str(key.as_str());
        let result = md5.result_str();
        found = result.starts_with(prefix);
    }
    return x
}

const INPUT_A: &str = "yzbqklnj";
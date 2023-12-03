// --- Day 4: The Ideal Stocking Stuffer ---
// Santa needs help mining some AdventCoins (very similar to bitcoins) to use as gifts for all the
// economically forward-thinking little girls and boys.
//
// To do this, he needs to find MD5 hashes which, in hexadecimal, start with at least five zeroes.
// The input to the MD5 hash is some secret key (your puzzle input, given below) followed by a
// number in decimal. To mine AdventCoins, you must find Santa the lowest positive number (no
// leading zeroes: 1, 2, 3, ...) that produces such a hash.
//
// For example:
//
// - If your secret key is abcdef, the answer is 609043, because the MD5 hash of abcdef609043 starts
//   with five zeroes (000001dbbfa...), and it is the lowest such number to do so.
// - If your secret key is pqrstuv, the lowest number it combines with to make an MD5 hash starting
//   with five zeroes is 1048970; that is, the MD5 hash of pqrstuv1048970 looks like 000006136ef....
//
// Your puzzle answer was 282749.
//
// --- Part Two ---
// Now find one that starts with six zeroes.
//
// Your puzzle answer was 9962624.

use md5::{Digest, Md5};
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
    while !found {
        x += 1;

        let key = format!("{}{}", INPUT_A, x);
        let mut hasher = Md5::new();
        hasher.update(key);
        let result_as_bytes = hasher.finalize();
        let result = format!("{:x}", result_as_bytes);
        found = result.starts_with(prefix);
    }
    return x;
}

const INPUT_A: &str = "yzbqklnj";

#[test]
fn test_a() {
    let result = part_a();
    assert_eq!(result, 282749);
}

#[test]
fn test_b() {
    let result = part_b();
    assert_eq!(result, 9962624);
}

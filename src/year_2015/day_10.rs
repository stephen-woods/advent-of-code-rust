// --- Day 10: Elves Look, Elves Say ---
// Today, the Elves are playing a game called look-and-say. They take turns making sequences by
// reading aloud the previous sequence and using that reading as the next sequence. For example, 211
// is read as "one two, two ones", which becomes 1221 (1 2, 2 1s).
//
// Look-and-say sequences are generated iteratively, using the previous value as input for the next
// step. For each step, take the previous value, and replace each run of digits (like 111) with the
// number of digits (3) followed by the digit itself (1).
//
// For example:
//
// - 1 becomes 11 (1 copy of digit 1).
// - 11 becomes 21 (2 copies of digit 1).
// - 21 becomes 1211 (one 2 followed by one 1).
// - 1211 becomes 111221 (one 1, one 2, and two 1s).
// - 111221 becomes 312211 (three 1s, two 2s, and one 1).
//
// Starting with the digits in your puzzle input, apply this process 40 times. What is the length of
// the result?
//
// Your puzzle answer was 492982.
//
// --- Part Two ---
// Neat, right? You might also enjoy hearing John Conway talking about this sequence (that's Conway
// of Conway's Game of Life fame).
//
// Now, starting again with the digits in your puzzle input, apply this process 50 times. What is
// the length of the new result?
//
// Your puzzle answer was 6989950.

use std::time::SystemTime;

pub fn run() {
    println!("--- Day 10: Elves Look, Elves Say ---");

    let now = SystemTime::now();
    let answer_a = part_a();
    let duration = now.elapsed().expect("Elapsed failed");
    println!("Starting with the digits in your puzzle input, apply this process 40 times. What is the length of the result?");
    println!(" {}", answer_a);
    println!(" in {}ms", duration.as_millis());

    let now = SystemTime::now();
    let answer_b = part_b();
    let duration = now.elapsed().expect("Elapsed failed");
    println!("Now, starting again with the digits in your puzzle input, apply this process 50 times. What is the length of the new result?");
    println!(" {}", answer_b);
    println!(" in {}ms", duration.as_millis());
}

fn part_a() -> usize {
    let mut say = String::from(INPUT_A);
    for _ in 0..40 {
        let look = parse_digits(&say);
        say = look_say(look);
    }

    say.len()
}

fn part_b() -> usize {
    let mut say = String::from(INPUT_A);
    for _ in 0..50 {
        let look = parse_digits(&say);
        say = look_say(look);
    }

    say.len()
}

fn parse_digits(look: &str) -> Vec<u32> {
    let ret: Result<Vec<u32>, String> = look
        .chars()
        .map(|c| {
            c.to_digit(10)
                .ok_or(String::from("Error: Input contains non digits"))
        })
        .collect();
    ret.unwrap()
}

fn look_say(look: Vec<u32>) -> String {
    let size = look.len();
    let mut i = 0;
    let mut ret = String::new();
    while i < size {
        let mut count = 0u32;
        let c = look[i];
        while i < size && look[i] == c {
            count += 1;
            i += 1;
        }
        ret.push(char::from_digit(count, 10).unwrap());
        ret.push(char::from_digit(c, 10).unwrap());
    }
    ret
}

const INPUT_A: &str = "1321131112";

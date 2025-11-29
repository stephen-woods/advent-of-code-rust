//! --- Day 5: How About a Nice Game of Chess? ---
//!
//! You are faced with a security door designed by Easter Bunny engineers that seem to have acquired
//! most of their security knowledge by watching hacking movies.
//!
//! The eight-character password for the door is generated one character at a time by finding the MD5
//! hash of some Door ID (your puzzle input) and an increasing integer index (starting with 0).
//!
//! A hash indicates the next character in the password if its hexadecimal representation starts with
//! five zeroes. If it does, the sixth character in the hash is the next character of the password.
//!
//! For example, if the Door ID is abc:
//!
//! The first index which produces a hash that starts with five zeroes is 3231929, which we find by
//! hashing abc3231929; the sixth character of the hash, and thus the first character of the password,
//! is 1. 5017308 produces the next interesting hash, which starts with 000008f82..., so the second
//! character of the password is 8. The third time a hash starts with five zeroes is for abc5278568,
//! discovering the character f.
//!
//! In this example, after continuing this search a total of eight times, the password is 18f47a30.
//!
//! Given the actual Door ID, what is the password?
//!
//! Your puzzle answer was 1a3099aa.
//!
//!
//! --- Part Two ---
//!
//! As the door slides open, you are presented with a second door that uses a slightly more inspired
//! security mechanism. Clearly unimpressed by the last version (in what movie is the password
//! decrypted in order?!), the Easter Bunny engineers have worked out a better solution.
//!
//! Instead of simply filling in the password from left to right, the hash now also indicates the
//! position within the password to fill. You still look for hashes that begin with five zeroes;
//! however, now, the sixth character represents the position (0-7), and the seventh character is the
//! character to put in that position.
//!
//! A hash result of 000001f means that f is the second character in the password. Use only the first
//! result for each position, and ignore invalid positions.
//!
//! For example, if the Door ID is abc:
//!
//! - The first interesting hash is from abc3231929, which produces 0000015...; so, 5 goes in
//!   position 1: _5______.
//!
//! - In the previous method, 5017308 produced an interesting hash; however, it is ignored, because
//!   it specifies an invalid position (8).
//!
//! - The second interesting hash is at index 5357525, which produces 000004e...; so, e goes in
//!   position 4: _5__e___.
//!
//! You almost choke on your popcorn as the final character falls into place, producing the password
//! 05ace8e3.
//!
//! Given the actual Door ID and this new method, what is the password? Be extra proud of your
//! solution if it uses a cinematic "decrypting" animation.
//!
//! Your puzzle answer was 694190cd.
use md5::{Digest, Md5};
use std::{io::Write, time::SystemTime};

pub fn run() {
    println!("--- How About a Nice Game of Chess? --- ");

    let now = SystemTime::now();
    let answer_a = part_a();
    let duration = now.elapsed().expect("Elapsed failed");
    println!(
        "Given the actual Door ID, what is the password?\n {}\n in {}ms",
        answer_a,
        duration.as_millis()
    );

    print!("Given the actual Door ID and this new method, what is the password?\n ");
    let now = SystemTime::now();
    let _answer_b = part_b();
    let duration = now.elapsed().expect("Elapsed failed");
    println!("\n in {}ms", duration.as_millis());
}

pub fn part_a() -> String {
    let mut length = 0;
    let mut i = 0;
    let mut password = String::new();

    while length < PASSWORD_LENGTH {
        let candidate = format!("{}{}", INPUT, i);
        let hash = Md5::new_with_prefix(candidate.as_bytes()).finalize();

        // Convert the hash into a hex string
        let hex_hash = base16ct::lower::encode_string(&hash);

        // If the hex hash starts with five zeroes, we have a candidate
        if hex_hash.starts_with(CANDIDATE_PREFIX) {
            let ch = hex_hash.chars().nth(5).unwrap();
            password.push(ch);
            length += 1;
        }
        i += 1;
    }

    password
}

pub fn part_b() -> String {
    let mut i = 0;

    // We are going to do the suggested animation of printing the password as we find characters
    // The password starts as all underscores
    let mut password = [MISSING_CHAR; PASSWORD_LENGTH];
    password.iter().for_each(|c| print!("{}", c));

    while password.contains(&'_') {
        let candidate = format!("{}{}", INPUT, i);
        let hash = Md5::new_with_prefix(candidate.as_bytes()).finalize();

        // Convert the hash into a hex string
        let hex_hash = base16ct::lower::encode_string(&hash);

        // If the hex hash starts with five zeroes, we have a candidate
        if hex_hash.starts_with(CANDIDATE_PREFIX) {
            let mut chars = hex_hash.chars();

            // Check to see if the the 6th character is a valid position, and if that position is
            // not already filled
            let pos = chars.nth(5).unwrap() as usize - DIGIT_BASE;
            if (0..8).contains(&pos) && password[pos] == MISSING_CHAR {
                // We found a new character for the password
                let ch = chars.next().unwrap();
                password[pos] = ch;

                // Backspace over the previous password. Make sure to flush stdout so the backspaces
                // are printed before we print the password
                (0..PASSWORD_LENGTH).for_each(|_| print!("{}", BACKSPACE));
                std::io::stdout().flush().unwrap();

                // Print out the password with the newly found character
                password.iter().for_each(|c| print!("{}", c));
                std::io::stdout().flush().unwrap();
            }
        }
        i += 1;
    }
    // Remove the password from the console
    // (0..PASSWORD_LENGTH).for_each(|_| print!("{} {}", BACKSPACE, BACKSPACE));
    // println!();
    password.iter().collect::<String>()
}

const PASSWORD_LENGTH: usize = 8;

const DIGIT_BASE: usize = '0' as usize;

const CANDIDATE_PREFIX: &str = "00000";

const MISSING_CHAR: char = '_';

const BACKSPACE: &str = "\x08";

const INPUT: &str = "uqwqemis";

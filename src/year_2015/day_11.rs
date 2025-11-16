/// --- Day 11: Corporate Policy ---
/// Santa's previous password expired, and he needs help choosing a new one.
///
/// To help him remember his new password after the old one expires, Santa has devised a method of
/// coming up with a password based on the previous one. Corporate policy dictates that passwords
/// must be exactly eight lowercase letters (for security reasons), so he finds his new password by
/// incrementing his old password string repeatedly until it is valid.
///
/// Incrementing is just like counting with numbers: xx, xy, xz, ya, yb, and so on. Increase the
/// rightmost letter one step; if it was z, it wraps around to a, and repeat with the next letter
/// to the left until one doesn't wrap around.
///
/// Unfortunately for Santa, a new Security-Elf recently started, and he has imposed some additional
/// password requirements:
///
/// - Passwords must include one increasing straight of at least three letters, like abc, bcd, cde,
///   and so on, up to xyz. They cannot skip letters; abd doesn't count.
/// - Passwords may not contain the letters i, o, or l, as these letters can be mistaken for other
///   characters and are therefore confusing.
/// - Passwords must contain at least two different, non-overlapping pairs of letters, like aa, bb,
///   or zz.
///
/// For example:
///
/// - hijklmmn meets the first requirement (because it contains the straight hij) but fails the
///   second requirement requirement (because it contains i and l).
/// - abbceffg meets the third requirement (because it repeats bb and ff) but fails the first
///   requirement.
/// - abbcegjk fails the third requirement, because it only has one double letter (bb).
/// - The next password after abcdefgh is abcdffaa.
/// - The next password after ghijklmn is ghjaabcc, because you eventually skip all the passwords
///   that start with ghi..., since i is not allowed.
///
/// Given Santa's current password (your puzzle input), what should his next password be?
///
/// Your puzzle answer was hepxxyzz.
///
/// --- Part Two ---
/// Santa's password expired again. What's the next one?
///
/// Your puzzle answer was heqaabcc.
use std::time::SystemTime;

pub fn run() {
    println!("--- Day 11: Corporate Policy ---");

    let now = SystemTime::now();
    let answer_a = part_a();
    let duration = now.elapsed().expect("Elapsed failed");
    println!("Given Santa's current password, what should his next password be?");
    println!(" {}", answer_a);
    println!(" in {}ms", duration.as_millis());

    let now = SystemTime::now();
    let answer_b = part_b();
    let duration = now.elapsed().expect("Elapsed failed");
    println!("Santa's password expired again. What's the next one??");
    println!(" {}", answer_b);
    println!(" in {}ms", duration.as_millis());
}

fn part_a() -> String {
    next_password(INPUT_A)
}

fn part_b() -> String {
    next_password("hepxxyzz")
}

fn next_password(current: &str) -> String {
    let sorted_invalids = vec![
        ascii_char_to_radix26("i"),
        ascii_char_to_radix26("l"),
        ascii_char_to_radix26("o"),
    ];

    let mut r26 = str_to_u64(current, 26, 97);
    // Bump to next candidate
    r26 += 1;
    let mut vd = u64_to_reverse_digits(r26, 26);
    let mut found = false;
    while !found {
        if check_contains_straight(&vd, 3)
            && check_consecutive_duplicates(&vd, 2)
            && check_contains_valid(&vd, &sorted_invalids)
        {
            found = true
        } else {
            r26 += 1;
            vd = u64_to_reverse_digits(r26, 26);
        }
    }

    reverse_digits_to_string(&vd)
}

fn str_to_u64(s: &str, radix: u64, ascii_base: u8) -> u64 {
    let zero: u64 = 0;
    // Assuming each character is an ASCII character byte, simply subtract all of the
    // characters before the ascii_base. Fold over each number to build up number.
    s.bytes()
        .map(|c| c - ascii_base)
        .fold(zero, |a, x| (a * radix) + u64::from(x))
}

fn ascii_char_to_radix26(s: &str) -> u8 {
    if (s.len() != 1) || (!s.is_ascii()) {
        panic!("String is not a single ASCII char")
    }

    let c = s.as_bytes().first().unwrap();
    c - 97
}

fn u64_to_reverse_digits(n: u64, radix: u64) -> Vec<u8> {
    // Returned Vec will have digits in reverse order.
    let mut ret: Vec<u8> = Vec::new();

    let mut num: u64 = n;
    loop {
        let m: u8 = (num % radix) as u8;
        num /= radix;

        ret.push(m);
        if num == 0 {
            break;
        }
    }
    ret
}

fn reverse_digits_to_string(vd: &[u8]) -> String {
    vd.iter()
        .rev()
        .map(|x| char::from_u32(u32::from(*x) + 97).unwrap())
        .collect()
}

fn check_contains_straight(vd: &[u8], count: i32) -> bool {
    let mut c = 1;
    let mut last: u8 = u8::MAX - 1;
    for x in vd.iter().rev() {
        if *x == last + 1 {
            c += 1;
            if c == count {
                break;
            }
        } else {
            c = 1
        }
        last = *x;
    }
    c == count
}

fn check_consecutive_duplicates(vd: &[u8], num_dupes: i32) -> bool {
    // Keep a count of how many duplicates we have encountered.
    let mut dupes = 0;

    // Set the last element to something invalid
    let mut last: u8 = u8::MAX - 1;
    for x in vd.iter().rev() {
        if *x == last {
            // If the current element matches the last one, we have a duplicate. Reset the last to
            // invalid so we do not count overlapping duplicates.
            dupes += 1;
            last = u8::MAX - 1;
            if dupes == num_dupes {
                break;
            }
        } else {
            last = *x;
        }
    }
    dupes == num_dupes
}

fn check_contains_valid(vd: &[u8], sorted_invalids: &[u8]) -> bool {
    if sorted_invalids.is_empty() {
        return true;
    }

    for x in vd.iter() {
        if sorted_invalids.binary_search(x).is_ok() {
            return false;
        }
    }
    true
}

const INPUT_A: &str = "hepxcrrq";

#[cfg(test)]
mod tests {
    use crate::year_2015::day_11::{part_a, part_b};

    #[test]
    fn test_a() {
        let answer = part_a();
        assert_eq!("hepxxyzz", answer);
    }

    #[test]
    fn test_b() {
        let answer = part_b();
        assert_eq!("heqaabcc", answer);
    }
}

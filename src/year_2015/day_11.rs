// --- Day 11: Corporate Policy ---
// Santa's previous password expired, and he needs help choosing a new one.
//
// To help him remember his new password after the old one expires, Santa has devised a method of
// coming up with a password based on the previous one. Corporate policy dictates that passwords
// must be exactly eight lowercase letters (for security reasons), so he finds his new password by
// incrementing his old password string repeatedly until it is valid.
//
// Incrementing is just like counting with numbers: xx, xy, xz, ya, yb, and so on. Increase the
// rightmost letter one step; if it was z, it wraps around to a, and repeat with the next letter
// to the left until one doesn't wrap around.
//
// Unfortunately for Santa, a new Security-Elf recently started, and he has imposed some additional
// password requirements:
//
// - Passwords must include one increasing straight of at least three letters, like abc, bcd, cde,
//   and so on, up to xyz. They cannot skip letters; abd doesn't count.
// - Passwords may not contain the letters i, o, or l, as these letters can be mistaken for other
//   characters and are therefore confusing.
// - Passwords must contain at least two different, non-overlapping pairs of letters, like aa, bb,
//   or zz.
// For example:
//
// - hijklmmn meets the first requirement (because it contains the straight hij) but fails the
//   second requirement requirement (because it contains i and l).
// - abbceffg meets the third requirement (because it repeats bb and ff) but fails the first
//   requirement.
// - abbcegjk fails the third requirement, because it only has one double letter (bb).
// - The next password after abcdefgh is abcdffaa.
// - The next password after ghijklmn is ghjaabcc, because you eventually skip all the passwords
//   that start with ghi..., since i is not allowed.
//
// Given Santa's current password (your puzzle input), what should his next password be?

use std::{time::SystemTime, collections::VecDeque};

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
    println!("huh?");
    println!(" {}", answer_b);
    println!(" in {}ms", duration.as_millis());
}

fn part_a() -> usize {
    // let xxx : Vec<u32> = INPUT_A.chars().into_iter().collect();
    3
}

fn part_b() -> usize {


    let aaa = str_to_radix26("abcz");

    println!("Ploop {aaa}");

    let bbb = radix26_to_vd(aaa);

    println!("Poop {:?}", bbb);
    3
}

fn str_to_radix26(s: &str) -> u32 {
    // Assuming each character is an ASCII character, simply subtract all of the characters
    // before lower case a. Fold over each number in reverse to build up a radix 26 number.
    s.chars()
        .map(|c| (c as u32) - 96)
        .rev()
        .fold(0, |a, x| (a * 26) + x)
}

fn radix26_to_vd(n: u32) -> VecDeque<char> {
    // Use a VecDeque instead of a Vec to efficiently prepend resulting chars
    let mut ret: VecDeque<char> = VecDeque::new();
    let mut num: u32 = n;

    loop {
        let m = num % 26;
        num = num / 26;

        let ch = char::from_u32(m + 96).unwrap();
        ret.push_front(ch);
        if num == 0 {
            break;
        }
    }
    ret
}

fn vd_to_string(vd: VecDeque<char>) -> String {
    vd.into_iter()
        .collect()
}

const INPUT_A: &str = "hepxcrrq";

#[test]
fn test_a() {
    // let a = str_to_radix26("a");
    // let b = str_to_radix26("b");
    let z = str_to_radix26("z");

    assert_eq!(z, 26);
    let aa = str_to_radix26("aa");
    assert_eq!(aa, 27);

    let mut gg = str_to_radix26("z");
    gg += 15;
    let gggx = radix26_to_vd(gg);
    let ggg = vd_to_string(gggx);
    assert_eq!(ggg, "aa");
    //let _b = INPUT_A;
    //let result = part_a();
    //assert_eq!(result, 492982);
}




// fn str_to_radix26(s: &str) -> u32 {
//     s.chars()
//         .map(|c| (c as u32) - 96)
//         .rev()
//         .fold(0, |a, x| (a * 26) + x)
// }

// fn radix26_to_string(n: u32) -> String {
//     let mut num = n;
//     let mut ret = String::from("");
//     loop {
//         let m = num % 26;
//         num = num / 26;

//         let ch = char::from_u32(m + 96).unwrap();
//         ret.push(ch);
//         if num == 0 {
//             break;
//         }
//     }
//     ret

//     // FIXME Need to reverse the string or prepend instead of append
// }
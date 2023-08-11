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

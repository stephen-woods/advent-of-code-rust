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
    println!("Huh?");
    println!(" {}", answer_b);
    println!(" in {}ms", duration.as_millis());
}


fn part_a() -> u32 {
    match parse_digits(INPUT_A) {
        Ok(r) => println!("{:?}",r),
        Err(e) => println!("{e}")
    }
    3
}


fn part_b() -> u32 {
    3
}

fn parse_digits(look: &str) -> Result<Vec<u32>, String> {
    look
        .chars()
        .map(|c| c
            .to_digit(10)
            .ok_or(String::from("Error: Input contains non digits"))
        )
        .collect()
}




fn look_say(look: Vec<u32>) -> Result<String, String> {
    if look.is_empty() {
        return Err(String::from("Error: Input is empty"));
    }

    let size = look.len();
    let mut i = 0;
    while i < size {
        let mut count = 1;
        let j = i;
        while j < size && look[i] == look[j] {
            count += 1
        }

        i += 1;
    }
    Ok(String::from("Ok"))
}

const INPUT_A: &str = "1321131112";


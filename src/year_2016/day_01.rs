/// --- Day 1: No Time for a Taxicab ---
///
/// Santa's sleigh uses a very high-precision clock to guide its movements, and the clock's
/// oscillator is regulated by stars. Unfortunately, the stars have been stolen... by the Easter
/// Bunny. To save Christmas, Santa needs you to retrieve all fifty stars by December 25th.
///
/// Collect stars by solving puzzles. Two puzzles will be made available on each day in the Advent
/// calendar; the second puzzle is unlocked when you complete the first. Each puzzle grants one
/// star. Good luck!
///
/// You're airdropped near Easter Bunny Headquarters in a city somewhere. "Near", unfortunately, is
/// as close as you can get - the instructions on the Easter Bunny Recruiting Document the Elves
/// intercepted start here, and nobody had time to work them out further.
///
/// The Document indicates that you should start at the given coordinates (where you just landed)
/// and face North. Then, follow the provided sequence: either turn left (L) or right (R) 90
/// degrees, then walk forward the given number of blocks, ending at a new intersection.
///
/// There's no time to follow such ridiculous instructions on foot, though, so you take a moment
/// and work out the destination. Given that you can only walk on the street grid of the city, how
/// far is the shortest path to the destination?
///
/// For example:
///
///  - Following R2, L3 leaves you 2 blocks East and 3 blocks North, or 5 blocks away.
///  - R2, R2, R2 leaves you 2 blocks due South of your starting position, which is 2 blocks away.
///  - R5, L5, R5, R3 leaves you 12 blocks away.
///
/// How many blocks away is Easter Bunny HQ?
///
/// --- Part Two ---
///
/// Then, you notice the instructions continue on the back of the Recruiting Document. Easter Bunny
/// HQ is actually at the first location you visit twice.
///
/// For example, if your instructions are R8, R4, R4, R8, the first location you visit twice is 4
/// blocks away, due East.
///
/// How many blocks away is the first location you visit twice?
use indoc::indoc;
use std::time::SystemTime;

pub fn run() {
    println!("--- No Time for a Taxicab --- ");

    let now = SystemTime::now();
    let answer_a = part_a();
    let duration = now.elapsed().expect("Elapsed failed");
    println!(
        "How many blocks away is Easter Bunny HQ\n {}\n in {}ms",
        answer_a,
        duration.as_millis()
    );

    // let now = SystemTime::now();
    // let answer_b = part_b();
    // let duration = now.elapsed().expect("Elapsed failed");
    // println!(
    //     "Given the ingredients in your kitchen and their properties, what is the total score of the highest-scoring cookie you can make with a calorie total of 500\n {}\n in {}ms",
    //     answer_b,
    //     duration.as_millis()
    // );
}

fn part_a() -> i32 {
    let mut loc = Location::new();

    let commands = parse(_INPUT_SAMPLE);
    for command in commands {
        loc.travel(command.as_str());
    }
    loc.manhattan_distance()
}

/// Location where direction is:
/// 0 - North
/// 1 - East
/// 2 - South
/// 3 - West
struct Location {
    x: i32,
    y: i32,
    d: u8,
}

impl Location {
    fn new() -> Self {
        Location { x: 0, y: 0, d: 0 }
    }

    fn travel(&mut self, s: &str) {
        let mut it = s.chars();
        match it.next() {
            Some('L') => self.d = (self.d + 3) % 4,
            Some('R') => self.d = (self.d + 1) % 4,
            _ => panic!("Invalid direction"),
        }

        let dist = it.collect::<String>().parse::<i32>().unwrap();
        match self.d {
            0 => self.y -= dist,
            1 => self.x += dist,
            2 => self.y += dist,
            3 => self.x -= dist,
            _ => panic!("Invalid direction"),
        }
    }

    fn manhattan_distance(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}

fn parse(input: &str) -> Vec<String> {
    input.split(", ").map(|s| s.to_string()).collect()
}

const _INPUT_SAMPLE: &str = indoc! {r#"
L2, L5, L5, R5, L2, L4, R1, R1, L4, R2, R1, L1, L4, R1, L4, L4, R5, R3, R1, L1, R1, L5, L1, R5, L4, R2, L5, L3, L3, R3, L3, R4, R4, L2, L5, R1, R2, L2, L1, R3, R4, L193, R3, L5, R45, L1, R4, R79, L5, L5, R5, R1, L4, R3, R3, L4, R185, L5, L3, L1, R5, L2, R1, R3, R2, L3, L4, L2, R2, L3, L2, L2, L3, L5, R3, R4, L5, R1, R2, L2, R4, R3, L4, L3, L1, R3, R2, R1, R1, L3, R4, L5, R2, R1, R3, L3, L2, L2, R2, R1, R2, R3, L3, L3, R4, L4, R4, R4, R4, L3, L1, L2, R5, R2, R2, R2, L4, L3, L4, R4, L5, L4, R2, L4, L4, R4, R1, R5, L2, L4, L5, L3, L2, L4, L4, R3, L3, L4, R1, L2, R3, L2, R1, R2, R5, L4, L2, L1, L3, R2, R3, L2, L1, L5, L2, L1, R4"#};

#[cfg(test)]
mod tests {
    #[test]
    fn test_part_a() {
        let result = super::part_a();
        assert_eq!(result, 181);
    }
}

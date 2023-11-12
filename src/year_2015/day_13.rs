// --- Day 13: Knights of the Dinner Table ---
// In years past, the holiday feast with your family hasn't gone so well. Not everyone
// gets along! This year, you resolve, will be different. You're going to find the
// optimal seating arrangement and avoid all those awkward conversations.
//
// You start by writing up a list of everyone invited and the amount their bappiness
// would increase or decrease if they were to find themselves sitting next to each
// other person. You have a circular table that will be just big enough to fit
// everyone comfortably, and so each person will have exactly two neighbors.
//
// For example, suppose you have only four attendees planned, and you calculate their potential happiness as follows:
//
// Alice would gain 54 happiness units by sitting next to Bob.
// Alice would lose 79 happiness units by sitting next to Carol.
// Alice would lose 2 happiness units by sitting next to David.
// Bob would gain 83 happiness units by sitting next to Alice.
// Bob would lose 7 happiness units by sitting next to Carol.
// Bob would lose 63 happiness units by sitting next to David.
// Carol would lose 62 happiness units by sitting next to Alice.
// Carol would gain 60 happiness units by sitting next to Bob.
// Carol would gain 55 happiness units by sitting next to David.
// David would gain 46 happiness units by sitting next to Alice.
// David would lose 7 happiness units by sitting next to Bob.
// David would gain 41 happiness units by sitting next to Carol.
// Then, if you seat Alice next to David, Alice would lose 2 happiness units (because
// David talks so much), but David would gain 46 happiness units (because Alice is
// such a good listener), for a total change of 44.
//
// If you continue around the table, you could then seat Bob next to Alice (Bob gains
// 83, Alice gains 54). Finally, seat Carol, who sits next to Bob (Carol gains 60, Bob
// loses 7) and David (Carol gains 55, David gains 41). The arrangement looks like this:
//
//      +41 +46
// +55   David    -2
// Carol       Alice
// +60    Bob    +54
//      -7  +83
// After trying every other seating arrangement in this hypothetical scenario, you find
// that this one is the most optimal, with a total change in happiness of 330.
//
// What is the total change in happiness for the optimal seating arrangement of the actual guest list?

use std::time::SystemTime;

pub fn run() {
    println!("--- Day 13: Knights of the Dinner Table --- ");

    let now = SystemTime::now();
    let answer_a = part_a();
    let duration = now.elapsed().expect("Elapsed failed");
    println!(
        "What is the total change in happiness for the optimal seating arrangement of the actual guest list?\n {}\n in {}ns",
        answer_a,
        duration.as_nanos()
    );

    let now = SystemTime::now();
    let answer_b = part_b();
    let duration = now.elapsed().expect("Elapsed failed");
    println!("What is the position of the character that causes Santa to first enter the basement?\n {}\n in {}ns", answer_b, duration.as_nanos());
}

fn part_a() -> i32 {
    3
}

fn part_b() -> i32 {
    3
}

// --- Day 13: Knights of the Dinner Table ---
// In years past, the holiday feast with your family hasn't gone so well. Not everyone
// gets along! This year, you resolve, will be different. You're going to find the
// optimal seating arrangement and avoid all those awkward conversations.
//
// You start by writing up a list of everyone invited and the amount their happiness
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
//
// Your puzzle answer was 618.
//
// --- Part Two ---
// In all the commotion, you realize that you forgot to seat yourself. At this point, you're pretty
// apathetic toward the whole thing, and your happiness wouldn't really go up or down regardless of
// who you sit next to. You assume everyone else would be just as ambivalent about sitting next to
// you, too.
//
// So, add yourself to the list, and give all happiness relationships that involve you a score of 0.
//
// What is the total change in happiness for the optimal seating arrangement that actually includes
// yourself?
//
// Your puzzle answer was 601.

use crate::algorithm::heap_permutations;
use indoc::indoc;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::time::SystemTime;

pub fn run() {
    println!("--- Day 13: Knights of the Dinner Table --- ");

    let now = SystemTime::now();
    let answer_a = part_a();
    let duration = now.elapsed().expect("Elapsed failed");
    println!(
        "What is the total change in happiness for the optimal seating arrangement of the actual guest list?\n {}\n in {}ms",
        answer_a,
        duration.as_millis()
    );

    let now = SystemTime::now();
    let answer_b = part_b();
    let duration = now.elapsed().expect("Elapsed failed");
    println!(
        "What is the total change in happiness for the optimal seating arrangement that actually includes yourself?\n {}\n in {}ms",
        answer_b,
        duration.as_millis()
    );
}

fn part_a() -> i32 {
    let (all_knights, relationships) = all_knights_and_relationships(_INPUT_A);

    let seatings = heap_permutations(&all_knights);

    happiest(seatings, relationships)
}

fn part_b() -> i32 {
    let (mut all_knights, relationships) = all_knights_and_relationships(_INPUT_A);
    all_knights.insert(String::from("Me"));

    let seatings = heap_permutations(&all_knights);

    happiest(seatings, relationships)
}

fn all_knights_and_relationships(input: &str) ->  (HashSet<String>, HashMap<(String, String), i32>)  {
    let kr = KnightRegex::init();
    let mut all_knights: HashSet<String> = HashSet::new();

    let mut relationships: HashMap<(String, String), i32> = HashMap::new();
    for line in input.lines() {
        if let Some(r) = kr.parse_happiness(line) {
            all_knights.insert(r.knight_a.clone());
            relationships.insert((r.knight_a.clone(), r.knight_b.clone()), r.happiness);
        }
    }
    (all_knights, relationships)
}

fn happiest(seatings: Vec<Vec<&str>>, relationships:  HashMap<(String, String), i32>) -> i32 {
    let mut answer = i32::MIN;
    for seating in seatings {
        let x = calculate_happiness(&seating, &relationships);
        answer = i32::max(answer, x);
    }
    answer
}

fn calculate_happiness(seating: &Vec<&str>, relationships: &HashMap<(String, String), i32>) -> i32 {
    let mut total = 0;

    let mut seating_pairs: Vec<&[&str]> = seating.windows(2).collect();

    // We need to include the wrap around relationship too
    let last = *seating.last().unwrap();
    let first = *seating.first().unwrap();
    let wrap_around = vec![last, first];
    seating_pairs.push(&wrap_around);

    for ab in seating_pairs {
        // We need to consider happiness in both directions between knights
        let key = &(ab[0].to_string(), ab[1].to_string());
        let h = relationships
            .get(key)
            .unwrap_or(&0);
        total += *h;

        let key = &(ab[1].to_string(), ab[0].to_string());
        let h = relationships
            .get(key)
            .unwrap_or(&0);
        total += *h;
    }

    total
}

struct KnightRelationship {
    knight_a: String,
    knight_b: String,
    happiness: i32,
}

struct KnightRegex {
    happiness: Regex,
}

impl KnightRegex {
    fn init() -> KnightRegex {
        KnightRegex {
            happiness: Regex::new(INPUT_REGEX).unwrap(),
        }
    }

    fn parse_happiness(&self, s: &str) -> Option<KnightRelationship> {
        self.happiness.captures(s).map(|c| {
            let a = c.name("a").unwrap().as_str();
            let b = c.name("b").unwrap().as_str();
            let gl = c.name("gl").unwrap().as_str();
            let n = c.name("n").unwrap().as_str();

            let abs_happiness = n.parse::<i32>().unwrap();
            let happiness = match gl {
                "gain" => abs_happiness,
                _ => -abs_happiness,
            };
            let knight_a = String::from(a);
            let knight_b = String::from(b);
            KnightRelationship {
                knight_a,
                knight_b,
                happiness,
            }
        })
    }
}

const INPUT_REGEX: &str = "^(?<a>[a-zA-Z]+) would (?<gl>gain|lose) (?<n>[0-9]+) happiness units by sitting next to (?<b>[a-zA-Z]+)\\.$";

const _INPUT_SAMPLE: &str = indoc! {r#"
Alice would gain 54 happiness units by sitting next to Bob.
Alice would lose 79 happiness units by sitting next to Carol.
Alice would lose 2 happiness units by sitting next to David.
Bob would gain 83 happiness units by sitting next to Alice.
Bob would lose 7 happiness units by sitting next to Carol.
Bob would lose 63 happiness units by sitting next to David.
Carol would lose 62 happiness units by sitting next to Alice.
Carol would gain 60 happiness units by sitting next to Bob.
Carol would gain 55 happiness units by sitting next to David.
David would gain 46 happiness units by sitting next to Alice.
David would lose 7 happiness units by sitting next to Bob.
David would gain 41 happiness units by sitting next to Carol."#};

const _INPUT_A: &str = indoc! {r#"
Alice would lose 57 happiness units by sitting next to Bob.
Alice would lose 62 happiness units by sitting next to Carol.
Alice would lose 75 happiness units by sitting next to David.
Alice would gain 71 happiness units by sitting next to Eric.
Alice would lose 22 happiness units by sitting next to Frank.
Alice would lose 23 happiness units by sitting next to George.
Alice would lose 76 happiness units by sitting next to Mallory.
Bob would lose 14 happiness units by sitting next to Alice.
Bob would gain 48 happiness units by sitting next to Carol.
Bob would gain 89 happiness units by sitting next to David.
Bob would gain 86 happiness units by sitting next to Eric.
Bob would lose 2 happiness units by sitting next to Frank.
Bob would gain 27 happiness units by sitting next to George.
Bob would gain 19 happiness units by sitting next to Mallory.
Carol would gain 37 happiness units by sitting next to Alice.
Carol would gain 45 happiness units by sitting next to Bob.
Carol would gain 24 happiness units by sitting next to David.
Carol would gain 5 happiness units by sitting next to Eric.
Carol would lose 68 happiness units by sitting next to Frank.
Carol would lose 25 happiness units by sitting next to George.
Carol would gain 30 happiness units by sitting next to Mallory.
David would lose 51 happiness units by sitting next to Alice.
David would gain 34 happiness units by sitting next to Bob.
David would gain 99 happiness units by sitting next to Carol.
David would gain 91 happiness units by sitting next to Eric.
David would lose 38 happiness units by sitting next to Frank.
David would gain 60 happiness units by sitting next to George.
David would lose 63 happiness units by sitting next to Mallory.
Eric would gain 23 happiness units by sitting next to Alice.
Eric would lose 69 happiness units by sitting next to Bob.
Eric would lose 33 happiness units by sitting next to Carol.
Eric would lose 47 happiness units by sitting next to David.
Eric would gain 75 happiness units by sitting next to Frank.
Eric would gain 82 happiness units by sitting next to George.
Eric would gain 13 happiness units by sitting next to Mallory.
Frank would gain 77 happiness units by sitting next to Alice.
Frank would gain 27 happiness units by sitting next to Bob.
Frank would lose 87 happiness units by sitting next to Carol.
Frank would gain 74 happiness units by sitting next to David.
Frank would lose 41 happiness units by sitting next to Eric.
Frank would lose 99 happiness units by sitting next to George.
Frank would gain 26 happiness units by sitting next to Mallory.
George would lose 63 happiness units by sitting next to Alice.
George would lose 51 happiness units by sitting next to Bob.
George would lose 60 happiness units by sitting next to Carol.
George would gain 30 happiness units by sitting next to David.
George would lose 100 happiness units by sitting next to Eric.
George would lose 63 happiness units by sitting next to Frank.
George would gain 57 happiness units by sitting next to Mallory.
Mallory would lose 71 happiness units by sitting next to Alice.
Mallory would lose 28 happiness units by sitting next to Bob.
Mallory would lose 10 happiness units by sitting next to Carol.
Mallory would gain 44 happiness units by sitting next to David.
Mallory would gain 22 happiness units by sitting next to Eric.
Mallory would gain 79 happiness units by sitting next to Frank.
Mallory would lose 16 happiness units by sitting next to George."#};

#[test]
fn test_a() {
    let answer = part_a();
    assert_eq!(618, answer);
}

#[test]
fn test_b() {
    let answer = part_b();
    assert_eq!(601, answer);
}
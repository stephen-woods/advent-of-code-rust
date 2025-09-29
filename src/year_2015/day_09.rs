// --- Day 9: All in a Single Night ---
// Every year, Santa manages to deliver all of his presents in a single night.
//
// This year, however, he has some new locations to visit; his elves have provided him the distances
// between every pair of locations. He can start and end at any two (different) locations he wants,
// but he must visit each location exactly once. What is the shortest distance he can travel to
// achieve this?
//
// For example, given the following distances:
//
//   London to Dublin = 464
//   London to Belfast = 518
//   Dublin to Belfast = 141
//
// The possible routes are therefore:
//
//   Dublin -> London -> Belfast = 982
//   London -> Dublin -> Belfast = 605
//   London -> Belfast -> Dublin = 659
//   Dublin -> Belfast -> London = 659
//   Belfast -> Dublin -> London = 605
//   Belfast -> London -> Dublin = 982
//
// The shortest of these is London -> Dublin -> Belfast = 605, and so the answer is 605 in this
// example.
//
// What is the distance of the shortest route?
//
// Your puzzle answer was 117.
//
// --- Part Two ---
// The next year, just to show off, Santa decides to take the route with the longest distance
// instead.
//
// He can still start and end at any two (different) locations he wants, and he still must visit
// each location exactly once.
//
// For example, given the distances above, the longest route would be 982 via (for example)
// Dublin -> London -> Belfast.
//
// What is the distance of the longest route?
//
// Your puzzle answer was 909.

use crate::algorithm::heap_permutations;
use indoc::indoc;
use regex::Regex;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};
use std::time::SystemTime;

pub fn run() {
    println!("--- Day 9: All in a Single Night ---");

    let now = SystemTime::now();
    let answer_a = part_a();
    let duration = now.elapsed().expect("Elapsed failed");
    println!("What is the distance of the shortest route?");
    println!(" {}", answer_a);
    println!(" in {}ms", duration.as_millis());

    let now = SystemTime::now();
    let answer_b = part_b();
    let duration = now.elapsed().expect("Elapsed failed");
    println!("What is the distance of the longest route?");
    println!(" {}", answer_b);
    println!(" in {}ms", duration.as_millis());
}

fn part_a() -> u32 {
    let dr = DayRegex::init();

    let mut all_points: HashSet<String> = HashSet::new();
    let mut distances: HashMap<(String, String), u32> = HashMap::new();

    for line in INPUT_A.lines() {
        if let Some(d) = Distance::from(line, &dr) {
            all_points.insert(d.point_a.clone());
            all_points.insert(d.point_b.clone());
            distances.insert((d.point_a.clone(), d.point_b.clone()), d.distance);
            distances.insert((d.point_b.clone(), d.point_a.clone()), d.distance);
        }
    }

    // Create all possible route permutations
    let permutations = heap_permutations(&all_points);

    // Find the shortest distance
    let mut answer = u32::MAX;
    for perm in permutations {
        let x = calc_distance(&perm, &distances);
        answer = min(answer, x);
    }

    answer
}

fn part_b() -> u32 {
    let dr = DayRegex::init();

    let mut all_points: HashSet<String> = HashSet::new();
    let mut distances: HashMap<(String, String), u32> = HashMap::new();

    for line in INPUT_A.lines() {
        if let Some(d) = Distance::from(line, &dr) {
            all_points.insert(d.point_a.clone());
            all_points.insert(d.point_b.clone());
            distances.insert((d.point_a.clone(), d.point_b.clone()), d.distance);
            distances.insert((d.point_b.clone(), d.point_a.clone()), d.distance);
        }
    }

    // Create all possible route permutations
    let permutations = heap_permutations(&all_points);

    // Find the longest distance
    let mut answer = 0;
    for perm in permutations {
        let x = calc_distance(&perm, &distances);
        answer = max(answer, x);
    }

    answer
}

fn calc_distance(points: &Vec<&str>, distances: &HashMap<(String, String), u32>) -> u32 {
    let mut total = 0;
    for ab in points.windows(2) {
        let k = &(ab[0].to_string(), ab[1].to_string());
        let d = distances.get(k).unwrap_or(&0);
        total += d;
    }
    total
}

struct Distance {
    point_a: String,
    point_b: String,
    distance: u32,
}

impl Distance {
    fn from(s: &str, dr: &DayRegex) -> Option<Distance> {
        dr.parse_distance(s)
    }
}

struct DayRegex {
    distance: Regex,
}

impl DayRegex {
    fn init() -> DayRegex {
        DayRegex {
            distance: Regex::new(r"^(?<a>[a-zA-Z]+) to (?<b>[a-zA-Z]+) = (?<d>[0-9]+)$").unwrap(),
        }
    }

    fn parse_distance(&self, s: &str) -> Option<Distance> {
        self.distance.captures(s).map(|c| {
            let a = c.name("a").unwrap().as_str();
            let b = c.name("b").unwrap().as_str();
            let d = c.name("d").unwrap().as_str();

            let point_a = String::from(a);
            let point_b = String::from(b);
            let distance = d.parse::<u32>().unwrap();

            Distance {
                point_a,
                point_b,
                distance,
            }
        })
    }
}

const _INPUT_SAMPLE: &str = indoc! {r#"
London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141"#};

const INPUT_A: &str = indoc! {r#"
Faerun to Tristram = 65
Faerun to Tambi = 129
Faerun to Norrath = 144
Faerun to Snowdin = 71
Faerun to Straylight = 137
Faerun to AlphaCentauri = 3
Faerun to Arbre = 149
Tristram to Tambi = 63
Tristram to Norrath = 4
Tristram to Snowdin = 105
Tristram to Straylight = 125
Tristram to AlphaCentauri = 55
Tristram to Arbre = 14
Tambi to Norrath = 68
Tambi to Snowdin = 52
Tambi to Straylight = 65
Tambi to AlphaCentauri = 22
Tambi to Arbre = 143
Norrath to Snowdin = 8
Norrath to Straylight = 23
Norrath to AlphaCentauri = 136
Norrath to Arbre = 115
Snowdin to Straylight = 101
Snowdin to AlphaCentauri = 84
Snowdin to Arbre = 96
Straylight to AlphaCentauri = 107
Straylight to Arbre = 14
AlphaCentauri to Arbre = 46"#};

#[test]
fn test_a() {
    let result = part_a();
    assert_eq!(result, 117);
}

#[test]
fn test_b() {
    let result = part_b();
    assert_eq!(result, 909);
}

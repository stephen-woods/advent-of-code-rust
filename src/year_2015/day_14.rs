// --- Day 14: Reindeer Olympics ---
// This year is the Reindeer Olympics! Reindeer can fly at high speeds, but must rest occasionally
// to recover their energy. Santa would like to know which of his reindeer is fastest, and so he has
// them race.
//
// Reindeer can only either be flying (always at their top speed) or resting (not moving at all),
// and always spend whole seconds in either state.
//
// For example, suppose you have the following Reindeer:
//
// Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
// Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.
// After one second, Comet has gone 14 km, while Dancer has gone 16 km. After ten seconds, Comet has
// gone 140 km, while Dancer has gone 160 km. On the eleventh second, Comet begins resting (staying
// at 140 km), and Dancer continues on for a total distance of 176 km. On the 12th second, both
// reindeer are resting. They continue to rest until the 138th second, when Comet flies for another
// ten seconds. On the 174th second, Dancer flies for another 11 seconds.
//
// In this example, after the 1000th second, both reindeer are resting, and Comet is in the lead at
// 1120 km (poor Dancer has only gotten 1056 km by that point). So, in this situation, Comet would
// win (if the race ended at 1000 seconds).
//
// Given the descriptions of each reindeer (in your puzzle input), after exactly 2503 seconds, what
// distance has the winning reindeer traveled?
//
// Your puzzle answer was 2696.
//
// --- Part Two ---
// Seeing how reindeer move in bursts, Santa decides he's not pleased with the old scoring system.
//
// Instead, at the end of each second, he awards one point to the reindeer currently in the lead.
// (If there are multiple reindeer tied for the lead, they each get one point.) He keeps the
// traditional 2503 second time limit, of course, as doing otherwise would be entirely ridiculous.
//
// Given the example reindeer from above, after the first second, Dancer is in the lead and gets one
// point. He stays in the lead until several seconds into Comet's second burst: after the 140th
// second, Comet pulls into the lead and gets his first point. Of course, since Dancer had been in
// the lead for the 139 seconds before that, he has accumulated 139 points by the 140th second.
//
// After the 1000th second, Dancer has accumulated 689 points, while poor Comet, our old champion,
// only has 312. So, with the new scoring system, Dancer would win (if the race ended at 1000
// seconds).
//
// Again given the descriptions of each reindeer (in your puzzle input), after exactly 2503 seconds,
// how many points does the winning reindeer have?
use indoc::indoc;
use regex::Regex;
use std::time::SystemTime;

pub fn run() {
    println!("--- Day 13: Knights of the Dinner Table --- ");

    let now = SystemTime::now();
    let answer_a = part_a();
    let duration = now.elapsed().expect("Elapsed failed");
    println!(
        "Given the descriptions of each reindeer (in your puzzle input), after exactly 2503 seconds, what distance has the winning reindeer traveled?\n {}\n in {}ns",
        answer_a,
        duration.as_nanos()
    );

    let now = SystemTime::now();
    let answer_b = part_b();
    let duration = now.elapsed().expect("Elapsed failed");
    println!(
        "Again given the descriptions of each reindeer (in your puzzle input), after exactly 2503 seconds, how many points does the winning reindeer have?\n {}\n in {}ns",
        answer_b,
        duration.as_nanos()
    );
}

fn part_a() -> u32 {
    let rr = RaceRegex::init();

    let mut racers: Vec<Reindeer> = Vec::new();
    for line in INPUT_A.lines() {
        if let Some(rd) = rr.parse(line) {
            racers.push(rd);
        }
    }
   start_distance_race(&mut racers, 2503)
}

fn part_b() -> i32 {
    3
}

fn start_distance_race(racers: &mut Vec<Reindeer>, seconds: usize) -> u32 {
    let mut time: usize = 0;
    while time < seconds {
        for r in racers.iter_mut() {
            if r.left == 0 {
                r.running = !r.running;
                r.left = if r.running {
                    r.run_seconds
                } else {
                    r.rest_seconds
                }
            }
            if r.running {
                r.distance += r.speed
            }
            r.left -= 1 ;
        }
        time += 1;
    }

    let mut farthest:u32 = 0;
    for r in racers {
        farthest = u32::max(farthest, r.distance)
    }
    farthest
}

struct Reindeer {
    name: String,
    speed: u32,
    run_seconds: u32,
    rest_seconds: u32,
    running: bool,
    left: u32,
    distance: u32,
}

struct RaceRegex {
    regex: Regex,
}

impl RaceRegex {
    fn init() -> RaceRegex {
        RaceRegex{
            regex: Regex::new(INPUT_REGEX).unwrap(),
        }
    }

    fn parse(&self, s: &str) -> Option<Reindeer> {
        self.regex.captures(s).map(|c| {
            let name = String::from(c.name("name").unwrap().as_str());
            let s_speed = c.name("speed").unwrap().as_str();
            let s_run = c.name("run").unwrap().as_str();
            let s_rest = c.name("rest").unwrap().as_str();

            let speed = s_speed.parse::<u32>().unwrap();
            let run_seconds = s_run.parse::<u32>().unwrap();
            let rest_seconds = s_rest.parse::<u32>().unwrap();
            Reindeer {
                name,
                speed,
                run_seconds,
                rest_seconds,
                running: false,
                left: 0,
                distance: 0,
            }
        })
    }
}

const INPUT_REGEX: &str = "^(?<name>[a-zA-Z]+) can fly (?<speed>[0-9]+) km/s for (?<run>[0-9]+) seconds, but then must rest for (?<rest>[0-9]+) seconds\\.$";
const _INPUT_SAMPLE: &str = indoc! {r#"
Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.
"#};

const INPUT_A: &str = indoc! {r#"
Rudolph can fly 22 km/s for 8 seconds, but then must rest for 165 seconds.
Cupid can fly 8 km/s for 17 seconds, but then must rest for 114 seconds.
Prancer can fly 18 km/s for 6 seconds, but then must rest for 103 seconds.
Donner can fly 25 km/s for 6 seconds, but then must rest for 145 seconds.
Dasher can fly 11 km/s for 12 seconds, but then must rest for 125 seconds.
Comet can fly 21 km/s for 6 seconds, but then must rest for 121 seconds.
Blitzen can fly 18 km/s for 3 seconds, but then must rest for 50 seconds.
Vixen can fly 20 km/s for 4 seconds, but then must rest for 75 seconds.
Dancer can fly 7 km/s for 20 seconds, but then must rest for 119 seconds."#};

#[test]
fn test_a() {
    let answer = part_a();
    assert_eq!(2696, answer);
}

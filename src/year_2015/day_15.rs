// --- Day 15: Science for Hungry People ---
// Today, you set out on the task of perfecting your milk-dunking cookie recipe. All you have to do
// is find the right balance of ingredients.
//
// Your recipe leaves room for exactly 100 teaspoons of ingredients. You make a list of the
// remaining ingredients you could use to finish the recipe (your puzzle input) and their properties
// per teaspoon:
//
// - capacity (how well it helps the cookie absorb milk)
// - durability (how well it keeps the cookie intact when full of milk)
// - flavor (how tasty it makes the cookie)
// - texture (how it improves the feel of the cookie)
// - calories (how many calories it adds to the cookie)
//
// You can only measure ingredients in whole-teaspoon amounts accurately, and you have to be
// accurate so you can reproduce your results in the future. The total score of a cookie can be
// found by adding up each of the properties (negative totals become 0) and then multiplying
// together everything except calories.
//
// For instance, suppose you have these two ingredients:
//
// Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
// Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3
//
// Then, choosing to use 44 teaspoons of butterscotch and 56 teaspoons of cinnamon (because the
// amounts of each ingredient must add up to 100) would result in a cookie with the following
// properties:
//
// A capacity of 44*-1 + 56*2 = 68
// A durability of 44*-2 + 56*3 = 80
// A flavor of 44*6 + 56*-2 = 152
// A texture of 44*3 + 56*-1 = 76

// Multiplying these together (68 * 80 * 152 * 76, ignoring calories for now) results in a total
// score of 62842880, which happens to be the best score possible given these ingredients. If any
// properties had produced a negative total, it would have instead become zero, causing the whole
// score to multiply to zero.
//
// Given the ingredients in your kitchen and their properties, what is the total score of the
// highest-scoring cookie you can make?

use indoc::indoc;
use regex::Regex;
use std::time::SystemTime;

pub fn run() {
    println!("--- Day 15: Science for Hungry People --- ");

    let now = SystemTime::now();
    let answer_a = part_a();
    let duration = now.elapsed().expect("Elapsed failed");
    println!(
        "Given the ingredients in your kitchen and their properties, what is the total score of the highest-scoring cookie you can make?\n {}\n in {}ns",
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

fn part_a() -> i64 {
    let ir = IngredientRegex::init();

    let mut ingredients: Vec<Ingredient> = Vec::new();
    for line in _INPUT_SAMPLE.lines() {
        if let Some(rd) = ir.parse(line) {
            ingredients.push(rd);
        }
    }

    let q = vec![44, 56];
    score(&ingredients, &q)
}




fn score(ingredients: &Vec<Ingredient>, quantities: &Vec<u8>) -> i64 {
    let mut cap = 0;
    let mut dur = 0;
    let mut fla = 0;
    let mut tex = 0;

    for i in 0..ingredients.len() {
        let ingredient = &ingredients[i];
        let q = quantities[i] as i64;
        cap += ingredient.capacity * q;
        dur +=ingredient.durability * q;
        fla += ingredient.flavor * q;
        tex += ingredient.texture * q;
    }

    i64::max(0, cap) *
        i64::max(0,dur) *
        i64::max(0,fla) *
        i64::max(0,tex)
}

fn part_b() -> u32 {
    3
}

struct Ingredient {
    _name: String,
    capacity: i64,
    durability: i64,
    flavor: i64,
    texture: i64,
    calories: i64,
}

struct IngredientRegex {
    regex: Regex
}

impl IngredientRegex {
    fn init() -> IngredientRegex {
        IngredientRegex {
            regex : Regex::new(INPUT_REGEX).unwrap(),
        }
    }

    fn parse(&self, s: &str) -> Option<Ingredient> {
        self.regex.captures(s).map(|c| {
            // Name is not actually used
            let _name = String::from(c.name("name").unwrap().as_str());
            let capacity = c.name("cap").unwrap().as_str().parse::<i64>().unwrap();
            let durability = c.name("dur").unwrap().as_str().parse::<i64>().unwrap();
            let flavor = c.name("fla").unwrap().as_str().parse::<i64>().unwrap();
            let texture = c.name("tex").unwrap().as_str().parse::<i64>().unwrap();
            let calories = c.name("cal").unwrap().as_str().parse::<i64>().unwrap();

            Ingredient {
                _name,
                capacity,
                durability,
                flavor,
                texture,
                calories,
            }
        })
    }
}


const INPUT_REGEX: &str = "^(?<name>[a-zA-Z]+): capacity (?<cap>[-]?[0-9]+), durability (?<dur>[-]?[0-9]+), flavor (?<fla>[-]?[0-9]+), texture (?<tex>[-]?[0-9]+), calories (?<cal>[-]?[0-9]+)$";
const _INPUT_SAMPLE: &str = indoc! {r#"
Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3
"#};

const INPUT_A: &str = indoc! {r#"
Sprinkles: capacity 2, durability 0, flavor -2, texture 0, calories 3
Butterscotch: capacity 0, durability 5, flavor -3, texture 0, calories 3
Chocolate: capacity 0, durability 0, flavor 5, texture -1, calories 8
Candy: capacity 0, durability -1, flavor 0, texture 5, calories 8"#};

#[test]
fn test_a() {
    let _answer = part_a();
  //  assert_eq!(2696, answer);
}

#[test]
fn test_b() {
    let _answer = part_b();
  //  assert_eq!(1084, answer);
}
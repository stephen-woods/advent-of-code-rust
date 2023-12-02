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
//
// Your puzzle answer was 21367368.
//
// --- Part Two ---
// Your cookie recipe becomes wildly popular! Someone asks if you can make another recipe that has
// exactly 500 calories per cookie (so they can use it as a meal replacement). Keep the rest of your
// award-winning process the same (100 teaspoons, same ingredients, same scoring system).
//
// For example, given the ingredients above, if you had instead selected 40 teaspoons of
// butterscotch and 60 teaspoons of cinnamon (which still adds to 100), the total calorie count would
// be 40*8 + 60*3 = 500. The total score would go down, though: only 57600000, the best you can do
// in such trying circumstances.
//
// Given the ingredients in your kitchen and their properties, what is the total score of the
// highest-scoring cookie you can make with a calorie total of 500?
//
// Your puzzle answer was 1766400.

use indoc::indoc;
use regex::Regex;
use std::time::SystemTime;

pub fn run() {
    println!("--- Day 15: Science for Hungry People --- ");

    let now = SystemTime::now();
    let answer_a = part_a();
    let duration = now.elapsed().expect("Elapsed failed");
    println!(
        "Given the ingredients in your kitchen and their properties, what is the total score of the highest-scoring cookie you can make?\n {}\n in {}ms",
        answer_a,
        duration.as_millis()
    );

    let now = SystemTime::now();
    let answer_b = part_b();
    let duration = now.elapsed().expect("Elapsed failed");
    println!(
        "Given the ingredients in your kitchen and their properties, what is the total score of the highest-scoring cookie you can make with a calorie total of 500\n {}\n in {}ms",
        answer_b,
        duration.as_millis()
    );
}

fn part_a() -> i64 {
    let ir = IngredientRegex::init();

    let mut ingredients: Vec<Ingredient> = Vec::new();
    for line in INPUT_A.lines() {
        if let Some(rd) = ir.parse(line) {
            ingredients.push(rd);
        }
    }

    let mut r = Recipe::init(ingredients, 100, 0);
    let mut best_score: i64 = 0;

    while !r.done {
        r.advance_part_a();
        let s = r.score();
        best_score = i64::max(best_score, s);
    }
    best_score
}

fn part_b() -> i64 {
    let ir = IngredientRegex::init();

    let mut ingredients: Vec<Ingredient> = Vec::new();
    for line in INPUT_A.lines() {
        if let Some(rd) = ir.parse(line) {
            ingredients.push(rd);
        }
    }

    let mut r = Recipe::init(ingredients, 100, 500);
    let mut best_score: i64 = 0;

    while !r.done {
        r.advance_part_b();
        let s = r.score();
        best_score = i64::max(best_score, s);
    }
    best_score
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
    regex: Regex,
}

impl IngredientRegex {
    fn init() -> IngredientRegex {
        IngredientRegex {
            regex: Regex::new(INPUT_REGEX).unwrap(),
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

struct Recipe {
    quantities: Vec<i64>,
    ingredients: Vec<Ingredient>,
    requested_teaspoons: i64,
    requested_calories: i64,
    done: bool,
}

impl Recipe {
    fn init(
        ingredients: Vec<Ingredient>,
        requested_teaspoons: i64,
        requested_calories: i64,
    ) -> Recipe {
        Recipe {
            quantities: vec![0; ingredients.len()],
            ingredients,
            requested_teaspoons,
            requested_calories,
            done: false,
        }
    }

    fn advance_part_a(&mut self) -> () {
        loop {
            if self.done {
                break;
            }
            self.tick();

            // Stop advancing if we have a full recipe
            let sum: i64 = self.quantities.iter().sum();
            if sum == self.requested_teaspoons {
                break;
            }
        }
    }
    
    fn advance_part_b(&mut self) -> () {
        loop {
            if self.done {
                break;
            }
            self.tick();

            // Stop advancing if we have a full recipe AND it has a total of 500 calories
            let sum: i64 = self.quantities.iter().sum();
            if sum == self.requested_teaspoons {
                let cal = self.calculate_calories();
                if cal == self.requested_calories {
                    break;
                }
            }
        }
    }

    fn tick(&mut self) -> () {
        let mut i: usize = self.quantities.len() - 1;

        loop {
            if self.quantities[i] == self.requested_teaspoons {
                if i == 0 {
                    self.done = true;
                    break;
                }
                self.quantities[i] = 0;
                i -= 1;
            } else {
                self.quantities[i] += 1;
                break;
            }
        }
    }

    fn calculate_calories(&self) -> i64 {
        let mut ret = 0i64;
        for i in 0..self.quantities.len() {
            let ic = self.ingredients[i].calories;
            let iq = self.quantities[i];
            ret += ic * iq;
        }
        ret
    }

    fn score(&self) -> i64 {
        let mut cap = 0;
        let mut dur = 0;
        let mut fla = 0;
        let mut tex = 0;

        for i in 0..self.ingredients.len() {
            let ingredient = &self.ingredients[i];
            let q = self.quantities[i];
            cap += ingredient.capacity * q;
            dur += ingredient.durability * q;
            fla += ingredient.flavor * q;
            tex += ingredient.texture * q;
        }

        i64::max(0, cap) * i64::max(0, dur) * i64::max(0, fla) * i64::max(0, tex)
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
    let answer = part_a();
    assert_eq!(21367368, answer);
}

#[test]
fn test_b() {
    let answer = part_b();
    assert_eq!(1766400, answer);
}

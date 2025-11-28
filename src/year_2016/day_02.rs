//! --- Day 2: Bathroom Security ---
//!
//! You arrive at Easter Bunny Headquarters under cover of darkness. However, you left in such a
//! rush that you forgot to use the bathroom! Fancy office buildings like this one usually have
//! keypad locks on their bathrooms, so you search the front desk for the code.
//!
//! "In order to improve security," the document you find says, "bathroom codes will no longer be
//! written down. Instead, please memorize and follow the procedure below to access the bathrooms."
//!
//! The document goes on to explain that each button to be pressed can be found by starting on the
//! previous button and moving to adjacent buttons on the keypad: U moves up, D moves down, L moves
//! left, and R moves right. Each line of instructions corresponds to one button, starting at the
//! previous button (or, for the first line, the "5" button); press whatever button you're on at
//! the end of each line. If a move doesn't lead to a button, ignore it.
//!
//! You can't hold it much longer, so you decide to figure out the code as you walk to the
//! bathroom. You picture a keypad like this:
//!
//! ```ignore
//! 1 2 3
//! 4 5 6
//! 7 8 9
//! ```
//!
//! Suppose your instructions are:
//!
//! ULL
//! RRDDD
//! LURDL
//! UUUUD
//!
//!  -  You start at "5" and move up (to "2"), left (to "1"), and left (you can't, and stay on
//!     "1"), so the first button is 1.
//!
//!  -  Starting from the previous button ("1"), you move right twice (to "3") and then down three
//!     times (stopping at "9" after two moves and ignoring the third), ending up with 9.
//!
//!  -  Continuing from "9", you move left, up, right, down, and left, ending with 8.
//!
//!  -  Finally, you move up four times (stopping at "2"), then down once, ending with 5.
//!
//! So, in this example, the bathroom code is 1985.
//!
//! Your puzzle input is the instructions from the document you found at the front desk. What is
//! the bathroom code?
//!
//! Your puzzle answer was 38961. 
//!
//! --- Part Two ---
//! 
//! You finally arrive at the bathroom (it's a several minute walk from the lobby so visitors can
//! behold the many fancy conference rooms and water coolers on this floor) and go to punch in the
//! code. Much to your bladder's dismay, the keypad is not at all like you imagined it. Instead,
//! you are confronted with the result of hundreds of man-hours of bathroom-keypad-design meetings:
//!
//! ```ignore
//!     1
//!   2 3 4
//! 5 6 7 8 9
//!   A B C
//!     D
//! ```
//!
//! You still start at "5" and stop when you're at an edge, but given the same instructions as
//! above, the outcome is very different:
//!
//!  -  You start at "5" and don't move at all (up and left are both edges), ending at 5.
//!
//!  -  Continuing from "5", you move right twice and down three times (through "6", "7", "B", "D",
//!     "D"), ending at D.
//!
//!  -  Then, from "D", you move five more times (through "D", "B", "C", "C", "B"), ending at B.
//!
//!  -    Finally, after five more moves, you end at 3.
//! 
//! So, given the actual keypad layout, the code would be 5DB3.
//! 
//! Using the same instructions in your puzzle input, what is the correct bathroom code?
//!
//! Your puzzle answer was 46C92
use indoc::indoc;
use std::time::SystemTime;

pub fn run() {
    println!("--- Bathroom Security --- ");

    let now = SystemTime::now();
    let answer_a = part_a();
    let duration = now.elapsed().expect("Elapsed failed");
    println!(
        "What is the bathroom code?\n {}\n in {}ms",
        answer_a,
        duration.as_millis()
    );

    let now = SystemTime::now();
    let answer_b = part_b();
    let duration = now.elapsed().expect("Elapsed failed");
    println!(
        "Using the same instructions in your puzzle input, what is the correct bathroom code?\n {}\n in {}ms",
        answer_b,
        duration.as_millis()
    );
}

pub fn part_a() -> String {
    let mut position = 5;

    let mut solution = String::new();
    for line in _INPUT.lines() {
        for direction in line.chars() {
            position = next(direction, position);
        }
        solution.push_str(&position.to_string());
    }
    solution
}


pub fn part_b() -> String {
    let mut position = 5;

    let mut solution = String::new();
    for line in _INPUT.lines() {
        for direction in line.chars() {
            position = next_hex(direction, position);
        }
        let button = match position {
            10 => String::from("A"),
            11 => String::from("B"),
            12 => String::from("C"),
            13 => String::from("D"),
            x => x.to_string(),
        };
        solution.push_str(&button);
    }
    solution
}


fn next(direction: char, position: i32) -> i32 {
    match direction {
        'L' => left(position),
        'R' => right(position),
        'U' => up(position),
        'D' => down(position),
        _ => panic!("Unknown direction"),
    }
}

fn next_hex(direction: char, position: i32) -> i32 {
    match direction {
        'L' => left_hex(position),
        'R' => right_hex(position),
        'U' => up_hex(position),
        'D' => down_hex(position),
        _ => panic!("Unknown direction"),
    }
}

fn left(p: i32) -> i32 {
    if p % 3 == 1 {
        p
    } else {
        p - 1
    }
}

fn right(p: i32) -> i32 {
    if p % 3 == 0 {
        p
    } else {
        p + 1
    }
}

fn up(p: i32) -> i32 {
    let x = p - 3;
    if (1..=9).contains(&x) {
        x
    } else {
        p
    }
}

fn down(p: i32) -> i32 {
    let x = p + 3;
    if (1..=9).contains(&x) {
        x
    } else {
        p
    }
}

fn left_hex(p: i32) -> i32 {
    match p {
        1|2|5|10|13 => p,
        x => x - 1,
    }
}

fn right_hex(p: i32) -> i32 {
    match p {
        1|4|9|12|13 => p,
        x => x + 1,
    }
}

fn up_hex(p: i32) -> i32 {
    match p {
        1|2|4|5|9 => p,
        3|13 => p - 2,
        x => x - 4 ,
    }
}

fn down_hex(p: i32) -> i32 {
    match p {
        5|9|10|12|13 => p,
        1|11 => p + 2,
        x => x + 4 ,
    }
}
const _INPUT_SAMPLE: &str = indoc! {r#"
ULL
RRDDD
LURDL
UUUUD"#};

const _INPUT: &str = indoc! {r#"
LLLRLLULLDDLDUDRDDURLDDRDLRDDRUULRULLLDLUURUUUDLUUDLRUDLDUDURRLDRRRUULUURLUDRURULRLRLRRUULRUUUDRRDDRLLLDDLLUDDDLLRLLULULRRURRRLDRLDLLRURDULLDULRUURLRUDRURLRRDLLDDURLDDLUDLRLUURDRDRDDUURDDLDDDRUDULDLRDRDDURDLUDDDRUDLUDLULULRUURLRUUUDDRLDULLLUDLULDUUDLDLRRLLLRLDUDRUULDLDRDLRRDLDLULUUDRRUDDDRDLRLDLRDUDRULDRDURRUULLUDURURUUDRDRLRRDRRDRDDDDLLRURULDURDLUDLUULDDLLLDULUUUULDUDRDURLURDLDDLDDUULRLUUDLDRUDRURURRDDLURURDRLRLUUUURLLRR
UUUUURRRURLLRRDRLLDUUUUDDDRLRRDRUULDUURURDRLLRRRDRLLUDURUDLDURURRLUDLLLDRDUDRDRLDRUDUDDUULLUULLDUDUDDRDUUUDLULUDUULLUUULURRUDUULDUDDRDURRLDDURLRDLULDDRUDUDRDULLRLRLLUUDDURLUUDLRUUDDLLRUURDUDLLDRURLDURDLRDUUDLRLLRLRURRUDRRLRDRURRRUULLUDLDURDLDDDUUDRUUUDULLLRDRRDRLURDDRUUUDRRUUDLUDDDRRRRRLRLDLLDDLRDURRURLLLULURULLULRLLDDLDRLDULLDLDDDRLUDDDUDUDRRLRDLLDULULRLRURDLUDDLRUDRLUURRURDURDRRDRULUDURRLULUURDRLDLRUDLUDRURLUDUUULRRLRRRULRRRLRLRLULULDRUUDLRLLRLLLURUUDLUDLRURUDRRLDLLULUDRUDRLLLRLLDLLDUDRRURRLDLUUUURDDDUURLLRRDRUUURRRDRUDLLULDLLDLUDRRDLLDDLDURLLDLLDLLLDR
LRDULUUUDLRUUUDURUUULLURDRURDRRDDDLRLRUULDLRRUDDLLUURLDRLLRUULLUDLUDUDRDRDLUUDULLLLRDDUDRRRURLRDDLRLDRLULLLRUUULURDDLLLLRURUUDDDLDUDDDDLLLURLUUUURLRUDRRLLLUUULRDUURDLRDDDUDLLRDULURURUULUDLLRRURDLUULUUDULLUDUUDURLRULRLLDLUULLRRUDDULRULDURRLRRLULLLRRDLLDDLDUDDDUDLRUURUDUUUDDLRRDLRUDRLLRDRDLURRLUDUULDRRUDRRUDLLLLRURRRRRUULULLLRDRDUDRDDURDLDDUURRURLDRRUDLRLLRRURULUUDDDLLLRDRLULLDLDDULDLUUDRURULLDLLLLDRLRRLURLRULRDLLULUDRDR
RURRRUDLURRURLURDDRULLDRDRDRRULRRDLDDLDUUURUULLRRDRLDRRDRULLURRRULLLDULDDDDLULRUULRURUDURDUDRLRULLLRDURDDUDDRDLURRURUURDLDDDDDURURRURLLLDDLDRRDUDDLLLDRRLDDUUULDLLDRUURUDDRRLDUULRRDDUDRUULRLDLRLRUURLLDRDLDRLURULDLULDRULURLLRRLLDDDURLRUURUULULRLLLULUDULUUULDRURUDDDUUDDRDUDUDRDLLLRDULRLDLRRDRRLRDLDDULULRLRUUDDUDRRLUDRDUUUDRLLLRRLRUDRRLRUUDDLDURLDRRRUDRRDUDDLRDDLULLDLURLUUDLUDLUDLDRRLRRRULDRLRDUURLUULRDURUDUUDDURDDLRRRLUUUDURULRURLDRURULDDUDDLUDLDLURDDRRDDUDUUURLDLRDDLDULDULDDDLDRDDLUURDULLUDRRRULRLDDLRDRLRURLULLLDULLUUDURLDDULRRDDUULDRLDLULRRDULUDUUURUURDDDRULRLRDLRRURR
UDDDRLDRDULDRLRDUDDLDLLDDLUUURDDDLUDRDUDLDURLUURUDUULUUULDUURLULLRLUDLLURUUUULRLRLLLRRLULLDRUULURRLLUDUDURULLLRRRRLRUULLRDRDRRDDLUDRRUULUDRUULRDLRDRRLRRDRRRLULRULUURRRULLRRRURUDUURRLLDDDUDDULUULRURUDUDUDRLDLUULUDDLLLLDRLLRLDULLLRLLDLUUDURDLLRURUUDDDDLLUDDRLUUDUDRDRLLURURLURRDLDDDULUURURURRLUUDUDLDLDDULLURUDLRLDLRLDLDUDULURDUDRLURRRULLDDDRDRURDDLDLULUDRUULDLULRDUUURLULDRRULLUDLDRLRDDUDURRRURRLRDUULURUUDLULDLRUUULUDRDRRUDUDULLDDRLRDLURDLRLUURDRUDRDRUDLULRUDDRDLLLRLURRURRLDDDUDDLRDRRRULLUUDULURDLDRDDDLDURRLRRDLLDDLULULRRDUDUUDUULRDRRDURDDDDUUDDLUDDUULDRDDULLUUUURRRUUURRULDRRDURRLULLDU
"#};

#[cfg(test)]
mod test {

    #[test]
    pub fn test_left() {
        assert_eq!(super::left(1), 1);
        assert_eq!(super::left(2), 1);
        assert_eq!(super::left(3), 2);
        assert_eq!(super::left(4), 4);
        assert_eq!(super::left(5), 4);
        assert_eq!(super::left(6), 5);
        assert_eq!(super::left(7), 7);
        assert_eq!(super::left(8), 7);
        assert_eq!(super::left(9), 8);
    }

    #[test]
    pub fn test_right() {
        assert_eq!(super::right(1), 2);
        assert_eq!(super::right(2), 3);
        assert_eq!(super::right(3), 3);
        assert_eq!(super::right(4), 5);
        assert_eq!(super::right(5), 6);
        assert_eq!(super::right(6), 6);
        assert_eq!(super::right(7), 8);
        assert_eq!(super::right(8), 9);
        assert_eq!(super::right(9), 9);
    }

    #[test]
    pub fn test_up() {
        assert_eq!(super::up(1), 1);
        assert_eq!(super::up(2), 2);
        assert_eq!(super::up(3), 3);
        assert_eq!(super::up(4), 1);
        assert_eq!(super::up(5), 2);
        assert_eq!(super::up(6), 3);
        assert_eq!(super::up(7), 4);
        assert_eq!(super::up(8), 5);
        assert_eq!(super::up(9), 6);
    }

    #[test]
    pub fn test_down() {
        assert_eq!(super::down(1), 4);
        assert_eq!(super::down(2), 5);
        assert_eq!(super::down(3), 6);
        assert_eq!(super::down(4), 7);
        assert_eq!(super::down(5), 8);
        assert_eq!(super::down(6), 9);
        assert_eq!(super::down(7), 7);
        assert_eq!(super::down(8), 8);
        assert_eq!(super::down(9), 9);
    }

    #[test]
    pub fn test_left_hex() {
        assert_eq!(super::left_hex(1), 1);
        assert_eq!(super::left_hex(2), 2);
        assert_eq!(super::left_hex(3), 2);
        assert_eq!(super::left_hex(4), 3);
        assert_eq!(super::left_hex(5), 5);
        assert_eq!(super::left_hex(6), 5);
        assert_eq!(super::left_hex(7), 6);
        assert_eq!(super::left_hex(8), 7);
        assert_eq!(super::left_hex(9), 8);
        assert_eq!(super::left_hex(10), 10); // A
        assert_eq!(super::left_hex(11), 10); // B
        assert_eq!(super::left_hex(12), 11); // C
        assert_eq!(super::left_hex(13), 13); // D
    }

    #[test]
    pub fn test_right_hex() {
        assert_eq!(super::right_hex(1), 1);
        assert_eq!(super::right_hex(2), 3);
        assert_eq!(super::right_hex(3), 4);
        assert_eq!(super::right_hex(4), 4);
        assert_eq!(super::right_hex(5), 6);
        assert_eq!(super::right_hex(6), 7);
        assert_eq!(super::right_hex(7), 8);
        assert_eq!(super::right_hex(8), 9);
        assert_eq!(super::right_hex(9), 9);
        assert_eq!(super::right_hex(10), 11); // A
        assert_eq!(super::right_hex(11), 12); // B
        assert_eq!(super::right_hex(12), 12); // C
        assert_eq!(super::right_hex(13), 13); // D
    }
    
    #[test]
    pub fn test_up_hex() {
        assert_eq!(super::up_hex(1), 1);
        assert_eq!(super::up_hex(2), 2);
        assert_eq!(super::up_hex(3), 1);
        assert_eq!(super::up_hex(4), 4);
        assert_eq!(super::up_hex(5), 5);
        assert_eq!(super::up_hex(6), 2);
        assert_eq!(super::up_hex(7), 3);
        assert_eq!(super::up_hex(8), 4);
        assert_eq!(super::up_hex(9), 9);
        assert_eq!(super::up_hex(10), 6); // A
        assert_eq!(super::up_hex(11), 7); // B
        assert_eq!(super::up_hex(12), 8); // C
        assert_eq!(super::up_hex(13), 11); // D
    }

    #[test]
    pub fn test_down_hex() {
        assert_eq!(super::down_hex(1), 3);
        assert_eq!(super::down_hex(2), 6);
        assert_eq!(super::down_hex(3), 7);
        assert_eq!(super::down_hex(4), 8);
        assert_eq!(super::down_hex(5), 5);
        assert_eq!(super::down_hex(6), 10);
        assert_eq!(super::down_hex(7), 11);
        assert_eq!(super::down_hex(8), 12);
        assert_eq!(super::down_hex(9), 9);
        assert_eq!(super::down_hex(10), 10); // A
        assert_eq!(super::down_hex(11), 13); // B
        assert_eq!(super::down_hex(12), 12); // C
        assert_eq!(super::down_hex(13), 13); // D
    }


    #[test]
    pub fn test_part_a() {
        let result = super::part_a();
        assert_eq!(&result, "38961");
    }

    #[test]
    pub fn test_part_b() {
        let result = super::part_b();
        assert_eq!(&result, "46C92");
    }
}

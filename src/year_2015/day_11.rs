use std::time::SystemTime;

pub fn run() {
    println!("--- Day 11: Corporate Policy ---");

    let now = SystemTime::now();
    let answer_a = part_a();
    let duration = now.elapsed().expect("Elapsed failed");
    println!("Given Santa's current password, what should his next password be?");
    println!(" {}", answer_a);
    println!(" in {}ms", duration.as_millis());

    let now = SystemTime::now();
    let answer_b = part_b();
    let duration = now.elapsed().expect("Elapsed failed");
    println!("huh?");
    println!(" {}", answer_b);
    println!(" in {}ms", duration.as_millis());
}

fn part_a() -> usize {
   // let xxx : Vec<u32> = INPUT_A.chars().into_iter().collect();
    3
}

fn part_b() -> usize {
     // "abcde".chars()
     //    .for_each(|c| {
     //        let x = (c as u32) - 49;
     //        println!("{:?}", x)
     //      //  char::from_u32(x).unwrap().to_digit(26)
     //    });
    //  let xx : String  = "abcde"
    //      .chars()
    //      .map(|c| {
    //          let x = (c as u32) - 49;
    //          char::from_u32(x).unwrap()
    //      }
    //      ).collect();
    //
    //     println!("{xx}");
    //
    // let bb : u64 = u64::from_str_radix(&xx, 26).unwrap();
    //
    // println!("{bb}");
    //
    // let  cc = format!("{}", std::fmt::radix(bb, 26));
    //
    // println!("{cc}");

    let aaa = str_to_radix26("abcz");

    println!("Ploop {aaa}");

    let bbb = radix26_to_string(aaa);

    println!("Poop {bbb}");
    3
}

fn str_to_radix26(s: &str) -> u32 {
    let xx: String = s.chars()
        .map(|c| {
            let x = (c as u32) - 49;
            char::from_u32(x).unwrap()
        })
        .collect();
    u32::from_str_radix(&xx, 26).unwrap()
}

fn radix26_to_string(n: u32) -> String {
    let mut num = n;
    let mut ret = vec![];
    loop {
        let m = num % 26;
        num = num / 26;
        ret.push(char::from_digit(m, 26).unwrap());
        if num == 0 {
            break;
        }
    }
    ret.into_iter()
        .map(|c| {
            let x = (c as u32) + 48;
            char:: from_u32(x).unwrap()
        })
        .rev()
        .collect()
}


fn req_1(s: &str) -> bool {
    todo!()
}


const INPUT_A: &str = "hepxcrrq";
use regex::Regex;
use std::fs::read_to_string;

#[allow(dead_code)]
pub fn solve() {
    println!("day three:");

    let re = Regex::new(r"(?:mul\()(\d*)(?:,)(\d*)(?:\))").unwrap();

    let doc = read_to_string("../inputs/three").expect("where file");

    let mut good_instructions: Vec<String> = vec![];

    splitter(&mut good_instructions, &doc, "don't()".to_string());

    let ungarbaged: i32 = re
        .captures_iter(format!("{:?}", good_instructions).as_str())
        .map(|caps| {
            let (_, [x, y]) = caps.extract();
            x.parse::<i32>().expect("where number") * y.parse::<i32>().expect("where number")
        })
        .sum::<i32>();

    print!("{} \n", ungarbaged);
}

#[allow(dead_code)]
fn splitter(keep: &mut Vec<String>, s: &String, delimeter: String) {
    let split_string = s.split_once(&delimeter);

    match split_string {
        Some(i) => {
            let new_delimeter;
            if delimeter == "don't()" {
                keep.push(i.0.to_string());
                new_delimeter = "do()";
            } else {
                new_delimeter = "don't()";
            }
            splitter(keep, &i.1.to_string(), new_delimeter.to_string());
        }
        _ => {
            if delimeter == "don't()" {
                keep.push(s.to_string());
            }
            return;
        }
    }
}

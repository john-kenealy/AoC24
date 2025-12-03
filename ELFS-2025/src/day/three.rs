#![allow(dead_code)]
use std::fs::File;
use std::io::{BufRead, BufReader};

#[allow(dead_code)]
pub fn solve() {
    let data = import("inputs/three");

    use std::time::Instant;
    let now = Instant::now();

    let mut answer = 0;

    for bank in data {
        let joltage = bank
            .iter()
            .enumerate()
            .fold((0, 0), |digits, (i, &battery)| {
                if i < bank.len() - 1 {
                    if battery > digits.0 {
                        return (battery, 0);
                    } else if battery > digits.1 {
                        return (digits.0, battery);
                    }
                } else {
                    if battery > digits.1 {
                        return (digits.0, battery);
                    }
                }
                digits
            });
        answer += (joltage.0.to_string() + &joltage.1.to_string())
            .parse::<usize>()
            .unwrap()
    }

    println!("Answer: {:?}", answer);

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

#[allow(dead_code)]
fn import(path: &str) -> Vec<Vec<u32>> {
    let doc = File::open(path).expect("where file");
    let reader = BufReader::new(doc);

    reader
        .lines()
        .map(|l| {
            l.expect("where string")
                .chars()
                .map(|n| n.to_digit(10).unwrap())
                .collect()
        })
        .collect()
}

//fn get_digit(bank: &String, index: usize) -> u32 {
//   bank.chars().nth(index).unwrap().to_digit(10).unwrap()
//}
//
//
//////////////////////////////////////////////////////////////////////////////////////////////

#[allow(dead_code)]
pub fn solve_two() {
    let data = import("inputs/three");

    use std::time::Instant;
    let now = Instant::now();

    let mut answer = 0;

    for bank in data {
        let joltage: String = joltage_builder(bank);
        answer += joltage.parse::<usize>().unwrap();
    }

    println!("Answer: {:?}", answer);

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

fn joltage_builder(bank: Vec<u32>) -> String {
    let mut joltage = String::new();
    let mut start = 0;

    for i in (0..12).rev() {
        //println!("start {start}");
        let sub_bank = &bank[start..bank.len() - i];
        let digit = sub_bank.iter().max().unwrap();

        start = sub_bank
            .iter()
            .position(|battery| battery == digit)
            .unwrap()
            + 1
            + start;

        joltage.push_str(&digit.to_string());
        //println!("{digit}");
    }
    //println!("{joltage}");
    joltage
}

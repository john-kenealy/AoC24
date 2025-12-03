#![allow(dead_code)]
use core::str;
use std::{collections::HashSet, fs::read_to_string, usize};

fn import(path: &str) -> Vec<Vec<usize>> {
    let mut doc = read_to_string(path).expect("where file");

    doc.pop();
    doc.split(',')
        .map(|p| {
            p.split('-')
                .map(|num| num.parse::<usize>().unwrap())
                .collect()
        })
        .collect()
}

pub fn solve() {
    let data = import("inputs/2");

    use std::time::Instant;
    let now = Instant::now();

    let mut answer: usize = 0;

    for pair in data {
        let start = pair[0];
        let end = pair[1];
        for number in start..=end {
            let string_number = number.to_string();
            if string_number.len() % 2 != 0 {
                continue;
            }
            let mid = string_number.len() / 2;
            let split_string = string_number.split_at(mid);
            if split_string.0 == split_string.1 {
                answer += number;
            }
        }
    }

    println!("Answer: {:?}", answer);

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

///////////////////////////////////////////////////////////////////////////////

pub fn solve_two() {
    let data = import("inputs/2");

    use std::time::Instant;
    let now = Instant::now();

    let mut answer: usize = 0;

    for pair in data {
        let start = pair[0];
        let end = pair[1];
        for number in start..=end {
            let string_number = number.to_string();
            if factorer(&string_number) {
                answer += number;
            }
        }
    }

    println!("Answer: {:?}", answer);

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

fn factorer(id: &String) -> bool {
    for i in (1..id.len()).rev() {
        if id.len() % i != 0 {
            continue;
        }

        if silly_checker(id, i) {
            return true;
        }
    }

    false
}

fn silly_checker(id: &String, factor: usize) -> bool {
    let mut silly: HashSet<String> = HashSet::new();

    for i in (0..id.len()).step_by(factor) {
        silly.insert(id[i..i + factor].to_string());
    }

    silly.len() == 1
}

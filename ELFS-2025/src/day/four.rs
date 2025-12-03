#![allow(dead_code)]
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solve() {
    let data = import("inputs/3");

    use std::time::Instant;
    let now = Instant::now();

    let mut answer = 0;

    println!("Answer: {:?}", answer);

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

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

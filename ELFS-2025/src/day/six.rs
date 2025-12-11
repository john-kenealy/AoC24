#![allow(dead_code)]
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::usize;

pub fn solve() {
    let data = import("inputs/6-fixed");

    use std::time::Instant;
    let now = Instant::now();

    let mut answer = 0;

    for i in 0..data[0].len() {
        let mut acc = data[0][i].parse::<usize>().unwrap();

        println!("################################################");
        println!("start: {acc}");

        for j in 1..data.len() - 1 {
            let operator = data[data.len() - 1][i].as_str();
            println!("operator: {operator}");
            println!("addend {:?}", data[j][i]);

            acc = match operator {
                "+" => acc + data[j][i].parse::<usize>().unwrap(),
                "*" => acc * data[j][i].parse::<usize>().unwrap(),
                _ => panic!("ruh roh")
            };

            println!("{acc}");
        }
        answer += acc;
    }

    println!("Answer: {:?}", answer);

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

fn import(path: &str) -> Vec<Vec<String>> {
    let doc = File::open(path).expect("where file");
    let reader = BufReader::new(doc);

    reader
        .lines()
        .map(|l| {
            l.expect("where string")
                .split(' ')
                .map(|s| s.to_string())
                .collect()
        })
        .collect()
}

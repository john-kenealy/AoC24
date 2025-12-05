#![allow(dead_code)]
use std::cmp::max;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::usize;

pub fn solve() {
    let ranges = import("inputs/5-ranges");
    let ids = import("inputs/5-ids");

    use std::time::Instant;
    let now = Instant::now();

    let mut answer = 0;

    for i in 0..ids.len() {
        for j in 0..ranges.len() {
            if ids[i][0] >= ranges[j][0] && ids[i][0] <= ranges[j][1] {
                answer += 1;
                break;
            }
        }
    }

    println!("Answer: {:?}", answer);

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

fn import(path: &str) -> Vec<Vec<usize>> {
    let doc = File::open(path).expect("where file");
    let reader = BufReader::new(doc);

    reader
        .lines()
        .map(|l| {
            l.expect("where string")
                .split('-')
                .map(|s| s.parse::<usize>().unwrap())
                .collect()
        })
        .collect()
}

pub fn solve_two() {
    let mut ranges = import("inputs/5-ranges");
    use std::time::Instant;
    let now = Instant::now();

    ranges.sort_by_key(|r| r[0]);

    let mut i = 1;
    while i < ranges.len() {
        let current = &ranges[i];
        let previous = &ranges[i - 1];

        if current[0] <= previous[1] {
            ranges[i - 1][1] = max(ranges[i - 1][1], ranges[i][1]);
            ranges.remove(i);
            i -= 1;
        }
        i += 1;
    }

    let answer: usize = ranges.iter_mut().fold(0, |acc, e| {
        acc + (e[1] - e[0] + 1)
    });

    println!("Answer: {:?}", answer);

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

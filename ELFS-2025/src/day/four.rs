#![allow(dead_code)]
use std::char;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solve() {
    let flat_grid = import("inputs/4");

    use std::time::Instant;
    let now = Instant::now();

    let mut answer = 0;

    for i in 0..flat_grid.len() {
        for j in 0..flat_grid[0].len() {
            let mut window: String = String::new();

            if flat_grid[i][j] == '@' {
                for h in 0..=2 {
                    //println!("h:{h}");
                    if (i == 0 && h == 0) || i + h > flat_grid.len() {
                        continue;
                    }

                    for w in 0..=2 {
                        //println!("w:{w}");
                        if (j == 0 && w == 0) || j + w > flat_grid.len() {
                            continue;
                        }
                        //println!("added: i:{:?} j:{:?}", i + h - 1, j + w - 1);
                        window.push(flat_grid[i + h - 1][j + w - 1]);
                    }
                }

                if window.matches('@').collect::<Vec<&str>>().len() < 5 {
                    //println!("i:{i}, j:{j}, {:?}", flat_grid[i]);
                    answer += 1;
                    //println!("{:?}", window);
                }
            }
        }
    }

    println!("Answer: {:?}", answer);

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

fn import(path: &str) -> Vec<Vec<char>> {
    let doc = File::open(path).expect("where file");
    let reader = BufReader::new(doc);

    reader
        .lines()
        .map(|l| l.expect("where string").chars().collect::<Vec<char>>())
        .collect()
}

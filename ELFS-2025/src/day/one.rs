use std::fs::File;
use std::io::{BufRead, BufReader};
use std::isize;

#[allow(dead_code)]
pub fn solve() {
    let data = import("inputs/one");

    use std::time::Instant;
    let now = Instant::now();

    let mut zero_counter = 0;
    let mut current_position: isize = 50;

    for line in data {
        let turn = line.split_at(1);
        let direction = turn.0;
        let magnitude: isize = turn.1.parse::<isize>().unwrap() % 100;

        match direction {
            "L" => {
                current_position = if current_position < magnitude {
                    100 - (current_position - magnitude).abs() % 100
                } else {
                    current_position - magnitude
                };
                if current_position == 0 {
                    zero_counter += 1;
                }
            }
            "R" => {
                current_position = if current_position + magnitude > 99 {
                    (current_position + magnitude) % 100
                } else {
                    current_position + magnitude
                };
                if current_position == 0 {
                    zero_counter += 1;
                }
            }
            _ => println!("ruh roh"),
        }
    }

    println!("Answer: {:?}", zero_counter);

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

fn import(path: &str) -> Vec<String> {
    let doc = File::open(path).expect("where file");
    let reader = BufReader::new(doc);

    reader.lines().map(|l| l.expect("where string")).collect()
}

//////////////////////////////////////////////////////////////////////////

pub fn solve_two() {
    let data = import("inputs/one");

    use std::time::Instant;
    let now = Instant::now();

    let mut zero_counter = 0;
    let mut current_position: isize = 50;

    for line in data {
        let turn = line.split_at(1);
        let direction = turn.0;
        let mut magnitude: isize = turn.1.parse::<isize>().unwrap();
        let dial_size = 100;

        zero_counter += magnitude / dial_size;
        magnitude = magnitude % dial_size;

        match direction {
            "L" => {
                let diff = (current_position - magnitude).abs();

                if current_position < magnitude {
                    //account for current position being zero
                    zero_counter += if current_position == 0 { 0 } else { 1 };

                    //new current position
                    current_position = dial_size - diff % dial_size
                } else {
                    if diff == 0 {
                        zero_counter += 1;
                    }
                    current_position = diff
                };
            }
            "R" => {
                let sum = current_position + magnitude;

                if sum >= dial_size {
                    zero_counter += 1;
                } else {
                    if sum == 0 {
                        zero_counter += 1;
                    }
                };
                current_position = sum % dial_size
            }
            _ => println!("ruh roh"),
        }
        //println!("{:?}", current_position)
    }

    println!("Answer: {:?}", zero_counter);

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

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
        let magnitude: isize = turn.1.parse::<isize>().unwrap();

        match direction {
            "L" => {
                let diff = (current_position - magnitude).abs();
                current_position = if current_position < magnitude {
                    //account for current position being zero
                    zero_counter += if current_position == 0 { 0 } else { 1 };

                    //count up for number of times 0 'ticked'
                    zero_counter += diff / 100;

                    //new current position
                    if diff % 100 == 0 { 0 } else { 100 - diff % 100 }
                } else {
                    if diff == 0 {
                        zero_counter += 1;
                    }
                    diff
                };
            }
            "R" => {
                let sum = current_position + magnitude;
                current_position = if sum > 99 {
                    zero_counter += sum / 100;
                    sum % 100
                } else {
                    if sum == 0 {
                        zero_counter += 1;
                    }
                    sum
                };
            }
            _ => println!("ruh roh"),
        }
    }

    println!("Answer: {:?}", zero_counter);

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

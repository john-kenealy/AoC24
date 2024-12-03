use std::{fs::File, io::{BufRead, BufReader}};

#[allow(dead_code)]
pub fn solve() {
    println!("Oneeeee is the loneliest number:");

    let doc = File::open("../inputs/one").expect("where file");
    let reader = BufReader::new(doc);
    
    let mut left: Vec<i32> = vec![];
    let mut right: Vec<i32> = vec![];

    let _: Vec<_> = reader.lines()
        .map(|l| l.expect("where line"))
        .map(|l| {
            let v: Vec<&str> = l.split("   ").collect();

            //println!("ahhh {}", v[0]);
            left.push(v[0].parse::<i32>().expect("where number"));

            //println!("ahhh {}", v[1]);
            right.push(v[1].parse::<i32>().expect("where number"));
        }).collect();

    left.sort();
    right.sort();

    let diff: Vec<i32> = left.iter()
        .zip(right)
        .map(|(l, r)| (l-r).abs())
        .collect();

    println!("{}", diff.iter().sum::<i32>());
    //println!("{:?}", right);
}

#[allow(dead_code)]
pub fn solve_two() {
    println!("day 1:");

    let doc = File::open("../inputs/one").expect("where file");
    let reader = BufReader::new(doc);
    
    let mut left: Vec<usize> = vec![];
    let mut right: Vec<usize> = vec![];

    let _: Vec<_> = reader.lines()
        .map(|l| l.expect("where line"))
        .map(|l| {
            let v: Vec<&str> = l.split("   ").collect();

            //println!("ahhh {}", v[0]);
            left.push(v[0].parse::<usize>().expect("where number"));

            //println!("ahhh {}", v[1]);
            right.push(v[1].parse::<usize>().expect("where number"));
        }).collect();


    let mult: Vec<usize> = left.iter()
        .map(|l| {
            right.iter().filter(|n| *n==l).count() * l

        })
        .collect();

    println!("{}", mult.iter().sum::<usize>());
    //println!("{:?}", right);
}

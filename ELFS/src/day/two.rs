use std::{
    fs::File, io::{BufRead, BufReader}, isize
};

pub fn solve() {
    println!("day two baby");

    let doc = File::open("../inputs/two").expect("where file");
    let reader = BufReader::new(doc);

    let reports: Vec<Vec<isize>> = reader
        .lines()
        .map(|l| {
            l.expect("where report")
                .split(" ")
                .map(|s| s.parse::<isize>().unwrap())
                .collect()
        })
        .collect();

    let mut count = 0;

    reports.iter().for_each(|report| {
            if report_checker(report, 0) {count += 1};
        });

    println!("{}", count);
}

fn report_checker(report: &Vec<isize>, inception_counter: usize) -> bool {

    let mut sign = 0;
    let mut i = 0;
    let mut should_increment = false;

    while i < report.len() - 1 {
        
        if inception_counter > 1 {break}

        let diff: isize = report[i] - report[i+1];

        let mut alt1: Vec<isize> = report.to_vec();
        let mut alt2: Vec<isize> = report.to_vec();

        alt1.remove(i);
        alt2.remove(i+1);

        // make sure diff is no greater than 3 either direction
        if diff.abs() > 3 {
            should_increment = report_checker(&alt1, inception_counter + 1) || report_checker(&alt2, inception_counter + 1);
            break;
        }

        // get new sign to track if the diff stays - or +
        let new_sign = if diff > 0 { 1 } else { 0 };

        // first loop just set sign
        if i == 0 {
            sign = new_sign;     
        } 

        // second and beyond check sign change and return if change
        // otherwise increment
        if sign != new_sign || diff == 0 {
            should_increment = report_checker(&alt1, inception_counter + 1) || report_checker(&alt2, inception_counter + 1);
            break;
        } else {
            should_increment = true;
        }

        i += 1;
    }
    //println!("{:?}, {}, {}", report, report.len(), should_increment);

    should_increment
}

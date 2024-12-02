use std::{
    fs::File,
    io::{BufRead, BufReader},
    isize,
};

pub fn solve() {
    println!("day two:");

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
        if report_checker(report, 0) {
            count += 1
        };
    });

    println!("{}", count);
}

fn report_checker(report: &Vec<isize>, inception_counter: usize) -> bool {
    let mut sign = 0;
    let mut i = 0;
    let mut should_increment = false;

    while i < report.len() - 1 {
        let diff: isize = report[i] - report[i + 1];

        // get new sign to track if the diff stays - or +
        let new_sign = if diff > 0 { 1 } else { 0 };

        // first loop just set sign
        if i == 0 {
            sign = new_sign;
        }

        // second and beyond check for rule breakage
        // otherwise increment
        if sign != new_sign || diff == 0 || diff.abs() > 3 {
            should_increment = false;

            // at earliest sign of failure, loop through all iterations of the report removing one
            // element at a time.
            if inception_counter < 1 {
                report.iter().enumerate().for_each(|(j, _)| {
                    let mut alt: Vec<isize> = report.to_vec();
                    alt.remove(j);
                    if report_checker(&alt, inception_counter + 1) {
                        should_increment = true
                    }
                });
            }
            break;
        } else {
            should_increment = true;
        }

        i += 1;
    }

    should_increment
}

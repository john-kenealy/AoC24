// X|Y means page X must be printed before page Y if BOTH are to be printed

use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    usize,
};

pub fn solve() {
    println!("day five:");

    let doc = File::open("../inputs/five").expect("where file");
    let reader = BufReader::new(doc);

    let mut rules: Vec<Vec<usize>> = vec![];
    let mut jobs: Vec<Vec<usize>> = vec![];

    let _ = reader
        .lines()
        .map(|l| {
            splitter(l.expect("where line"), &mut rules, &mut jobs);
        })
        .collect::<Vec<_>>();

    let combined_rules = rule_combiner(&rules);

    let good_jobs_middle = job_checker(&combined_rules, &mut jobs);
    //println!("count of jobs: {:?}\n", jobs.len());
    println!("sum of middles: {:?}", good_jobs_middle.iter().sum::<usize>());

    println!("middle nums: {:?}", good_jobs_middle);
}

// check each page in each job to confirm it follows rules
fn job_checker(
    combined_rules: &HashMap<usize, Vec<usize>>,
    jobs: &mut Vec<Vec<usize>>,
) -> Vec<usize> {
    let mut good_prints = vec![];

    for i in 0..jobs.len() {
        let mut job = jobs[i].clone();
        let mut valid_job = true;
        
        let mut j = 0;

        while j < job.len() {
            let page = job[j];
            if combined_rules.contains_key(&page) {
                if !job_re_orderer(&mut job, j, combined_rules.get(&page).unwrap()) {
                    valid_job = false;
                    j -= 1;
                }
            }
            j += 1;
        }
        if !valid_job {
            println!("job: {:?}", job);

            good_prints.push(get_middle(job.to_vec()));
        }
    }

    good_prints
}

fn get_middle(job: Vec<usize>) -> usize {
    *job.get((job.len() - 1) / 2).expect("where middle")
}

fn job_re_orderer(job: &mut Vec<usize>, page_index: usize, rule: &Vec<usize>) -> bool {
    //let page_index = job.iter().position(|p| p == page);

    let mut valid_job = true;

    for i in (0..page_index).rev() {
        if rule.contains(&job[i]) {
            valid_job = false;
            //println!("job: {:?}", job);
            let removed = job.remove(i);
            //println!("removed: {}", removed);
            //println!("index: {:?}", page_index);
            job.insert(page_index, removed);
            //println!("new job: {:?}", job);
            //break;
        }
    }

    valid_job
}

#[allow(dead_code)]
fn job_validator(job: &Vec<usize>, page_index: usize, rule: &Vec<usize>) -> bool {
    //let page_index = job.iter().position(|p| p == page);

    let mut valid_job = true;

    for i in (0..page_index).rev() {
        if rule.contains(&job[i]) {
            valid_job = false;
            break;
        }
    }

    valid_job
}

// split line into two vectors based on whether the line is a rule or a print job
fn splitter(line: String, rules: &mut Vec<Vec<usize>>, jobs: &mut Vec<Vec<usize>>) {
    if line.contains("|") {
        let rule: Vec<usize> = line
            .split("|")
            .map(|e| e.parse::<usize>().expect("where number"))
            .collect();
        rules.push(rule);
    } else if line.contains(",") {
        let job: Vec<usize> = line
            .split(",")
            .map(|e| e.parse::<usize>().expect("where number"))
            .collect();
        jobs.push(job);
    }
}

// make a combined set of rules for easier traversal?? hopefully??
fn rule_combiner(rules: &Vec<Vec<usize>>) -> HashMap<usize, Vec<usize>> {
    let mut combined_rules: HashMap<usize, Vec<usize>> = HashMap::new();

    rules.iter().for_each(|rule| {
        if combined_rules.contains_key(&rule[0]) {
            combined_rules
                .entry(rule[0])
                .and_modify(|r| r.push(rule[1]));
        } else {
            combined_rules.insert(rule[0], vec![rule[1]]);
        }
    });

    combined_rules
}

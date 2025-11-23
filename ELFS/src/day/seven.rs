use core::str;
use std::{
    collections::{HashMap, HashSet}, fs::File, io::{BufRead, BufReader}, isize
};

pub fn solve() {
    let _equations = import();
    println!("done");

    build_operator_strings(1);
}

fn import() -> Vec<Vec<isize>> {
    let doc = File::open("inputs/seven-test").expect("where file");
    let reader = BufReader::new(doc);
    let equations: Vec<Vec<isize>> = reader
        .lines()
        .map(|l| {
            l.expect("where equation")
                .replace(":", "")
                .split(" ")
                .map(|s| s.parse::<isize>().unwrap())
                .collect()
        })
        .collect();
    return equations;
}

fn build_operator_strings(_size: isize) {
    //let mut operator_strings = vec![];
    let operators = "+*";
    let mut operator_strings: HashSet<String> = HashSet::new();

    add_char("".to_string(), 3, operators, &mut operator_strings);

    println!("{:?}", operator_strings);
}

fn add_char(operator_string: String, length: usize, operators: &str,  operator_strings: &mut HashSet<String>) {
    if operator_string.len() < length {
        for c in operators.chars() {
            let mut new_operator_string = operator_string.clone();
            new_operator_string.push(c);
            add_char(new_operator_string, length, operators, operator_strings); 
        }
    }
    else {
        operator_strings.insert(operator_string);
    }
}


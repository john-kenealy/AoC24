use core::str;
use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
    usize,
};

pub fn solve() {
    let equations = import();

    use std::time::Instant;
    let now = Instant::now();

    let mut operator_string_map: HashMap<usize, HashSet<String>> = HashMap::new();
    let mut valid_solutions: Vec<usize> = Vec::new();

    for e in equations {
        let length = e.len() - 2;
        if !operator_string_map.contains_key(&length) {
            build_operator_strings_map(&mut operator_string_map, length);
        }

        if check_for_valid_operator_string(&e, &operator_string_map.get(&length).unwrap()) {
            valid_solutions.push(e[0]);
        }
    }

    println!("{:?}", valid_solutions.iter().sum::<usize>());

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

fn import() -> Vec<Vec<usize>> {
    let doc = File::open("inputs/seven").expect("where file");
    let reader = BufReader::new(doc);
    let equations: Vec<Vec<usize>> = reader
        .lines()
        .map(|l| {
            l.expect("where equation")
                .replace(":", "")
                .split(" ")
                .map(|s| s.parse::<usize>().unwrap())
                .collect()
        })
        .collect();
    return equations;
}

fn build_operator_strings_map(
    operator_string_map: &mut HashMap<usize, HashSet<String>>,
    length: usize,
) {
    let mut operator_strings: HashSet<String> = HashSet::new();
    build_operator_strings_given_length("".to_string(), length, "+*|", &mut operator_strings);
    operator_string_map.insert(length, operator_strings);
}

fn build_operator_strings_given_length(
    operator_string: String,
    length: usize,
    operators: &str,
    operator_strings: &mut HashSet<String>,
) {
    if operator_string.len() < length {
        for c in operators.chars() {
            let mut new_operator_string = operator_string.clone();
            new_operator_string.push(c);
            build_operator_strings_given_length(
                new_operator_string,
                length,
                operators,
                operator_strings,
            );
        }
    } else {
        operator_strings.insert(operator_string);
    }
}

fn check_for_valid_operator_string(
    equation: &Vec<usize>,
    operator_strings: &HashSet<String>,
) -> bool {
    for string in operator_strings {
        let mut potential_solution = equation[1];
        string.chars().enumerate().for_each(|(i, c)| match c {
            '+' => potential_solution = potential_solution + equation[i + 2],
            '*' => potential_solution = potential_solution * equation[i + 2],
            '|' => { 
                    let mut temp_string = potential_solution.to_string();
                    temp_string.push_str(&equation[i+2].to_string());
                    potential_solution = temp_string.parse::<usize>().unwrap();
                },
            _ => println!("ruh roh..."),
        });
        if potential_solution == equation[0] {
            return true;
        }
    }

    return false;
}

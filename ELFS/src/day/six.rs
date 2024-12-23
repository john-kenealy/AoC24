use std::{
    char,
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
    isize, usize,
};

#[allow(dead_code)]
struct Patrol {
    seen_locations: HashSet<(usize, usize)>,
    visit_count: usize,
}

struct PatrolTwo {
    seen_locations: HashSet<((usize, usize), usize)>,
}

pub fn solve() {
    println!("day six:");

    let _patrol = Patrol {
        seen_locations: HashSet::new(),
        visit_count: 0,
    };

    let mut data = import();

    use std::time::Instant;
    let now = Instant::now();

    //problem_one(&data, &mut patrol);

    let count = problem_two(&mut data);

    let elapsed = now.elapsed();
    //println!("Part 1 Elapsed: {:.2?}", elapsed);
    println!("Part 2 Elapsed: {:.2?}", elapsed);

    //println!("Count of locations visited: {:?}", patrol.visit_count)
    println!("Count of locations to obstruct: {:?}", count)
}

fn import() -> Vec<Vec<char>> {
    let doc = File::open("../inputs/six").expect("where file");
    let reader = BufReader::new(doc);

    reader
        .lines()
        .map(|l| l.expect("where char").chars().collect::<Vec<_>>())
        .collect()
}

#[allow(dead_code)]
fn problem_one(data: &Vec<Vec<char>>, patrol: &mut Patrol) {
    let mut start: (usize, usize) = (42, 42);
    data.iter()
        .enumerate()
        .for_each(|(i, row)| match row.iter().position(|e| *e == '^') {
            Some(j) => start = (i, j),
            _ => (),
        });

    println!("start {:?}", start);
    traverse(data, patrol, start, "up");
}

#[allow(dead_code)]
fn traverse(data: &Vec<Vec<char>>, patrol: &mut Patrol, index: (usize, usize), direction: &str) {
    // Update list of locations seen and incrememnt count of unizue coordinates hit
    if !patrol.seen_locations.contains(&index) {
        patrol.seen_locations.insert(index);
        patrol.visit_count += 1
    };

    let next_index;
    let next_direction;

    match next_direction_checker(data, direction, index) {
        Some(i) => {
            next_index = i.0;
            next_direction = i.1;
        }
        None => return,
    }

    traverse(data, patrol, next_index, next_direction);
}

#[allow(dead_code)]
fn next_direction_checker<'a>(
    data: &Vec<Vec<char>>,
    direction: &'a str,
    index: (usize, usize),
) -> Option<((usize, usize), &'a str)> {
    let i_index: (isize, isize) = (
        index.0.try_into().expect("where number"),
        index.1.try_into().expect("where number"),
    );
    let next_direction;

    let try_i_index = match direction {
        "up" => {
            next_direction = "right";
            (i_index.0 - 1, i_index.1)
        }
        "right" => {
            next_direction = "down";
            (i_index.0, i_index.1 + 1)
        }
        "down" => {
            next_direction = "left";
            (i_index.0 + 1, i_index.1)
        }
        _ => {
            next_direction = "up";
            (i_index.0, i_index.1 - 1)
        }
    };

    if try_i_index.0 < 0 || try_i_index.1 < 0 {
        return None;
    }

    let next_index: (usize, usize) = (
        try_i_index.0.try_into().expect("where usize"),
        try_i_index.1.try_into().expect("where usize"),
    );

    if next_index.0 == data.len() || next_index.1 == data[0].len() {
        return None;
    }

    if data[next_index.0][next_index.1] == '#' {
        next_direction_checker(data, next_direction, index)
    } else {
        Some((next_index, direction))
    }
}

// ##############################################################
// ##############################################################
// ##############################################################
// ##################         PART 2        #####################
// ##############################################################
// ##############################################################

fn problem_two(data: &mut Vec<Vec<char>>) -> usize {
    let mut start: (usize, usize) = (42, 42);
    data.iter()
        .enumerate()
        .for_each(|(i, row)| match row.iter().position(|e| *e == '^') {
            Some(j) => start = (i, j),
            _ => (),
        });

    println!("start {:?}", start);

    let mut blocks: HashSet<(usize, usize)> = HashSet::new();

    let mut count: usize = 0;

    traverse_two(
        data,
        start,
        0,
        &mut blocks,
        &mut (start.0 + 1, start.1),
        &mut count
    );

    //count;
    //for ele in blocks.iter() {
    //    println!("block: {:?}", ele);
    //}
    println!("removed?: {:?}", blocks.remove(&start));
    blocks.len()
}

fn traverse_two(
    data: &mut Vec<Vec<char>>,
    index: (usize, usize),
    direction: usize,
    blocks: &mut HashSet<(usize, usize)>,
    block: &mut (usize, usize),
    count: &mut usize
) {
    // Update list of locations seen and incrememnt count of unizue coordinates hit

    let next_index;
    let next_direction;
    let mut probe_patrol = PatrolTwo {
        seen_locations: HashSet::new(),
    };

    probe_patrol.seen_locations.insert((index, direction));

    match next_direction_checker_two(data, direction, index) {
        Some(i) => {
            next_index = i.0;
            next_direction = i.1;

            //add temporary block
            //println!("{:?}", data[next_index.0][next_index.1]);
            data[next_index.0][next_index.1] = '#';
            if traverse_two_probe(data, index, direction, &mut probe_patrol, count) {
                if blocks.insert(next_index) {
                    //println!("block found: {:?}, up to {:?}", next_index, blocks.len());
                }
            }

            // remove temporary block
            data[next_index.0][next_index.1] = '.';
        }
        None => {
            return;
        }
    }

    *count = 0;
    //println!("{:?}", count);
    traverse_two(data, next_index, next_direction,  blocks, block, count);
}

fn traverse_two_probe(
    data: &Vec<Vec<char>>,
    index: (usize, usize),
    direction: usize,
    probe_patrol: &mut PatrolTwo,
    count: &mut usize
) -> bool {
    //println!("Probe testing: {:?} direction: {:?}", index, direction);
    match next_direction_checker_two(data, direction, index) {
        Some(i) => {
            if probe_patrol.seen_locations.insert((i.0, i.1)) {
                //traverse_two_probe(data, i.0, i.1, patrol, probe_patrol, count)
            } else {
                *count += 1;
                //println!("repeat: {:?}", (index, direction));
            }

            if *count > 100 {
                true
            } else {
                traverse_two_probe(data, i.0, i.1, probe_patrol, count)
            }
        }
        None => false,
    }
}

fn next_direction_checker_two<'a>(
    data: &Vec<Vec<char>>,
    direction: usize,
    index: (usize, usize)
) -> Option<((usize, usize), usize)> {
    let i_index: (isize, isize) = (
        index.0.try_into().expect("where number"),
        index.1.try_into().expect("where number"),
    );
    let next_direction: usize;

    let try_i_index = match direction {
        0 => {
            next_direction = 1;
            (i_index.0 - 1, i_index.1)
        }
        1 => {
            next_direction = 2;
            (i_index.0, i_index.1 + 1)
        }
        2 => {
            next_direction = 3;
            (i_index.0 + 1, i_index.1)
        }
        _ => {
            next_direction = 0;
            (i_index.0, i_index.1 - 1)
        }
    };

    //if !probe {println!("{:?}: probe, index: {:?}", probe, try_i_index);}
    if try_i_index.0 < 0 || try_i_index.1 < 0 {
        return None;
    }

    let next_index: (usize, usize) = (
        try_i_index.0.try_into().expect("where usize"),
        try_i_index.1.try_into().expect("where usize"),
    );

    if next_index.0 == data.len() || next_index.1 == data[0].len() {
        return None;
    }

    if data[next_index.0][next_index.1] == '#' {
        next_direction_checker_two(data, next_direction, index)
    } else {
        //println!("Next_direction: next index {:?}, probe: {:?}", next_index, probe);
        Some((next_index, direction))
    }
}

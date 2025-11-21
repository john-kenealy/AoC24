use std::{
    char,
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
    isize, usize,
};

#[derive(Debug)]
struct Route {
    seen_locations: HashSet<Position>,
    visit_count: usize,
}

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
struct Position {
    y: usize,
    x: usize,
    direction: Cardinal,
}

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
enum Cardinal {
    N,
    S,
    E,
    W,
}

struct Map {
    grid: Vec<Vec<char>>,
}

pub fn solve() {
    println!("day six:");
    let mut route = Route {
        seen_locations: HashSet::new(),
        visit_count: 0,
    };

    let mut _blocks: HashSet<(usize, usize)> = HashSet::new();

    let mut map = Map { grid: import() };

    use std::time::Instant;
    let now = Instant::now();

    //problem_one(&data, &mut patrol);

    let count = problem_one(&mut map.grid, &mut route);

    let elapsed = now.elapsed();
    println!("Part 1 Elapsed: {:.2?}", elapsed);
    println!("Part 2 Elapsed: {:.2?}", elapsed);

    println!("Count of locations visited: {:?}", route.visit_count);
    println!("Count of locations to obstruct: {:?}", count);
}

fn import() -> Vec<Vec<char>> {
    let doc = File::open("inputs/six").expect("where file");
    let reader = BufReader::new(doc);

    reader
        .lines()
        .map(|l| l.expect("where char").chars().collect::<Vec<_>>())
        .collect()
}

fn problem_one(data: &Vec<Vec<char>>, route: &mut Route) {
    let mut start: (usize, usize) = (42, 42);
    data.iter()
        .enumerate()
        .for_each(|(i, row)| match row.iter().position(|e| *e == '^') {
            Some(j) => start = (i, j),
            _ => (),
        });

    println!("start {:?}", start);
    patrol(
        data,
        route,
        Position {
            y: start.0,
            x: start.1,
            direction: Cardinal::N,
        },
    );
}

fn patrol(data: &Vec<Vec<char>>, route: &mut Route, mut current_position: Position) {
    // 1. add current_position position to route
    add_to_route(route, &current_position);

    // 2. determine if we are at the end of the route
    if check_for_edge(&current_position, data.len(), data[0].len()) {
        println!("found edge.");
        return;
    }

    // 3. determing next step
    let mut next_position = get_next_position(&current_position);
    let mut end_patrol: bool = false;

    // 3.1 check next spot for
    while data[next_position.y][next_position.x] == '#' {
        current_position.direction = next_cardinal_direction(current_position.direction);

        if !add_to_route(route, &current_position) {
            println!("encountered infinite route");
            end_patrol = true;
            break;
        }

        if check_for_edge(&current_position, data.len(), data[0].len()) {
            println!("leaving the grid");
            end_patrol = true;
            break;
        }

        next_position = get_next_position(&current_position);
    }

    if !end_patrol {
        patrol(data, route, next_position);
    } else {
        println!("found edge or infinite loop.")
    }
}

fn next_cardinal_direction(current_direction: Cardinal) -> Cardinal {
    match current_direction {
        Cardinal::N => Cardinal::E,
        Cardinal::E => Cardinal::S,
        Cardinal::S => Cardinal::W,
        Cardinal::W => Cardinal::N,
    }
}

fn get_next_position(current_position: &Position) -> Position {
    match current_position.direction {
        Cardinal::N => Position {
            y: current_position.y - 1,
            x: current_position.x,
            direction: Cardinal::N,
        },
        Cardinal::E => Position {
            y: current_position.y,
            x: current_position.x + 1,
            direction: Cardinal::E,
        },
        Cardinal::S => Position {
            y: current_position.y + 1,
            x: current_position.x,
            direction: Cardinal::S,
        },
        Cardinal::W => Position {
            y: current_position.y,
            x: current_position.x - 1,
            direction: Cardinal::W,
        },
    }
}

fn add_to_route(route: &mut Route, position: &Position) -> bool {
    let res = route.seen_locations.insert(position.clone());

    if res {
        route.visit_count += 1;
    };

    res
}

fn check_for_edge(position: &Position, height: usize, width: usize) -> bool {
    match position.direction {
        Cardinal::N => {
            if position.y == 0 {
                return true;
            }
        }
        Cardinal::E => {
            if position.x == width - 1 {
                return true;
            }
        }
        Cardinal::S => {
            if position.y == height - 1 {
                return true;
            }
        }
        Cardinal::W => {
            if position.x == 0 {
                return true;
            }
        }
    };

    return false;
}

//fn add_position(
//    data: &mut Vec<Vec<char>>,
//    position: (usize, usize),
//    direction: &str,
//    route: &mut Route,
//) {
//    route.seen_locations.insert(position)
//}

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

//fn problem_two(map: &mut Map) -> usize {
//    let mut start: (usize, usize) = (42, 42);
//    map.grid.iter()
//        .enumerate()
//        .for_each(|(i, row)| match row.iter().position(|e| *e == '^') {
//            Some(j) => start = (i, j),
//            _ => (),
//        });
//
//    println!("start {:?}", start);
//
//    let mut blocks: HashSet<(usize, usize)> = HashSet::new();
//
//    let mut count: usize = 0;
//
//    traverse_two(
//        map,
//        start,
//        0,
//        &mut blocks,
//        &mut (start.0 + 1, start.1),
//        &mut count
//    );
//
//    //count;
//    //for ele in blocks.iter() {
//    //    println!("block: {:?}", ele);
//    //}
//    println!("removed?: {:?}", blocks.remove(&start));
//    blocks.len()
//}
//
//fn traverse_two(
//    map: &mut Map,
//    index: (usize, usize),
//    direction: usize,
//    blocks: &mut HashSet<(usize, usize)>,
//    block: &mut (usize, usize),
//    count: &mut usize
//) {
//    // Update list of locations seen and incrememnt count of coordinates hit
//
//    let next_index;
//    let next_direction;
//    let mut probe_patrol = PatrolTwo {
//        seen_locations: HashSet::new(),
//    };
//
//    probe_patrol.seen_locations.insert((index, direction));
//
//    match next_direction_checker_two(map, direction, index) {
//        Some(i) => {
//            next_index = i.0;
//            next_direction = i.1;
//
//            //add temporary block
//            //println!("{:?}", data[next_index.0][next_index.1]);
//            map.grid[next_index.0][next_index.1] = '#';
//            if traverse_two_probe(map, index, direction, &mut probe_patrol, count) {
//                if blocks.insert(next_index) {
//                    //println!("block found: {:?}, up to {:?}", next_index, blocks.len());
//                }
//            }
//
//            // remove temporary block
//            map.grid[next_index.0][next_index.1] = '.';
//        }
//        None => {
//            return;
//        }
//    }
//
//    *count = 0;
//    //println!("{:?}", count);
//    traverse_two(map, next_index, next_direction,  blocks, block, count);
//}
//
//fn traverse_two_probe(
//    map: &Map,
//    index: (usize, usize),
//    direction: usize,
//    probe_patrol: &mut PatrolTwo,
//    count: &mut usize
//) -> bool {
//    //println!("Probe testing: {:?} direction: {:?}", index, direction);
//    match next_direction_checker_two(map, direction, index) {
//        Some(i) => {
//            if probe_patrol.seen_locations.insert((i.0, i.1)) {
//                //traverse_two_probe(data, i.0, i.1, patrol, probe_patrol, count)
//            } else {
//                *count += 1;
//                //println!("repeat: {:?}", (index, direction));
//            }
//
//            if *count > 100 {
//                true
//            } else {
//                traverse_two_probe(map, i.0, i.1, probe_patrol, count)
//            }
//        }
//        None => false,
//    }
//}
//
//fn next_direction_checker_two<'a>(
//    map: &Map,
//    direction: usize,
//    index: (usize, usize)
//) -> Option<((usize, usize), usize)> {
//    let i_index: (isize, isize) = (
//        index.0.try_into().expect("where number"),
//        index.1.try_into().expect("where number"),
//    );
//    let next_direction: usize;
//
//    let try_i_index = match direction {
//        0 => {
//            next_direction = 1;
//            (i_index.0 - 1, i_index.1)
//        }
//        1 => {
//            next_direction = 2;
//            (i_index.0, i_index.1 + 1)
//        }
//        2 => {
//            next_direction = 3;
//            (i_index.0 + 1, i_index.1)
//        }
//        _ => {
//            next_direction = 0;
//            (i_index.0, i_index.1 - 1)
//        }
//    };
//
//    //if !probe {println!("{:?}: probe, index: {:?}", probe, try_i_index);}
//    if try_i_index.0 < 0 || try_i_index.1 < 0 {
//        return None;
//    }
//
//    let next_index: (usize, usize) = (
//        try_i_index.0.try_into().expect("where usize"),
//        try_i_index.1.try_into().expect("where usize"),
//    );
//
//    if next_index.0 == map.grid.len() || next_index.1 == map.grid[0].len() {
//        return None;
//    }
//
//    if map.grid[next_index.0][next_index.1] == '#' {
//        next_direction_checker_two(map, next_direction, index)
//    } else {
//        //println!("Next_direction: next index {:?}, probe: {:?}", next_index, probe);
//        Some((next_index, direction))
//    }
//}

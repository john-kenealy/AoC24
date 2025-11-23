use std::{
    char,
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
    usize,
};

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct Route {
    seen_locations: HashSet<Position>,
    visit_count: usize,
}

#[allow(dead_code)]
#[derive(Eq, Hash, PartialEq, Clone, Debug)]
struct Position {
    y: usize,
    x: usize,
    direction: Cardinal,
}

#[allow(dead_code)]
#[derive(Eq, Hash, PartialEq, Clone, Debug)]
enum Cardinal {
    N,
    S,
    E,
    W,
}

#[allow(dead_code)]
struct Map {
    grid: Vec<Vec<char>>,
}

#[allow(dead_code)]
pub fn solve() {
    println!("day six:");
    let mut route = Route {
        seen_locations: HashSet::new(),
        visit_count: 0,
    };

    let mut blocks: HashSet<(usize, usize)> = HashSet::new();

    let mut map = Map { grid: import() };

    use std::time::Instant;
    let now = Instant::now();

    problem_one(&mut map.grid, &mut route);


    let mut part_1_locations: HashSet<(usize, usize)> = HashSet::new();
    for location in &route.seen_locations {
        part_1_locations.insert((location.y, location.x));
    }

    for p1 in part_1_locations {
        let mut test_data = map.grid.clone();
        test_data[p1.0][p1.1] = '#';

        if patrol(
            &test_data,
            &mut Route {
                seen_locations: HashSet::new(),
                visit_count: 0,
            },
            Position {
                y: 39,
                x: 46,
                direction: Cardinal::N,
            },
        ) {
            blocks.insert((p1.0, p1.1));
            //println!("{:?}", blocks.len());
        }
    }

    blocks.remove(&(39, 46));

    let elapsed = now.elapsed();
    println!("Part 1 Elapsed: {:.2?}", elapsed);
    println!("Part 2 Elapsed: {:.2?}", elapsed);
    //println!(
    //    "Count of distinct locations visited: {:?}",
    //    part_1_locations.len()
    //);
    println!("Count of locations to obstruct: {:?}", blocks.len());
}

#[allow(dead_code)]
fn import() -> Vec<Vec<char>> {
    let doc = File::open("inputs/six").expect("where file");
    let reader = BufReader::new(doc);

    reader
        .lines()
        .map(|l| l.expect("where char").chars().collect::<Vec<_>>())
        .collect()
}

#[allow(dead_code)]
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

#[allow(dead_code)]
fn patrol(data: &Vec<Vec<char>>, route: &mut Route, mut current_position: Position) -> bool {
    // 1. add current_position position to route
    add_to_route(route, &current_position);

    // 2. determine if we are at the end of the route
    if check_for_edge(&current_position, data.len(), data[0].len()) {
        //println!("found edge.");

        return false;
    }

    // 3. determing next step
    let mut next_position = get_next_position(&current_position);
    let mut end_patrol: bool = false;
    let mut infinite_loop: bool = false;

    // 3.1 check next spot for
    while data[next_position.y][next_position.x] == '#' {
        current_position.direction = next_cardinal_direction(current_position.direction);

        if !add_to_route(route, &current_position) {
            //println!("encountered infinite route");
            infinite_loop = true;
            end_patrol = true;
            break;
        }

        if check_for_edge(&current_position, data.len(), data[0].len()) {
            //println!("leaving the grid");
            end_patrol = true;
            break;
        }

        next_position = get_next_position(&current_position);
    }

    if !end_patrol {
        patrol(data, route, next_position)
    } else {
        infinite_loop   
    }
}

#[allow(dead_code)]
fn next_cardinal_direction(current_direction: Cardinal) -> Cardinal {
    match current_direction {
        Cardinal::N => Cardinal::E,
        Cardinal::E => Cardinal::S,
        Cardinal::S => Cardinal::W,
        Cardinal::W => Cardinal::N,
    }
}

#[allow(dead_code)]
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

#[allow(dead_code)]
fn add_to_route(route: &mut Route, position: &Position) -> bool {
    let res = route.seen_locations.insert(position.clone());

    if res {
        route.visit_count += 1;
    };

    res
}

#[allow(dead_code)]
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

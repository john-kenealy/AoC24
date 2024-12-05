use std::{
    fs::File,
    io::{BufRead, BufReader},
    usize,
};

#[allow(dead_code)]
pub fn solve() {
    println!("day four:");

    let doc = File::open("../inputs/four").expect("where file");
    let reader = BufReader::new(doc);

    let char_vec: Vec<Vec<char>> = reader
        .lines()
        .map(|l| l.expect("where string").chars().collect::<Vec<_>>())
        .collect();

    //println!("{:?}", char_vec[0][0]);

    let h: i32 = char_vec.len().try_into().unwrap();
    let w: i32 = char_vec[0].len().try_into().unwrap();

    let mut counter: usize = 0;

    //char_vec.iter().enumerate().for_each(|(i, row)| {
    //    row.iter().enumerate().for_each(|(j, char)| {
    //        if *char == 'X' {
    //            xmas_finder(
    //                &char_vec,
    //                'M',
    //                i.try_into().unwrap(),
    //                j.try_into().unwrap(),
    //                h,
    //                w,
    //                0,
    //                0,
    //                &mut counter,
    //            );
    //        }
    //    });
    //});

    char_vec.iter().enumerate().for_each(|(i, row)| {
        row.iter().enumerate().for_each(|(j, char)| {
            if *char == 'A' {
                mas_finder(
                    &char_vec,
                    i.try_into().unwrap(),
                    j.try_into().unwrap(),
                    h,
                    w,
                    true,
                    &mut counter,
                );
            }
        });
    });

    println!("Matches Found: {}", counter)
}

#[allow(dead_code)]
fn xmas_finder(
    puzzle: &Vec<Vec<char>>,
    next_letter: char,
    current_row: i32,
    current_col: i32,
    h: i32,
    w: i32,
    row_inc: i32,
    col_inc: i32,
    counter: &mut usize,
) {
    match next_letter {
        'M' => {
            for row in -1..2 {
                for col in -1..2 {
                    if coord_validater(h, w, current_row + row, current_col + col) {
                        let new_row: usize = usize::try_from(current_row + row).unwrap();
                        let new_col: usize = usize::try_from(current_col + col).unwrap();

                        if puzzle[new_row][new_col] == 'M' {
                            xmas_finder(
                                puzzle,
                                'A',
                                new_row.try_into().unwrap(),
                                new_col.try_into().unwrap(),
                                h,
                                w,
                                row,
                                col,
                                counter,
                            );
                        }
                    }
                }
            }
        }
        'A' => {
            if coord_validater(h, w, current_row + row_inc, current_col + col_inc) {
                let new_row: usize = usize::try_from(current_row + row_inc).unwrap();
                let new_col: usize = usize::try_from(current_col + col_inc).unwrap();

                if puzzle[new_row][new_col] == 'A' {
                    xmas_finder(
                        puzzle,
                        'S',
                        new_row.try_into().unwrap(),
                        new_col.try_into().unwrap(),
                        h,
                        w,
                        row_inc,
                        col_inc,
                        counter,
                    );
                }
            }
        }
        'S' => {
            if coord_validater(h, w, current_row + row_inc, current_col + col_inc) {
                let new_row: usize = usize::try_from(current_row + row_inc).unwrap();
                let new_col: usize = usize::try_from(current_col + col_inc).unwrap();

                if puzzle[new_row][new_col] == 'S' {
                    *counter += 1;
                }
            }
        }
        _ => println!("WHERE X M A S ?"),
    }
}

#[allow(dead_code)]
fn mas_finder(
    puzzle: &Vec<Vec<char>>,
    current_row: i32,
    current_col: i32,
    h: i32,
    w: i32,
    pass_one: bool,
    counter: &mut usize,
) {
    match pass_one {
        true => {
            if coord_validater(h, w, current_row - 1, current_col - 1)
                && coord_validater(h, w, current_row + 1, current_col + 1)
            {
                let new_row: usize = usize::try_from(current_row - 1).unwrap();
                let new_col: usize = usize::try_from(current_col - 1).unwrap();

                match puzzle[new_row][new_col] {
                    'M' => {
                        if puzzle[new_row + 2][new_col + 2] == 'S' {
                            mas_finder(puzzle, current_row, current_col, h, w, false, counter);
                        }
                    }
                    'S' => {
                        if puzzle[new_row + 2][new_col + 2] == 'M' {
                            mas_finder(puzzle, current_row, current_col, h, w, false, counter);
                        }
                    }
                    _ => return,
                }
            }
        }
        false => {
            if coord_validater(h, w, current_row - 1, current_col + 1)
                && coord_validater(h, w, current_row + 1, current_col - 1)
            {
                let new_row: usize = usize::try_from(current_row - 1).unwrap();
                let new_col: usize = usize::try_from(current_col + 1).unwrap();

                match puzzle[new_row][new_col] {
                    'M' => {
                        if puzzle[new_row + 2][new_col - 2] == 'S' {
                            *counter += 1;
                        }
                    }
                    'S' => {
                        if puzzle[new_row + 2][new_col - 2] == 'M' {
                            *counter += 1;
                        }
                    }
                    _ => return,
                }
            }
        }
    }
}

fn coord_validater(h: i32, w: i32, row: i32, col: i32) -> bool {
    if row < 0 || col < 0 {
        return false;
    } else if row > h - 1 || col > w - 1 {
        return false;
    }

    true
}

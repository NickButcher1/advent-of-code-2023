use aoc::board::Board;
use itertools::iproduct;
use std::collections::HashSet;

const COMPLETE: char = '.';

fn perimeta_count(board: &Board, r: usize, c: usize, key: char) -> i32 {
    let mut perimeta_count = 0;
    [(-1, 0), (0, 1), (1, 0), (0, -1)]
        .iter()
        .for_each(|(dr, dc)| {
            let new_r = (r as i32 + dr) as usize;
            let new_c = (c as i32 + dc) as usize;
            if board.cells[new_r][new_c] != key {
                perimeta_count += 1;
            }
        });
    perimeta_count
}

fn is_left_border(board: &Board, r: usize, c: usize, key: char) -> bool {
    board.cells[r][c - 1] != key
}

fn is_right_border(board: &Board, r: usize, c: usize, key: char) -> bool {
    board.cells[r][c + 1] != key
}

fn is_top_border(board: &Board, r: usize, c: usize, key: char) -> bool {
    board.cells[r - 1][c] != key
}

fn is_bottom_border(board: &Board, r: usize, c: usize, key: char) -> bool {
    board.cells[r + 1][c] != key
}

fn is_border_of_region(board: &Board, r: usize, c: usize, key: char) -> (bool, bool, bool, bool) {
    (
        board.cells[r - 1][c] != key,
        board.cells[r + 1][c] != key,
        board.cells[r][c - 1] != key,
        board.cells[r][c + 1] != key,
    )
}

fn count_sides(board: &Board, points: Vec<(usize, usize)>, key: char) -> i32 {
    return if points.len() == 1 {
        4
    } else if points.len() == 2 {
        4
    } else {
        // Discard points not on the border.
        let mut outer_points: HashSet<(usize, usize)> = HashSet::new();

        for (r, c) in &points {
            let (top, bottom, left, right) = is_border_of_region(board, *r, *c, key);
            if top || bottom || left || right {
                outer_points.insert((*r, *c));
            }
        }

        let mut sides = 0;

        for (r, c) in points {
            // Count left side only if above or below is a corner.
            if is_left_border(board, r, c, key) {
                if outer_points.contains(&(r - 1, c)) {
                    if !is_left_border(board, r - 1, c, key) {
                        sides += 1;
                    }
                } else {
                    sides += 1;
                }

                if outer_points.contains(&(r + 1, c)) {
                    if !is_left_border(board, r + 1, c, key) {
                        sides += 1;
                    }
                } else {
                    sides += 1;
                }
            }

            if is_right_border(board, r, c, key) {
                if outer_points.contains(&(r - 1, c)) {
                    if !is_right_border(board, r - 1, c, key) {
                        sides += 1;
                    }
                } else {
                    sides += 1;
                }

                if outer_points.contains(&(r - 1, c)) {
                    if !is_right_border(board, r - 1, c, key) {
                        sides += 1;
                    }
                } else {
                    sides += 1;
                }
            }

            if is_top_border(board, r, c, key) {
                if outer_points.contains(&(r, c - 1)) {
                    if !is_top_border(board, r, c - 1, key) {
                        sides += 1;
                    }
                } else {
                    sides += 1;
                }

                if outer_points.contains(&(r, c - 1)) {
                    if !is_top_border(board, r, c - 1, key) {
                        sides += 1;
                    }
                } else {
                    sides += 1;
                }
            }

            if is_bottom_border(board, r, c, key) {
                if outer_points.contains(&(r, c + 1)) {
                    if !is_bottom_border(board, r, c + 1, key) {
                        sides += 1;
                    }
                } else {
                    sides += 1;
                }

                if outer_points.contains(&(r, c + 1)) {
                    if !is_bottom_border(board, r, c + 1, key) {
                        sides += 1;
                    }
                } else {
                    sides += 1;
                }
            }
        }

        sides / 2
    };
}

pub fn solve12(input: &[String]) -> (i128, i128) {
    let mut input_board = Board::from_input(input);
    input_board.add_border(COMPLETE);
    let mut board = Board::from_input(input);
    board.add_border(COMPLETE);
    let mut regions = vec![];

    // Loop over every board cell. Each time a cell we haven't processed yet is found, flood fill it.
    let mut queue: Vec<(usize, usize)> = vec![];

    for (r, c) in iproduct!(1..(board.num_rows - 1), 1..(board.num_cols - 1)) {
        if board.cells[r][c] != COMPLETE {
            let key = board.cells[r][c];

            let mut area = 0;
            let mut perimeta = 0;
            let mut points_in_region = vec![];

            board.cells[r][c] = COMPLETE;
            area += 1;
            perimeta += perimeta_count(&input_board, r, c, key);
            points_in_region.push((r, c));
            queue.push((r, c));

            while !queue.is_empty() {
                let (test_r, test_c) = queue.pop().unwrap();
                [(-1, 0), (0, 1), (1, 0), (0, -1)]
                    .iter()
                    .for_each(|(dr, dc)| {
                        let new_r = (test_r as i32 + dr) as usize;
                        let new_c = (test_c as i32 + dc) as usize;
                        if board.cells[new_r][new_c] == key {
                            board.cells[new_r][new_c] = COMPLETE;
                            area += 1;
                            perimeta += perimeta_count(&input_board, new_r, new_c, key);
                            points_in_region.push((new_r, new_c));
                            queue.push((new_r, new_c));
                        }
                    });
            }

            regions.push((
                (r, c),
                key,
                area,
                perimeta,
                count_sides(&input_board, points_in_region, key),
            ));
        }
    }

    let mut total_cost_one = 0;
    let mut total_cost_two = 0;
    for ((r, c), key, area, perimeta, num_sides) in regions {
        total_cost_one += area * perimeta;
        total_cost_two += area * num_sides;
    }

    (total_cost_one as i128, total_cost_two as i128)
}

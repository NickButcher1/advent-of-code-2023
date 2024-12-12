use aoc::board::Board;
use itertools::iproduct;
use std::collections::HashSet;

const COMPLETE: char = '.';
const ALL_DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn perimeter_count_for_cell(board: &Board, r: usize, c: usize, key: char) -> i32 {
    ALL_DIRECTIONS
        .iter()
        .filter(|&(dr, dc)| {
            let new_r = (r as i32 + dr) as usize;
            let new_c = (c as i32 + dc) as usize;
            board.cells[new_r][new_c] != key
        })
        .count() as i32
}

fn is_border(board: &Board, r: usize, c: usize, dr: i32, dc: i32) -> bool {
    board.cells[(r as i32 + dr) as usize][(c as i32 + dc) as usize] != board.cells[r][c]
}

fn is_corner(
    board: &Board,
    points: &HashSet<(usize, usize)>,
    r: usize,
    c: usize,
    dr: i32,
    dc: i32,
) -> i32 {
    if !points.contains(&(r, c)) || !is_border(board, r, c, dr, dc) {
        1
    } else {
        0
    }
}

fn count_sides(board: &Board, points: &HashSet<(usize, usize)>) -> i32 {
    // For every cell X in the region, consider each of these four cases around it:
    //
    //   BOB    AXA    BA    AB
    //   AXA    BOB    OX    XO
    //                 BA    AB
    //
    // If XO is a border of X's region, then for each of the two AB, we have a corner unless AB is
    // a continuation of the border XO.  The number of sides equals the number of corners for any
    // region, but because we count each corner twice (once for each of the two sides that touches
    // it, we halve the number of corners to get the number of sides.
    let mut corners = 0;

    for (r, c) in points.clone().into_iter() {
        if is_border(board, r, c, -1, 0) {
            corners += is_corner(board, points, r, c - 1, -1, 0);
            corners += is_corner(board, points, r, c + 1, -1, 0);
        }

        if is_border(board, r, c, 1, 0) {
            corners += is_corner(board, points, r, c - 1, 1, 0);
            corners += is_corner(board, points, r, c + 1, 1, 0);
        }

        if is_border(board, r, c, 0, -1) {
            corners += is_corner(board, points, r - 1, c, 0, -1);
            corners += is_corner(board, points, r + 1, c, 0, -1);
        }

        if is_border(board, r, c, 0, 1) {
            corners += is_corner(board, points, r - 1, c, 0, 1);
            corners += is_corner(board, points, r + 1, c, 0, 1);
        }
    }

    corners / 2
}

pub fn solve12(input: &[String]) -> (i128, i128) {
    let mut input_board = Board::from_input(input);
    input_board.add_border(COMPLETE);
    let mut board = input_board.clone();

    let mut queue: Vec<(usize, usize)> = vec![];

    let mut total_cost_one = 0;
    let mut total_cost_two = 0;

    // Loop over every board cell. Each time a cell we haven't processed yet is found, flood fill it
    // in order to calculate its area.
    for (r, c) in iproduct!(1..(board.num_rows - 1), 1..(board.num_cols - 1)) {
        if board.cells[r][c] != COMPLETE {
            let key = board.cells[r][c];

            let mut points_in_region: HashSet<(usize, usize)> = HashSet::new();

            board.cells[r][c] = COMPLETE;
            points_in_region.insert((r, c));
            queue.push((r, c));

            while !queue.is_empty() {
                let (test_r, test_c) = queue.pop().unwrap();
                ALL_DIRECTIONS.iter().for_each(|(dr, dc)| {
                    let new_r = (test_r as i32 + dr) as usize;
                    let new_c = (test_c as i32 + dc) as usize;
                    if board.cells[new_r][new_c] == key {
                        board.cells[new_r][new_c] = COMPLETE;
                        points_in_region.insert((new_r, new_c));
                        queue.push((new_r, new_c));
                    }
                });
            }

            let area = points_in_region.len() as i32;
            let perimeter = points_in_region
                .iter()
                .map(|&(r, c)| perimeter_count_for_cell(&input_board, r, c, key))
                .sum::<i32>();
            total_cost_one += area * perimeter;
            total_cost_two += area * count_sides(&input_board, &points_in_region);
        }
    }

    (total_cost_one as i128, total_cost_two as i128)
}

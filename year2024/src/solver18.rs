use aoc::board::Board;
use aoc::int_board::IntBoard;
use aoc::point::{read_points, Point};
use aoc::solution::{Solution, Solutions};
use itertools::iproduct;
use std::collections::VecDeque;

const SAFE: char = '.';
const CORRUPT: char = '#';

struct Position {
    r: usize,
    c: usize,
    steps: usize,
}

fn find_lowest_steps_for_board(board: &Board) -> i32 {
    let (target_r, target_c) = (board.num_rows - 2, board.num_cols - 2);
    // Tips of paths that need to be explored.
    let mut live_points = VecDeque::from([Position {
        r: 1,
        c: 1,
        steps: 0,
    }]);

    // Track the lowest score to reach each cell, found so far.
    let mut lowest_steps_to_end = IntBoard::create_empty(board.num_rows + 2, board.num_cols + 2);
    for (c, r) in iproduct!(
        0..lowest_steps_to_end.num_cols,
        0..lowest_steps_to_end.num_rows
    ) {
        lowest_steps_to_end.cells[r][c] = i32::MAX;
    }
    lowest_steps_to_end.cells[1][1] = 0;

    while let Some(position) = live_points.pop_front() {
        for new_dir in 0..=3 {
            let (dr, dc) = [(-1, 0), (0, 1), (1, 0), (0, -1)][new_dir];
            let new_position = Position {
                r: (position.r as i32 + dr) as usize,
                c: (position.c as i32 + dc) as usize,
                steps: position.steps + 1,
            };

            if board.cells[new_position.r][new_position.c] == SAFE
                && new_position.steps
                    < lowest_steps_to_end.cells[new_position.r][new_position.c] as usize
            {
                lowest_steps_to_end.cells[new_position.r][new_position.c] =
                    new_position.steps as i32;

                if new_position.r != target_r || new_position.c != target_c {
                    live_points.push_back(new_position);
                }
            }
        }
    }

    lowest_steps_to_end.cells[target_r][target_c]
}

fn parse_input(input: &[String]) -> (Board, Vec<Point>) {
    let points = read_points(input);
    let (board_size, max_points_part_one) = if points.len() == 25 {
        (7, 12)
    } else {
        (71, 1024)
    };
    let mut board = Board::create_empty(board_size, board_size, SAFE);
    board.add_border(CORRUPT);

    for point in points.iter().take(max_points_part_one) {
        board.cells[point.y as usize + 1][point.x as usize + 1] = CORRUPT;
    }

    (board, points)
}

pub fn solve18(input: &[String]) -> Solutions {
    let (mut board, points) = parse_input(input);

    let solution_one = find_lowest_steps_for_board(&board);

    // Now add the remaining corrupt points.
    for point in &points {
        board.cells[point.y as usize + 1][point.x as usize + 1] = CORRUPT;
    }

    // Brute force - remove the remaining points one at a time, and check for a path. As soon as
    // there is a path, we know the point that needs to be removed to solve part two.
    let solution_two = points
        .iter()
        .rev()
        .find_map(|point| {
            board.cells[point.y as usize + 1][point.x as usize + 1] = SAFE;
            let lowest_steps = find_lowest_steps_for_board(&board);

            (lowest_steps != i32::MAX).then(|| format!("{},{}", point.x, point.y))
        })
        .unwrap_or_default();

    (Solution::I32(solution_one), Solution::STR(solution_two))
}

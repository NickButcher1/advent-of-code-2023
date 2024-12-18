use aoc::board::Board;
use aoc::int_board::IntBoard;
use aoc::point::read_points;
use aoc::solution::{Solution, Solutions};
use itertools::iproduct;
use std::collections::VecDeque;

const SAFE: char = '.';
const CORRUPT: char = '#';

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct Position {
    r: usize,
    c: usize,
    steps: usize,
}

fn find_lowest_steps_for_board(board: &Board, board_size: usize) -> i32 {
    let start_position = Position {
        r: 1,
        c: 1,
        steps: 0,
    };
    let (target_r, target_c) = (board_size, board_size);
    // Tips of paths that need to be explored.
    let mut live_points: VecDeque<Position> = VecDeque::new();
    live_points.push_back(start_position.clone());

    let mut lowest_steps_to_end = IntBoard::create_empty(board_size + 2, board_size + 2);
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

            if board.cells[new_position.r][new_position.c] == SAFE {
                if new_position.steps
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
    }

    lowest_steps_to_end.cells[target_r][target_c]
}

pub fn solve18(input: &[String]) -> Solutions {
    let points = read_points(input);
    let (board_size, max_points_part_one) = if points.len() == 25 {
        (7, 12)
    } else {
        (71, 1024)
    };
    let mut board = Board::create_empty(board_size, board_size, SAFE);
    board.add_border(CORRUPT);

    // Part one.
    for i in 0..max_points_part_one {
        board.cells[points[i].y as usize + 1][points[i].x as usize + 1] = CORRUPT;
    }

    let solution_one = find_lowest_steps_for_board(&board, board_size);

    // Part two.
    // Now add the remaining corrupt points.
    for i in 0..points.len() {
        board.cells[points[i].y as usize + 1][points[i].x as usize + 1] = CORRUPT;
    }

    // Brute force - remove the remaining points one at a time, and check for a path. As long as
    // there is a path, we have solved it.
    let mut solution_two = "".to_string();
    for i in (0..points.len()).rev() {
        board.cells[points[i].y as usize + 1][points[i].x as usize + 1] = SAFE;
        let lowest_steps = find_lowest_steps_for_board(&board, board_size);
        if lowest_steps != i32::MAX {
            solution_two = format!("{},{}", points[i].x, points[i].y).to_string();
            break;
        }
    }

    (Solution::I32(solution_one), Solution::STR(solution_two))
}

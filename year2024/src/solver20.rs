use std::collections::VecDeque;
use itertools::iproduct;
use aoc::board::Board;
use aoc::int_board::IntBoard;
use aoc::solution::{Solution, Solutions};

const EMPTY: char = '.';
const WALL: char = '#';
const START: char = 'S';
const END: char = 'E';

// TODO Remove some of these.
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct Position {
    r: usize,
    c: usize,
    steps: usize,
    // cheat_start_r: usize,
    // cheat_start_c: usize,
    // cheat_end_r: usize,
    // cheat_end_c: usize,
}

fn find_lowest_steps_for_board(board: &Board, start: (usize, usize), end: (usize, usize)) -> i32 {
    // Tips of paths that need to be explored.
    let mut live_points = VecDeque::from([Position {
        r: start.0,
        c: start.1,
        steps: 0,
    }]);

    // Track the lowest score to reach each cell, found so far.
    let mut lowest_steps_to_end = IntBoard::create_empty(board.num_rows, board.num_cols);
    for (c, r) in iproduct!(
        0..lowest_steps_to_end.num_cols,
        0..lowest_steps_to_end.num_rows
    ) {
        lowest_steps_to_end.cells[r][c] = i32::MAX;
    }
    lowest_steps_to_end.cells[start.0][start.1] = 0;

    while let Some(position) = live_points.pop_front() {
        for new_dir in 0..=3 {
            let (dr, dc) = [(-1, 0), (0, 1), (1, 0), (0, -1)][new_dir];
            let new_position = Position {
                r: (position.r as i32 + dr) as usize,
                c: (position.c as i32 + dc) as usize,
                steps: position.steps + 1,
            };

            if board.cells[new_position.r][new_position.c] == EMPTY
                && new_position.steps
                < lowest_steps_to_end.cells[new_position.r][new_position.c] as usize
            {
                lowest_steps_to_end.cells[new_position.r][new_position.c] =
                    new_position.steps as i32;

                if new_position.r != end.0 || new_position.c != end.1 {
                    live_points.push_back(new_position);
                }
            }
        }
    }

    lowest_steps_to_end.cells[end.0][end.1]
}

pub fn solve20(input: &[String]) -> Solutions {
    let mut board = Board::from_input(input);
    // The board already has a border, but because we want to look 2 steps ahead in any direction,
    // we need a double border.
    board.add_border(WALL);
    let start = board.find(START);
    let end = board.find(END);
    board.cells[start.0][start.1] = EMPTY;
    board.cells[end.0][end.1] = EMPTY;

    println!("START: {start:?}");
    println!("END:   {end:?}");

    // First the unmodified board.
    let unmodified_cheapest_path = find_lowest_steps_for_board(&board, start, end);
    println!("CHEAPEST PATH: {unmodified_cheapest_path}");

    // Then try every possible cheat.
    let mut solution_one = 0;
    let mut savings = vec![0; 10_000];
    for (r, c) in iproduct!(1..(board.num_rows - 1), 1..(board.num_cols - 1)) {
        // Only remove a wall if it has an empty cell either side of it.
        if board.cells[r][c] == WALL && ((board.cells[r - 1][c] == EMPTY && board.cells[r + 1][c] == EMPTY) || (board.cells[r][c - 1] == EMPTY && board.cells[r][c + 1] == EMPTY))    {
            let mut cheat_board = board.clone();
            cheat_board.cells[r][c] = EMPTY;
            let cheapest_path = find_lowest_steps_for_board(&cheat_board, start, end);
            let saving = unmodified_cheapest_path - cheapest_path;
            savings[saving as usize] += 1;
            // println!("Remove wall at {r},{c}    cheapest path {cheapest_path}    saving {saving}");
            if saving >= 100 {
                solution_one += 1;
            }
        }
    }

    println!("\nSAVINGS");
    for i in 0..savings.len() {
        if savings[i] != 0 {
            println!("    {i:>4}    {}", savings[i]);
        }
    }

    (Solution::I32(solution_one), Solution::U32(0))
}

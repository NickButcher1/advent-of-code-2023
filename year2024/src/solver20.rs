use aoc::board::Board;
use aoc::int_board::IntBoard;
use aoc::solution::{Solution, Solutions};
use itertools::iproduct;
use std::collections::VecDeque;

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

    // First the unmodified board.
    let unmodified_cheapest_path = find_lowest_steps_for_board(&board, start, end);

    // Then try every possible cheat.
    let mut solution_one = 0;
    for (r, c) in iproduct!(1..(board.num_rows - 1), 1..(board.num_cols - 1)) {
        // Only remove a wall if it has an empty cell either side of it.
        if board.cells[r][c] == WALL
            && ((board.cells[r - 1][c] == EMPTY && board.cells[r + 1][c] == EMPTY)
                || (board.cells[r][c - 1] == EMPTY && board.cells[r][c + 1] == EMPTY))
        {
            let mut cheat_board = board.clone();
            cheat_board.cells[r][c] = EMPTY;
            let cheapest_path = find_lowest_steps_for_board(&cheat_board, start, end);
            let saving = unmodified_cheapest_path - cheapest_path;
            if saving >= 100 {
                solution_one += 1;
            }
        }
    }

    // PART TWO
    // Find the cheapest cost from the start to every empty cell.
    // Find the cheapest cost from the end to every empty cell.
    let mut cost_from_start = IntBoard::create_empty(board.num_rows, board.num_cols);
    let mut cost_from_end = IntBoard::create_empty(board.num_rows, board.num_cols);
    for (r, c) in iproduct!(1..(board.num_rows - 1), 1..(board.num_cols - 1)) {
        if board.cells[r][c] == EMPTY {
            cost_from_start.cells[r][c] = find_lowest_steps_for_board(&board, start, (r, c));
            cost_from_end.cells[r][c] = find_lowest_steps_for_board(&board, end, (r, c));
        }
    }

    // Visual inspection of the input shows that there is only a single path from start to end and
    // it visits every cell exactly once.
    //
    // Iterate over all possible pairs of cheat start points (must be on an empty cell) and cheat
    // end points (must also be on an empty cell, and withing a taxicab distance of 20 from the
    // cheat start point).
    //
    // The cheapest cost for this cheat is the sum of:
    // - start -> cheat start
    // - taxtcab distance of cheat start -> cheat end
    // - cheat end -> end.
    let mut solution_two = 0;
    for (cheat_start_r, cheat_start_c) in
        iproduct!(1..(board.num_rows - 1), 1..(board.num_cols - 1))
    {
        let unmodified_cheapest_cost = cost_from_start.cells[cheat_start_r][cheat_start_c]
            + cost_from_end.cells[cheat_start_r][cheat_start_c];
        if unmodified_cheapest_cost != 0 {
            for (cheat_end_r_delta, cheat_end_c_delta) in iproduct!(-20..=20, -20..=20) {
                let taxicab_distance =
                    (cheat_end_r_delta as i32).abs() + (cheat_end_c_delta as i32).abs();
                if (2..=20).contains(&taxicab_distance) {
                    let mut best_saving = 0;
                    let cheat_end_r: i32 = cheat_start_r as i32 + cheat_end_r_delta;
                    let cheat_end_c: i32 = cheat_start_c as i32 + cheat_end_c_delta;

                    if cheat_end_r > 0
                        && cheat_end_c > 0
                        && cheat_end_r < board.num_rows as i32
                        && cheat_end_c < board.num_rows as i32
                        && board.cells[cheat_end_r as usize][cheat_end_c as usize] == EMPTY
                    {
                        let modified_cheapest_cost = cost_from_start.cells[cheat_start_r]
                            [cheat_start_c]
                            + cost_from_end.cells[cheat_end_r as usize][cheat_end_c as usize]
                            + taxicab_distance;
                        let saving = unmodified_cheapest_cost - modified_cheapest_cost;
                        if saving > best_saving {
                            best_saving = saving;
                        }
                    }

                    if best_saving >= 100 {
                        solution_two += 1;
                    }
                }
            }
        }
    }

    (Solution::I32(solution_one), Solution::U32(solution_two))
}

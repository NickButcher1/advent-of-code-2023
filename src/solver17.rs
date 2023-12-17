use crate::board::Board;
use std::collections::HashMap;

#[derive(Clone, Debug, Eq, Hash, PartialEq, Copy)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

pub fn solve17(input: &[String]) -> (i128, i128) {
    let board: Board = Board::from_input(input);
    (solve(&board, 1, 3) as i128, solve(&board, 4, 10) as i128)
}

pub fn solve(board: &Board, min_blocks_in_a_row: usize, max_blocks_in_a_row: usize) -> u64 {
    let mut cheapest_path_to_reach_end_from: HashMap<(usize, usize, usize, Dir), (usize, usize)> =
        HashMap::new();

    // Cache lookups from (row, column, num in that direction, direction) to (num_moves, energy).
    cheapest_path_to_reach_end_from.insert((0, 0, 0, Dir::Down), (0, 0));
    cheapest_path_to_reach_end_from.insert((0, 0, 0, Dir::Right), (0, 0));

    let mut best_solution = 999999999;
    loop {
        for i in 1..=max_blocks_in_a_row {
            for dir in [Dir::Right, Dir::Down] {
                let test_key = (board.num_rows - 1, board.num_cols - 1, i, dir);
                if let Some(cheapest_path) = cheapest_path_to_reach_end_from.get(&test_key) {
                    if cheapest_path.1 < best_solution {
                        best_solution = cheapest_path.1;
                        println!("BEST: {best_solution}");
                    }
                }
            }
        }

        for r in 0..board.num_rows {
            for c in 0..board.num_cols {
                for dir in [Dir::Up, Dir::Down, Dir::Left, Dir::Right] {
                    for depth in 0..=max_blocks_in_a_row {
                        let key = (r, c, depth, dir);
                        if let Some(cheapest_path) = cheapest_path_to_reach_end_from.get(&key) {
                            let old_num_moves_in_direction = cheapest_path.0;
                            let old_energy = cheapest_path.1;

                            for (offset_r, offset_c, new_dir) in [
                                (0, 1, Dir::Right),
                                (0, -1, Dir::Left),
                                (1, 0, Dir::Down),
                                (-1, 0, Dir::Up),
                            ] {
                                let new_moves_in_direction = if new_dir == dir {
                                    old_num_moves_in_direction + 1
                                } else {
                                    // Change of direction is not allowed if:
                                    // - we haven't done the minimum allowed number of moves in that direction
                                    // - it goes back to where we just came from.
                                    if old_num_moves_in_direction < min_blocks_in_a_row
                                        || new_dir == Dir::Up && dir == Dir::Down
                                        || new_dir == Dir::Down && dir == Dir::Up
                                        || new_dir == Dir::Left && dir == Dir::Right
                                        || new_dir == Dir::Right && dir == Dir::Left
                                    {
                                        continue;
                                    }
                                    1
                                };

                                if new_moves_in_direction > max_blocks_in_a_row {
                                    continue;
                                }

                                let new_r: i32 = r as i32 + offset_r;
                                let new_c: i32 = c as i32 + offset_c;

                                // Can't go off the board.
                                if new_r == -1
                                    || new_r == board.num_rows as i32
                                    || new_c == -1
                                    || new_c == board.num_cols as i32
                                {
                                    continue;
                                }

                                let new_energy = old_energy
                                    + board.cells[new_r as usize][new_c as usize]
                                        .to_digit(10)
                                        .unwrap() as usize;

                                let new_key = (
                                    new_r as usize,
                                    new_c as usize,
                                    new_moves_in_direction,
                                    new_dir,
                                );

                                if let Some(existing_cheapest_path) =
                                    cheapest_path_to_reach_end_from.get(&new_key)
                                {
                                    let this_old_energy = existing_cheapest_path.1;
                                    if new_energy < this_old_energy {
                                        cheapest_path_to_reach_end_from
                                            .insert(new_key, (new_moves_in_direction, new_energy));
                                    }
                                } else {
                                    cheapest_path_to_reach_end_from
                                        .insert(new_key, (new_moves_in_direction, new_energy));
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    0 // TODO
}

use aoc::board::Board;
use aoc::dir::Dir;
use std::collections::HashMap;

#[derive(Clone, Eq, Hash, PartialEq, Debug)]
struct Key {
    r: usize,
    c: usize,
    dir: Dir,
    steps_in_dir: usize,
}

pub fn solve17(input: &[String]) -> (i128, i128) {
    let board: Board = Board::from_input(input);
    (solve(&board, 1, 3) as i128, solve(&board, 4, 10) as i128)
}

pub fn solve(board: &Board, min_blocks_in_a_row: usize, max_blocks_in_a_row: usize) -> usize {
    let mut cheapest_energy_to_reach_end_from: HashMap<Key, usize> = HashMap::new();

    // Cache lookups from (row, column, direction, num steps in that direction) to energy.
    cheapest_energy_to_reach_end_from.insert(
        Key {
            r: 0,
            c: 0,
            dir: Dir::Down,
            steps_in_dir: 0,
        },
        0,
    );
    cheapest_energy_to_reach_end_from.insert(
        Key {
            r: 0,
            c: 0,
            dir: Dir::Right,
            steps_in_dir: 0,
        },
        0,
    );

    let mut best_solution = 999999999;
    let mut loops_with_no_better_solution = 0;
    loop {
        for r in 0..board.num_rows {
            for c in 0..board.num_cols {
                for dir in [Dir::Up, Dir::Down, Dir::Left, Dir::Right] {
                    for old_steps_in_dir in 0..=max_blocks_in_a_row {
                        if let Some(cheapest_energy) = cheapest_energy_to_reach_end_from.get(&Key {
                            r,
                            c,
                            dir,
                            steps_in_dir: old_steps_in_dir,
                        }) {
                            let old_energy = *cheapest_energy;

                            for (offset_r, offset_c, new_dir) in [
                                (0, 1, Dir::Right),
                                (0, -1, Dir::Left),
                                (1, 0, Dir::Down),
                                (-1, 0, Dir::Up),
                            ] {
                                let new_steps_in_dir = if new_dir == dir {
                                    old_steps_in_dir + 1
                                } else {
                                    // Change of direction is not allowed if:
                                    // - we haven't done the minimum allowed number of moves in that direction
                                    // - it goes back to where we just came from.
                                    if old_steps_in_dir < min_blocks_in_a_row
                                        || new_dir == Dir::Up && dir == Dir::Down
                                        || new_dir == Dir::Down && dir == Dir::Up
                                        || new_dir == Dir::Left && dir == Dir::Right
                                        || new_dir == Dir::Right && dir == Dir::Left
                                    {
                                        continue;
                                    }
                                    1
                                };

                                if new_steps_in_dir > max_blocks_in_a_row {
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

                                let new_key = Key {
                                    r: new_r as usize,
                                    c: new_c as usize,
                                    steps_in_dir: new_steps_in_dir,
                                    dir: new_dir,
                                };

                                if let Some(existing_cheapest_energy) =
                                    cheapest_energy_to_reach_end_from.get(&new_key)
                                {
                                    let this_old_energy = *existing_cheapest_energy;
                                    if new_energy < this_old_energy {
                                        cheapest_energy_to_reach_end_from
                                            .insert(new_key, new_energy);
                                    }
                                } else {
                                    cheapest_energy_to_reach_end_from.insert(new_key, new_energy);
                                }
                            }
                        }
                    }
                }
            }
        }

        loops_with_no_better_solution += 1;
        for steps_in_dir in 1..=max_blocks_in_a_row {
            for dir in [Dir::Right, Dir::Down] {
                let test_key = Key {
                    r: board.num_rows - 1,
                    c: board.num_cols - 1,
                    dir,
                    steps_in_dir,
                };

                if let Some(cheapest_energy) = cheapest_energy_to_reach_end_from.get(&test_key) {
                    if *cheapest_energy < best_solution {
                        best_solution = *cheapest_energy;
                        loops_with_no_better_solution = 0;
                    }
                }
            }
        }
        if loops_with_no_better_solution == 10 {
            return best_solution;
        }
    }
}

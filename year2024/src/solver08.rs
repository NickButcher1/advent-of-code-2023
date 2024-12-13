use aoc::board::Board;
use itertools::{iproduct, Itertools};
use std::collections::HashMap;

const EMPTY: char = '.';
const ANTINODE: char = '#';

fn add_antinode(board: &mut Board, r: i32, c: i32) -> bool {
    if r >= 0 && c >= 0 && (r as usize) < board.num_rows && (c as usize) < board.num_cols {
        board.cells[r as usize][c as usize] = ANTINODE;
        true
    } else {
        false
    }
}
fn solve(board: &Board, antennas: &HashMap<char, Vec<(usize, usize)>>, is_part_two: bool) -> u64 {
    let mut antinode_board = Board::create_empty(board.num_rows, board.num_cols, EMPTY);

    for ch in antennas.keys() {
        let combinations = antennas.get(ch).unwrap().into_iter().combinations(2);
        for combo in combinations {
            let dr = combo[0].0.abs_diff(combo[1].0);
            let dc = combo[0].1.abs_diff(combo[1].1);

            // Always put the leftmost point first.
            let mut combo2 = combo.clone();
            combo2.sort_by_key(|coord| coord.1);

            let (mult_min, mult_max) = if is_part_two { (0, i32::MAX) } else { (1, 1) };

            for mult in mult_min..=mult_max {
                let antinode1_c = combo2[0].1 as i32 - mult * dc as i32;
                let antinode2_c = combo2[1].1 as i32 + mult * dc as i32;

                let (antinode1_r, antinode2_r) = if combo2[1].0 >= combo2[0].0 {
                    // Slope down.
                    (
                        combo2[0].0 as i32 - mult * dr as i32,
                        combo2[1].0 as i32 + mult * dr as i32,
                    )
                } else {
                    // Slope up.
                    (
                        combo2[0].0 as i32 + mult * dr as i32,
                        combo2[1].0 as i32 - mult * dr as i32,
                    )
                };
                let added1 = add_antinode(&mut antinode_board, antinode1_r, antinode1_c);
                let added2 = add_antinode(&mut antinode_board, antinode2_r, antinode2_c);
                if !added1 && !added2 {
                    // Off the edge of the board in both directions.
                    break;
                }
            }
        }
    }

    antinode_board.count(ANTINODE)
}

pub fn solve08(input: &[String]) -> (i128, i128) {
    let board = Board::from_input(input);

    // Make HashMap from char -> list of r,c tuple.
    let mut antennas: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    for (c, r) in iproduct!(0..board.num_cols, 0..board.num_rows) {
        if board.cells[r][c] != EMPTY {
            antennas
                .entry(board.cells[r][c])
                .and_modify(|v| v.push((r, c)))
                .or_insert(vec![(r, c)]);
        }
    }

    (
        solve(&board, &antennas, false) as i128,
        solve(&board, &antennas, true) as i128,
    )
}

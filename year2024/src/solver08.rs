use aoc::board::Board;
use itertools::{iproduct, Itertools};
use std::collections::HashMap;

const EMPTY: char = '.';
const ANTINODE: char = '#';

fn solve(board: &Board, antennas: &HashMap<char, Vec<(usize, usize)>>, is_part_two: bool) -> u64 {
    let mut antinode_board = Board::create_empty(board.num_rows, board.num_cols, EMPTY);

    for ch in antennas.keys() {
        let mut combinations = antennas.get(ch).unwrap().into_iter().combinations(2);
        // Coordinates are r,c.
        while let Some(combo) = combinations.next() {
            let dr = combo[0].0.abs_diff(combo[1].0);
            let dc = combo[0].1.abs_diff(combo[1].1);

            let mut combo2 = combo.clone();
            combo2.sort_by_key(|coord| coord.1);

            let (mult_min, mult_max) = if is_part_two { (0, 100) } else { (1, 1) };

            for mult in mult_min..=mult_max {
                let antinode1_c = combo2[0].1 as i32 - mult * dc as i32;
                let antinode2_c = combo2[1].1 as i32 + mult * dc as i32;

                let (antinode1_r, antinode2_r) = if combo2[1].0 >= combo2[0].0 {
                    (
                        combo2[0].0 as i32 - mult * dr as i32,
                        combo2[1].0 as i32 + mult * dr as i32,
                    )
                } else {
                    (
                        combo2[0].0 as i32 + mult * dr as i32,
                        combo2[1].0 as i32 - mult * dr as i32,
                    )
                };
                if antinode1_r >= 0
                    && antinode1_c >= 0
                    && (antinode1_r as usize) < board.num_rows
                    && (antinode1_c as usize) < board.num_cols
                {
                    antinode_board.cells[antinode1_r as usize][antinode1_c as usize] = ANTINODE;
                }
                if antinode2_r >= 0
                    && antinode2_c >= 0
                    && (antinode2_r as usize) < board.num_rows
                    && (antinode2_c as usize) < board.num_cols
                {
                    antinode_board.cells[antinode2_r as usize][antinode2_c as usize] = ANTINODE;
                }
            }
        }
    }

    antinode_board.count(ANTINODE)
}

pub fn solve08(input: &[String]) -> (i128, i128) {
    let board = Board::from_input(input);

    // Make HashMap from char -> list of x,y tuple.
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

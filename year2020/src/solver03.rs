use aoc::board::Board;
use aoc::solution::{Solution, Solutions};

const TREE: char = '#';

pub fn solve03(input: &[String]) -> Solutions {
    let board = Board::from_input(input);

    (
        Solution::I128(trees_for_slope(&board, 3, 1)),
        Solution::I128(
            trees_for_slope(&board, 1, 1)
                * trees_for_slope(&board, 3, 1)
                * trees_for_slope(&board, 5, 1)
                * trees_for_slope(&board, 7, 1)
                * trees_for_slope(&board, 1, 2),
        ),
    )
}

pub fn trees_for_slope(board: &Board, col_inc: usize, row_inc: usize) -> i128 {
    let mut r = 0;
    let mut c = 0;
    let mut num_trees = 0;

    loop {
        r += row_inc;
        if r >= board.num_rows {
            break;
        }

        c = (c + col_inc) % board.num_cols;
        if board.cells[r][c] == TREE {
            num_trees += 1;
        }
    }

    num_trees
}

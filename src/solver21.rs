use aoc::board::Board;
use std::collections::HashSet;

const ROCK: char = '#';
const START: char = 'S';
const EMPTY: char = '.';

const TARGET_STEPS_PART_1: usize = 64;
const TARGET_STEPS_PART_2: usize = 26_501_365;

fn solve_part_1(start_cell: (usize, usize), valid_moves: &[Vec<Vec<(usize, usize)>>]) -> i128 {
    let mut valid_cells: HashSet<(usize, usize)> = HashSet::new();
    valid_cells.insert(start_cell);

    for step in 1..=TARGET_STEPS_PART_1 {
        let mut new_valid_cells: HashSet<(usize, usize)> = HashSet::new();
        for (start_r, start_c) in valid_cells {
            for new_cell in &valid_moves[start_r][start_c] {
                new_valid_cells.insert(*new_cell);
            }
        }
        valid_cells = new_valid_cells;
    }
    valid_cells.len() as i128
}

fn solve_part_2(
    start_cell: (usize, usize),
    valid_moves: &[Vec<Vec<(usize, usize, isize, isize)>>],
) -> i128 {
    let mut valid_cells: HashSet<(usize, usize, isize, isize)> = HashSet::new();
    valid_cells.insert((start_cell.0, start_cell.0, 0, 0));

    println!("START: {start_cell:?}: valid_cells {valid_cells:?}");
    for step in 1..=TARGET_STEPS_PART_2 {
        let mut new_valid_cells: HashSet<(usize, usize, isize, isize)> = HashSet::new();
        for (start_r, start_c, start_big_r, start_big_c) in valid_cells {
            for new_cell in &valid_moves[start_r][start_c] {
                let modified_new_cell = (
                    new_cell.0,
                    new_cell.1,
                    new_cell.2 + start_big_r,
                    new_cell.3 + start_big_c,
                );
                new_valid_cells.insert(modified_new_cell);
            }
        }
        valid_cells = new_valid_cells;
        println!("STEP: {step}, {} valid cells", valid_cells.len());
    }
    valid_cells.len() as i128
}

pub fn solve21(input: &[String]) -> (i128, i128) {
    let mut board: Board = Board::from_input(input);
    let start_cell = board.find_and_replace(START, EMPTY);

    let mut valid_moves_part_1: Vec<Vec<Vec<(usize, usize)>>> =
        vec![vec![vec![]; board.num_cols]; board.num_rows];
    // Also track the board offset.
    let mut valid_moves_part_2: Vec<Vec<Vec<(usize, usize, isize, isize)>>> =
        vec![vec![vec![]; board.num_cols]; board.num_rows];

    for r in 0..board.num_rows {
        for c in 0..board.num_cols {
            if r != 0 && board.cells[r - 1][c] == EMPTY {
                valid_moves_part_1[r][c].push((r - 1, c));
                valid_moves_part_2[r][c].push((r - 1, c, 0, 0));
            }
            if r != board.num_rows - 1 && board.cells[r + 1][c] == EMPTY {
                valid_moves_part_1[r][c].push((r + 1, c));
                valid_moves_part_2[r][c].push((r + 1, c, 0, 0));
            }
            if c != 0 && board.cells[r][c - 1] == EMPTY {
                valid_moves_part_1[r][c].push((r, c - 1));
                valid_moves_part_2[r][c].push((r, c - 1, 0, 0));
            }
            if c != board.num_cols - 1 && board.cells[r][c + 1] == EMPTY {
                valid_moves_part_1[r][c].push((r, c + 1));
                valid_moves_part_2[r][c].push((r, c + 1, 0, 0));
            }

            // These also have a change of board "big R".
            if r == 0 && board.cells[board.num_rows - 1][c] == EMPTY {
                valid_moves_part_2[r][c].push((board.num_rows - 1, c, -1, 0));
            }
            if r == board.num_rows - 1 && board.cells[0][c] == EMPTY {
                valid_moves_part_2[r][c].push((0, c, 1, 0));
            }
            if c == 0 && board.cells[r][board.num_cols - 1] == EMPTY {
                valid_moves_part_2[r][c].push((r, board.num_cols - 1, 0, -1));
            }
            if c == board.num_cols - 1 && board.cells[r][0] == EMPTY {
                valid_moves_part_2[r][c].push((r, 0, 0, 1));
            }
        }
    }

    (
        solve_part_1(start_cell, &valid_moves_part_1),
        solve_part_2(start_cell, &valid_moves_part_2),
    )
}

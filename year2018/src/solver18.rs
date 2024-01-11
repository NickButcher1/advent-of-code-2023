use aoc::board::{Board, Cells};

const OPEN: char = '.';
const TREE: char = '|';
const LUMBERYARD: char = '#';
const BORDER: char = ' ';

const TARGET_MINUTES_PART_2: usize = 1_000_000_000;

pub fn solve18(input: &[String]) -> (i128, i128) {
    (solve_for_minutes(input, 10), solve_part_2(input))
}

pub fn solve_for_minutes(input: &[String], minutes: usize) -> i128 {
    let mut board = Board::from_input(input);
    board.add_border(BORDER);
    for _ in 0..minutes {
        step(&mut board);
    }
    board.count(TREE) as i128 * board.count(LUMBERYARD) as i128
}

pub fn solve_part_2(input: &[String]) -> i128 {
    let mut board = Board::from_input(input);
    let mut hashes: Vec<Vec<u8>> = vec![vec![]];

    board.add_border(BORDER);
    for i in 1..1000 {
        step(&mut board);
        let hash = md5::compute(format!("{board:?}")).to_ascii_lowercase();
        for j in 1..hashes.len() {
            if hashes[j] == hash {
                let cycle_start = j;
                let cycle_len = i - j;
                let x = cycle_start + ((TARGET_MINUTES_PART_2 - cycle_start) % cycle_len);
                return solve_for_minutes(input, x);
            }
        }
        hashes.push(hash);
    }
    unreachable!();
}

pub fn step(board: &mut Board) {
    let mut new_cells: Cells = vec![];

    for r in 0..=board.num_rows - 1 {
        let mut row_vec: Vec<char> = vec![];
        for c in 0..=board.num_cols - 1 {
            if r == 0 || r == board.num_rows - 1 || c == 0 || c == board.num_cols - 1 {
                row_vec.push(BORDER);
            } else {
                let neighbour_cells = [
                    board.cells[r - 1][c - 1],
                    board.cells[r - 1][c],
                    board.cells[r - 1][c + 1],
                    board.cells[r][c - 1],
                    board.cells[r][c + 1],
                    board.cells[r + 1][c - 1],
                    board.cells[r + 1][c],
                    board.cells[r + 1][c + 1],
                ];
                let new_char = match board.cells[r][c] {
                    OPEN => {
                        let num_trees_around =
                            neighbour_cells.iter().filter(|&&c| c == TREE).count();
                        if num_trees_around >= 3 {
                            TREE
                        } else {
                            OPEN
                        }
                    }
                    TREE => {
                        let num_lumberyards_around =
                            neighbour_cells.iter().filter(|&&c| c == LUMBERYARD).count();
                        if num_lumberyards_around >= 3 {
                            LUMBERYARD
                        } else {
                            TREE
                        }
                    }
                    LUMBERYARD => {
                        let num_trees_around =
                            neighbour_cells.iter().filter(|&&c| c == TREE).count();
                        let num_lumberyards_around =
                            neighbour_cells.iter().filter(|&&c| c == LUMBERYARD).count();
                        if num_lumberyards_around >= 1 && num_trees_around >= 1 {
                            LUMBERYARD
                        } else {
                            OPEN
                        }
                    }
                    _ => unreachable!(),
                };
                row_vec.push(new_char);
            }
        }
        new_cells.push(row_vec);
    }

    board.cells = new_cells;
}

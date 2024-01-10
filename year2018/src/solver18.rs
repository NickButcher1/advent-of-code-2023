use aoc::board::{Board, Cells};

const OPEN: char = '.';
const TREE: char = '|';
const LUMBERYARD: char = '#';
const BORDER: char = ' ';

pub fn solve18(input: &[String]) -> (i128, i128) {
    let mut board = Board::from_input(input);
    board.add_border(BORDER);
    board.print();
    for i in 0..10 {
        println!("STEP {i}");
        step(&mut board);
        board.print();
    }
    let num_trees = board.count(TREE);
    let num_lumberyards = board.count(LUMBERYARD);
    (num_trees as i128 * num_lumberyards as i128, 0)
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

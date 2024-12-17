use aoc::board::{Board, Cells};
use aoc::solution::{Solution, Solutions};

const FLOOR: char = '.';
const EMPTY: char = 'L';
const OCCUPIED: char = '#';

pub fn solve11(input: &[String]) -> Solutions {
    let mut board = Board::from_input(input);
    board.add_border(FLOOR);

    let mut last_hash = board.hash();
    loop {
        step(&mut board);
        let hash = board.hash();
        if hash == last_hash {
            break;
        }
        last_hash = hash;
    }
    (Solution::U64(board.count(OCCUPIED)), Solution::U64(0))
}

pub fn step(board: &mut Board) {
    let mut new_cells: Cells = vec![];

    for r in 0..=board.num_rows - 1 {
        let mut row_vec: Vec<char> = vec![];
        for c in 0..=board.num_cols - 1 {
            if r == 0 || r == board.num_rows - 1 || c == 0 || c == board.num_cols - 1 {
                row_vec.push(FLOOR);
            } else {
                let neighbour_cells = board.neighbour_cells(r, c);
                let new_char = match board.cells[r][c] {
                    FLOOR => FLOOR,
                    EMPTY => {
                        let num_occupied_around =
                            neighbour_cells.iter().filter(|&&c| c == OCCUPIED).count();
                        if num_occupied_around == 0 {
                            OCCUPIED
                        } else {
                            EMPTY
                        }
                    }
                    OCCUPIED => {
                        let num_occupied_around =
                            neighbour_cells.iter().filter(|&&c| c == OCCUPIED).count();
                        if num_occupied_around >= 4 {
                            EMPTY
                        } else {
                            OCCUPIED
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

use aoc::board::Board;
use std::collections::HashSet;

const START: char = 'S';
const EMPTY: char = '.';

const TARGET_STEPS_PART_1: usize = 64;
const TARGET_STEPS_PART_2: usize = 26_501_365;

type Cell = (usize, usize);
type Offset = (isize, isize);

fn solve_part_1(num_steps: usize, start_cell: Cell, valid_moves: &[Vec<Vec<Cell>>]) -> i128 {
    let mut valid_cells: HashSet<Cell> = HashSet::new();
    valid_cells.insert(start_cell);

    for _ in 1..=num_steps {
        let mut new_valid_cells: HashSet<Cell> = HashSet::new();
        for (start_r, start_c) in valid_cells {
            for new_cell in &valid_moves[start_r][start_c] {
                new_valid_cells.insert(*new_cell);
            }
        }
        valid_cells = new_valid_cells;
    }
    valid_cells.len() as i128
}

// This is far too slow to ever solve it, but it works on the sample input.
#[allow(dead_code)]
fn solve_part_2_brute_force(
    num_steps: usize,
    board: &Board,
    start_cell: Cell,
    valid_moves: &[Vec<Vec<(Cell, Offset)>>],
) -> i128 {
    // Start with a list of BIG_R/BIG_C for each cell in the board.
    let mut valid_cells: Vec<Vec<HashSet<Offset>>> =
        vec![vec![HashSet::new(); board.num_cols]; board.num_rows];
    valid_cells[start_cell.0][start_cell.1].insert((0, 0));

    for _ in 1..=num_steps {
        let mut new_valid_cells: Vec<Vec<HashSet<Offset>>> =
            vec![vec![HashSet::new(); board.num_cols]; board.num_rows];
        // For every cell on the board...
        for r in 0..board.num_rows {
            for c in 0..board.num_cols {
                // For every occupied cell...
                let hs = &valid_cells[r][c];
                if !hs.is_empty() {
                    // For all the valid moves,
                    for ((new_r, new_c), (big_r_offset, big_c_offset)) in &valid_moves[r][c] {
                        let new_hs = &mut new_valid_cells[*new_r][*new_c];

                        for old_big_board in hs {
                            new_hs.insert((
                                old_big_board.0 + big_r_offset,
                                old_big_board.1 + big_c_offset,
                            ));
                        }
                    }
                }
            }
        }
        valid_cells = new_valid_cells;
    }

    valid_cells
        .iter()
        .take(board.num_rows)
        .fold(0, |num_occupied_cells, valid_cell| {
            if valid_cell.is_empty() {
                num_occupied_cells
            } else {
                num_occupied_cells + valid_cell.len()
            }
        }) as i128
}

fn solve_part_2(board: &Board, start_cell: Cell, valid_moves: &[Vec<Vec<(Cell, Offset)>>]) -> i128 {
    // So the target number of steps 26_501_365 has remainder 65 when divided by the board width.
    // That can't be a coincidence!
    //
    // After much inspection of the output from part 1, I finally spotted there is a repeating pattern
    // of occupied cells.
    // - 0: first 65 steps.
    // - 1: 65 + 131 steps
    // - 2: 65 + 131*2 steps
    // - 3: 65 + 131*3 steps and so on.
    let _val_at_0 = solve_part_2_brute_force(65, board, start_cell, valid_moves);
    let _val_at_1 = solve_part_2_brute_force(65 + 131, board, start_cell, valid_moves);
    let _val_at_2 = solve_part_2_brute_force(65 + 131 * 2, board, start_cell, valid_moves);

    // This gives values 3799, 34047 and 94475. So for S=steps we have:
    //
    // s*a*a + s*b + c = Number of occupied cells
    //
    // And three examples is enough to solve for a, b and c.
    //
    // 65*a*a + 65*b +c = 3799
    // (65+131)*a*a + (65+131)*b +c = 34047
    // (65+131*2)*a*a + (65+131*2)*b +c = 94475

    // I plugged those values into an online quadratic equation solver which gave me:
    let a = 15090;
    let b = 15158;
    let c = 3799;

    // Now it is just a case of putting the actual number of steps into the equation.
    let steps = (TARGET_STEPS_PART_2 as i128 - 65) / 131;
    (a * steps * steps) + (b * steps) + c
}

pub fn solve21(input: &[String]) -> (i128, i128) {
    let mut board: Board = Board::from_input(input);
    let start_cell = board.find_and_replace(START, EMPTY);

    let mut valid_moves_part_1: Vec<Vec<Vec<Cell>>> =
        vec![vec![vec![]; board.num_cols]; board.num_rows];
    // Also track the board offset.
    let mut valid_moves_part_2: Vec<Vec<Vec<(Cell, Offset)>>> =
        vec![vec![vec![]; board.num_cols]; board.num_rows];

    for r in 0..board.num_rows {
        for c in 0..board.num_cols {
            if r != 0 && board.cells[r - 1][c] == EMPTY {
                valid_moves_part_1[r][c].push((r - 1, c));
                valid_moves_part_2[r][c].push(((r - 1, c), (0, 0)));
            }
            if r != board.num_rows - 1 && board.cells[r + 1][c] == EMPTY {
                valid_moves_part_1[r][c].push((r + 1, c));
                valid_moves_part_2[r][c].push(((r + 1, c), (0, 0)));
            }
            if c != 0 && board.cells[r][c - 1] == EMPTY {
                valid_moves_part_1[r][c].push((r, c - 1));
                valid_moves_part_2[r][c].push(((r, c - 1), (0, 0)));
            }
            if c != board.num_cols - 1 && board.cells[r][c + 1] == EMPTY {
                valid_moves_part_1[r][c].push((r, c + 1));
                valid_moves_part_2[r][c].push(((r, c + 1), (0, 0)));
            }

            // These also have a change of board "big R".
            if r == 0 && board.cells[board.num_rows - 1][c] == EMPTY {
                valid_moves_part_2[r][c].push(((board.num_rows - 1, c), (-1, 0)));
            }
            if r == board.num_rows - 1 && board.cells[0][c] == EMPTY {
                valid_moves_part_2[r][c].push(((0, c), (1, 0)));
            }
            if c == 0 && board.cells[r][board.num_cols - 1] == EMPTY {
                valid_moves_part_2[r][c].push(((r, board.num_cols - 1), (0, -1)));
            }
            if c == board.num_cols - 1 && board.cells[r][0] == EMPTY {
                valid_moves_part_2[r][c].push(((r, 0), (0, 1)));
            }
        }
    }

    (
        solve_part_1(TARGET_STEPS_PART_1, start_cell, &valid_moves_part_1),
        solve_part_2(&board, start_cell, &valid_moves_part_2),
    )
}

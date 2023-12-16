use crate::board::Board;

const EMPTY: char = '.';
const MOVABLE: char = 'O';

pub fn solve14(input: Vec<String>) -> (i128, i128) {
    let mut board: Board = Board::from_input(input);

    (
        solve_part_1(&mut board) as i128,
        solve_part_2(&mut board) as i128,
    )
}

fn tilt_north(board: &mut Board) {
    for r in 1..board.num_rows {
        for c in 0..board.num_cols {
            if board.cells[r][c] == MOVABLE {
                for r2 in (0..r).rev() {
                    if board.cells[r2][c] == EMPTY {
                        board.cells[r2][c] = MOVABLE;
                        board.cells[r2 + 1][c] = EMPTY;
                    } else {
                        break;
                    }
                }
            }
        }
    }
}

fn score_board(board: &Board) -> usize {
    let mut score = 0;

    for r in 0..board.num_rows {
        for c in 0..board.num_cols {
            if board.cells[r][c] == MOVABLE {
                score += board.num_rows - r;
            }
        }
    }

    score
}

fn one_cycle(board: &mut Board) {
    for _ in 0..4 {
        tilt_north(board);
        board.rotate_clockwise();
    }
}

fn solve_part_1(board: &mut Board) -> usize {
    tilt_north(board);
    score_board(board)
}

// There is a repeating cycle but it doesn't begin from the first board. Loop over last_board_in_cycle and for
// each cycle, record the new board and then compare it against all previous boards. Stop when we
// find a match because we now know the cycle length and the offset of the start of the cycle.
fn solve_part_2(board: &mut Board) -> usize {
    let mut seen_boards: Vec<Board> = vec![board.clone()];

    let mut first_board_in_cycle = 0;
    let mut last_board_in_cycle = 0;
    while first_board_in_cycle == 0 {
        last_board_in_cycle += 1;
        one_cycle(board);

        for i in 0..seen_boards.len() {
            if *board == seen_boards[i] {
                first_board_in_cycle = i;
            }
        }

        seen_boards.push(board.clone());
    }

    // The repeating cycle doesn't necessarily finish on the target, so move the start point
    // until it is an integer multiple of cycles from the target.
    let cycle_len = last_board_in_cycle - first_board_in_cycle;

    for i in first_board_in_cycle..last_board_in_cycle {
        if (1_000_000_000 - i) % cycle_len == 0 {
            return score_board(&seen_boards[i]);
        }
    }

    unreachable!();
}

const EMPTY: usize = 0;
const MOVABLE: usize = 1;
const FIXED: usize = 2;

type Board = Vec<Vec<usize>>;

pub fn solve14(input: Vec<String>) -> (i128, i128) {
    let mut board: Board = vec![];

    for line in &input {
        let chars: Vec<char> = line.chars().collect();
        let mut row_vec: Vec<usize> = vec![];

        for c in chars {
            let c_int = match c {
                'O' => MOVABLE,
                '#' => FIXED,
                '.' => EMPTY,
                _ => panic!("ERROR!"),
            };
            row_vec.push(c_int);
        }

        board.push(row_vec);
    }

    (
        solve_part_1(&mut board) as i128,
        solve_part_2(&mut board) as i128,
    )
}

// No doubt I could refactor these four into a single tilt function, but I've done enough for today.
fn tilt_north(board: &mut Board) {
    for r in 1..board.len() {
        for c in 0..board[0].len() {
            if board[r][c] == MOVABLE {
                for r2 in (0..r).rev() {
                    if board[r2][c] == EMPTY {
                        board[r2][c] = MOVABLE;
                        board[r2 + 1][c] = EMPTY;
                    } else {
                        break;
                    }
                }
            }
        }
    }
}

fn tilt_south(board: &mut Board) {
    board.reverse();
    tilt_north(board);
    board.reverse();
}

fn tilt_west(board: &mut Board) {
    for c in 1..board[0].len() {
        for r in 0..board.len() {
            if board[r][c] == MOVABLE {
                for c2 in (0..c).rev() {
                    if board[r][c2] == EMPTY {
                        board[r][c2] = MOVABLE;
                        board[r][c2 + 1] = EMPTY;
                    } else {
                        break;
                    }
                }
            }
        }
    }
}

fn tilt_east(board: &mut Board) {
    for r in 0..board.len() {
        board[r].reverse();
    }
    tilt_west(board);
    for r in 0..board.len() {
        board[r].reverse();
    }
}

fn score_board(board: &Board) -> usize {
    let mut score = 0;

    for r in 0..board.len() {
        for c in 0..board[0].len() {
            if board[r][c] == MOVABLE {
                score += board.len() - r;
            }
        }
    }

    score
}

fn one_cycle(board: &mut Board) {
    tilt_north(board);
    tilt_west(board);
    tilt_south(board);
    tilt_east(board);
}

fn are_boards_equal(board_1: &Board, board_2: &Board) -> bool {
    for r in 0..board_1.len() {
        if board_1[r] != board_2[r] {
            return false;
        }
    }
    true
}

fn solve_part_1(board: &mut Board) -> usize {
    tilt_north(board);
    score_board(&board)
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
            if are_boards_equal(&seen_boards[i], board) {
                first_board_in_cycle = i;
            }
        }

        seen_boards.push(board.clone());
    }

    // The repeating cycle doesn't necessarily finish on the target, so move the start point
    // until it is an integer multiple of cycles from the target.
    let cycle_len = last_board_in_cycle - first_board_in_cycle;

    for i in first_board_in_cycle..last_board_in_cycle {
        if (1000000000 - i) % cycle_len == 0 {
            return score_board(&seen_boards[i]);
        }
    }

    unreachable!();
}

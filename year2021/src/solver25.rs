use string_builder::Builder;

type Board = Vec<Vec<u8>>;

const EMPTY: u8 = 0;
const CUC_RIGHT: u8 = 1;
const CUC_DOWN: u8 = 2;

#[allow(dead_code)]
fn print_board(board: &Board, num_rows: usize, num_cols: usize) {
    let mut builder = Builder::default();

    for r in 0..num_rows {
        builder.append('\n');
        for c in 0..num_cols {
            let ch = match board[r][c] {
                EMPTY => '.',
                CUC_RIGHT => '>',
                CUC_DOWN => 'v',
                _ => panic!("ERROR!"),
            };
            builder.append(ch);
        }
    }

    println!("{}", builder.string().unwrap());
}

fn move_one_step(board: &mut Board, num_rows: usize, num_cols: usize) -> bool {
    let mut num_moved = 0;

    // Move CUC_RIGHT right if possible, wrap at the end.
    for r in 0..num_rows {
        let mut moveit_r: Vec<bool> = vec![false; num_cols];
        for c in (0..num_cols).rev() {
            if c == (num_cols - 1) {
                if board[r][c] == CUC_RIGHT && board[r][0] == EMPTY {
                    num_moved += 1;
                    moveit_r[c] = true;
                }
            } else if board[r][c] == CUC_RIGHT && board[r][c + 1] == EMPTY {
                num_moved += 1;
                moveit_r[c] = true;
            }
        }

        for c in (0..num_cols).rev() {
            if moveit_r[c] {
                board[r][c] = EMPTY;
            }
        }

        for c in (0..num_cols).rev() {
            if moveit_r[c] {
                if c == (num_cols - 1) {
                    board[r][0] = CUC_RIGHT;
                } else {
                    board[r][c + 1] = CUC_RIGHT;
                }
            }
        }
    }

    // Move CUC_DOWN down if possible, wrap at the end.

    for c in 0..num_cols {
        let mut moveit_c: Vec<bool> = vec![false; board.len()];
        for r in (0..num_rows).rev() {
            if r == (num_rows - 1) {
                if board[r][c] == CUC_DOWN && board[0][c] == EMPTY {
                    num_moved += 1;
                    moveit_c[r] = true;
                }
            } else if board[r][c] == CUC_DOWN && board[r + 1][c] == EMPTY {
                num_moved += 1;
                moveit_c[r] = true;
            }
        }

        for r in (0..num_rows).rev() {
            if moveit_c[r] {
                board[r][c] = EMPTY;
            }
        }

        for r in (0..num_rows).rev() {
            if moveit_c[r] {
                if r == (num_rows - 1) {
                    board[0][c] = CUC_DOWN;
                } else {
                    board[r + 1][c] = CUC_DOWN;
                }
            }
        }
    }

    num_moved == 0
}

pub fn solve25(input: &[String]) -> (i128, i128) {
    let num_rows = input.len();
    let num_cols = input[0].len();

    let mut board: Board = vec![vec![]; num_rows];
    for r in 0..num_rows {
        let chars: Vec<char> = input[r].chars().collect();
        for c in chars {
            let value = match c {
                '.' => EMPTY,
                '>' => CUC_RIGHT,
                'v' => CUC_DOWN,
                _ => panic!("ERROR!"),
            };
            board[r].push(value);
        }
    }

    let mut steps = 0;
    loop {
        steps += 1;
        // print_board(&board, num_rows, num_cols);
        if move_one_step(&mut board, num_rows, num_cols) {
            println!("Solved {}", steps);
            break;
        }
    }

    (steps, 0)
}

const ASH: usize = 0;
const ROCK: usize = 1;

type Board = Vec<Vec<usize>>;

pub fn solve13(input: Vec<String>) -> (i128, i128) {
    let mut part_1_total: u64 = 0;
    let mut part_2_total: u64 = 0;

    let mut board_a: Board = vec![];

    for line in &input {
        if line.len() == 0 {
            let (score1, _) = score_board_1(&board_a, true);
            part_1_total += score1;
            let (score1, _) = score_board_1(&flip_board(&board_a), false);
            part_1_total += score1;
            part_2_total += score_board_part_2(&board_a, true);
            part_2_total += score_board_part_2(&flip_board(&board_a), false);
            board_a.clear();
        } else {
            let chars: Vec<char> = line.chars().collect();
            let mut row_vec: Vec<usize> = vec![];

            for c in chars {
                let c_int = if c == '.' { ASH } else { ROCK };
                row_vec.push(c_int);
            }

            board_a.push(row_vec);
        }
    }
    // Get the last board.
    let (score1, _) = score_board_1(&board_a, true);
    part_1_total += score1;
    let (score1, _) = score_board_1(&flip_board(&board_a), false);
    part_1_total += score1;
    part_2_total += score_board_part_2(&board_a, true);
    part_2_total += score_board_part_2(&flip_board(&board_a), false);

    (part_1_total as i128, part_2_total as i128)
}

fn flip_board(board: &Board) -> Board {
    let mut flip_board: Board = vec![];

    for c in 0..board[0].len() {
        let mut row_vec: Vec<usize> = vec![];
        for r in 0..board.len() {
            row_vec.push(board[r][c]);
        }
        flip_board.push(row_vec);
    }

    flip_board
}
fn score_board_part_2(board: &Board, score_rows: bool) -> u64 {
    let (part_1_score, part_1_row) = score_board_1(board, score_rows);
    // Try all possible flips until we find a new line of reflection.
    let num_rows = board.len();
    let num_cols = board[0].len();
    for r in 0..num_rows {
        for c in 0..num_cols {
            let mut new_board = board.clone().to_owned();
            if new_board[r][c] == ASH {
                new_board[r][c] = ROCK;
            } else {
                new_board[r][c] = ASH;
            }

            let new_score = score_board_2(&new_board, score_rows, part_1_row);
            if new_score != 0 && new_score != part_1_score {
                return new_score;
            }
        }
    }

    0
}

fn score_board_2(board: &Board, score_rows: bool, part_1_row: usize) -> u64 {
    let num_rows = board.len();

    // Assume the line of symmetry is never on the edge of the board.
    let mut score: u64 = 0;
    for r in 1..num_rows {
        // Between r and (r+1) is the line of symmetry.
        let mut is_symmetry = true;
        for d in 0..num_rows {
            let top_row: i32 = r as i32 - d as i32 - 1;
            let bottom_row: i32 = r as i32 + d as i32;
            // println!("        TRY DIFF {} COMP ROWS {} v {}", d, top_row, bottom_row);
            // Check if reached edge of board.
            if top_row < 0 {
                break;
            }
            if bottom_row == num_rows as i32 {
                break;
            }
            // Check if these two rows are symmetric.
            if board[top_row as usize] != board[bottom_row as usize] {
                is_symmetry = false;
                break;
            }
        }

        if is_symmetry {
            if part_1_row != r {
                score += if score_rows { 100 * r as u64 } else { r as u64 };
            }
        }
    }
    score
}

fn score_board_1(board: &Board, score_rows: bool) -> (u64, usize) {
    let num_rows = board.len();
    // Assume the line of symmetry is never on the edge of the board.
    let mut score: u64 = 0;
    for r in 1..num_rows {
        // Between r and (r+1) is the line of symmetry.
        let mut is_symmetry = true;
        for d in 0..num_rows {
            let top_row: i32 = r as i32 - d as i32 - 1;
            let bottom_row: i32 = r as i32 + d as i32;
            // Check if reached edge of board.
            if top_row < 0 {
                break;
            }
            if bottom_row == num_rows as i32 {
                break;
            }
            // Check if these two rows are symmetric.
            if board[top_row as usize] != board[bottom_row as usize] {
                is_symmetry = false;
                break;
            }
        }

        if is_symmetry {
            score += if score_rows { 100 * r as u64 } else { r as u64 };
            return (score, r);
        }
    }
    (0, 0)
}

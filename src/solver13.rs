const ASH: usize = 0;
const ROCK: usize = 1;

type Board = Vec<Vec<usize>>;

pub fn solve13(input: Vec<String>) -> (i128, i128) {
    let mut part_1_total: usize = 0;
    let mut part_2_total: usize = 0;

    let mut board: Board = vec![];

    for line in &input {
        if line.is_empty() {
            part_1_total +=
                score_board_part_1(&board, true) + score_board_part_1(&flip_board(&board), false);
            part_2_total +=
                score_board_part_2(&board, true) + score_board_part_2(&flip_board(&board), false);
            board.clear();
        } else {
            let chars: Vec<char> = line.chars().collect();
            let mut row_vec: Vec<usize> = vec![];

            for c in chars {
                let c_int = if c == '.' { ASH } else { ROCK };
                row_vec.push(c_int);
            }

            board.push(row_vec);
        }
    }

    // Get the last board.
    part_1_total +=
        score_board_part_1(&board, true) + score_board_part_1(&flip_board(&board), false);
    part_2_total +=
        score_board_part_2(&board, true) + score_board_part_2(&flip_board(&board), false);

    (part_1_total as i128, part_2_total as i128)
}

// Flip the board on the diagonal axis. We can then use the same logic to solve for columns instead of rows.
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

fn score_board_part_1(board: &Board, is_flipped: bool) -> usize {
    let (score, _) = score_board(board, is_flipped, -1);
    score
}

// For part 2, try flipping every bit on the board in turn and calculating the score. Stop as soon
// as we find a score that differs from part 1.
fn score_board_part_2(board: &Board, is_flipped: bool) -> usize {
    let (part_1_score, part_1_row) = score_board(board, is_flipped, -1);
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

            let (new_score, _) = score_board(&new_board, is_flipped, part_1_row as i32);
            if new_score != 0 && new_score != part_1_score {
                return new_score;
            }
        }
    }

    0
}

fn score_board(board: &Board, is_flipped: bool, ignore_row: i32) -> (usize, usize) {
    let num_rows = board.len();

    for r in 1..num_rows {
        // Test if there is a line of symmetry between row r and row r+1. Do this by expanding outwards
        // towards the edge of the board, comparing pairs of rows.
        // - If we find a non-identical pair then stop because there is no line of symmetry.
        // - If we reach the edge of the board then we have found a line of symmetry.
        let mut is_symmetry = true;
        for d in 0..num_rows {
            let top_row: i32 = r as i32 - d as i32 - 1;
            let bottom_row: i32 = r as i32 + d as i32;
            if top_row < 0 || bottom_row == num_rows as i32 {
                break;
            }
            if board[top_row as usize] != board[bottom_row as usize] {
                is_symmetry = false;
                break;
            }
        }

        if is_symmetry && (ignore_row == -1 || ignore_row != r as i32) {
            let score = if is_flipped { 100 * r } else { r };
            return (score, r);
        }
    }
    (0, 0)
}

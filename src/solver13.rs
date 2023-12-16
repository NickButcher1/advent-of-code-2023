use crate::board::Board;

const ASH: char = '.';
const ROCK: char = '#';

pub fn solve13(input: &[String]) -> (i128, i128) {
    let mut part_1_total: usize = 0;
    let mut part_2_total: usize = 0;

    let boards: Vec<Board> = Board::from_input_multiple(input);

    for mut board in boards {
        part_1_total +=
            score_board_part_1(&board, true) + score_board_part_1(board.clone().flip(), false);
        part_2_total +=
            score_board_part_2(&mut board, true) + score_board_part_2(board.clone().flip(), false);
    }

    (part_1_total as i128, part_2_total as i128)
}

fn score_board_part_1(board: &Board, is_flipped: bool) -> usize {
    let (score, _) = score_board(board, is_flipped, -1);
    score
}

// For part 2, try flipping every bit on the board in turn and calculating the score. Stop as soon
// as we find a score that differs from part 1.
fn score_board_part_2(board: &mut Board, is_flipped: bool) -> usize {
    let (part_1_score, part_1_row) = score_board(board, is_flipped, -1);
    // Try all possible flips until we find a new line of reflection.
    for r in 0..board.num_rows {
        for c in 0..board.num_cols {
            let (old_value, new_value) = if board.cells[r][c] == ASH {
                (ASH, ROCK)
            } else {
                (ROCK, ASH)
            };
            board.cells[r][c] = new_value;

            let (new_score, _) = score_board(board, is_flipped, part_1_row as i32);
            if new_score != 0 && new_score != part_1_score {
                return new_score;
            }
            board.cells[r][c] = old_value;
        }
    }

    0
}

fn score_board(board: &Board, is_flipped: bool, ignore_row: i32) -> (usize, usize) {
    for r in 1..board.num_rows {
        // Test if there is a line of symmetry between row r and row r+1. Do this by expanding outwards
        // towards the edge of the board, comparing pairs of rows.
        // - If we find a non-identical pair then stop because there is no line of symmetry.
        // - If we reach the edge of the board then we have found a line of symmetry.
        let mut is_symmetry = true;
        for d in 0..board.num_rows {
            let top_row: i32 = r as i32 - d as i32 - 1;
            let bottom_row: i32 = r as i32 + d as i32;
            if top_row < 0 || bottom_row == board.num_rows as i32 {
                break;
            }
            if board.cells[top_row as usize] != board.cells[bottom_row as usize] {
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

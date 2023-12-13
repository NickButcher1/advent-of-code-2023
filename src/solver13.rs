const ASH: usize  = 0;
const ROCK: usize = 1;

type Board = Vec<Vec<usize>>;

pub fn solve13(input: Vec<String>) -> (i128, i128) {
    let mut part_1_total: u64 = 0;
    let mut part_2_total: u64 = 0;

    let mut board_a: Board = vec![];

    for line in &input {
        if line.len() == 0 {
            println!("RESET");
            part_1_total += score_board(&board_a, true);
            part_1_total += score_board(&flip_board(&board_a), false);
            board_a.clear();
        } else {
            println!("Read line: {}", line);
            let chars: Vec<char> = line.chars().collect();
            let mut row_vec: Vec<usize> = vec![];

            for c in chars {
                let c_int = if c == '.' { ASH } else { ROCK };
                row_vec.push(c_int);
            }

            board_a.push(row_vec);
            // println!("DBG: {:?}", board_a);
        }
    }
    // Get the last board.
    part_1_total += score_board(&board_a, true);
    part_1_total += score_board(&flip_board(&board_a), false);

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

    return flip_board;
}
fn score_board(board: &Board, score_rows: bool) -> u64 {
    println!("SCORE BOARD");
    let num_rows = board.len();
    let num_cols = board[0].len();
    println!("({},{})  {:?}", num_rows, num_cols, board);

    // Assume the line of symmetry is never on the edge of the board.
    let mut score: u64 = 0;
    for r in 1..num_rows {
        // Between r and (r+1) is the line of symmetry.
        println!("    TRY ROW {}", r);
        let mut is_symmetry = true;
        for d in 0..num_rows {

            let top_row: i32 = r as i32 - d as i32 - 1;
            let bottom_row: i32 = r as i32 + d as i32;
            println!("        TRY DIFF {} COMP ROWS {} v {}", d, top_row, bottom_row);
            // Check if reached edge of board.
            if top_row < 0 {
                println!("        BREAK OFF TOP");
                break;
            }
            if bottom_row == num_rows as i32 {
                println!("        BREAK OFF BOTTOM");
                break;
            }
            // Check if these two rows are symmetric.
            if board[top_row as usize] != board[bottom_row as usize] {
                println!("        BREAK NO SYMMETRY");
                is_symmetry = false;
                break;
            }
        }

        // Wrong: 18341

        if is_symmetry {
            println!("SYMMETRY AT {} IS_FLIP = {}", r, !score_rows);
            score += if score_rows {
                100 * r as u64
            } else {
                r as u64
            };
        }
    }
    score
}
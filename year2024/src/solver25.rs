use aoc::board::Board;
use aoc::solution::{Solution, Solutions};

const FILLED: char = '#';
const EMPTY: char = '.';

pub fn solve25(input: &[String]) -> Solutions {
    let boards = Board::from_input_multiple(input);
    let mut locks: Vec<[i32; 5]> = vec![];
    let mut keys: Vec<[i32; 5]> = vec![];

    for board in boards {
        if board.cells[0][0] == FILLED {
            // Lock: row of first EMPTY - 1.
            let mut lock: [i32; 5] = [0; 5];
            for c in 0..board.num_cols {
                let mut r = 0;
                while board.cells[r][c] == FILLED {
                    r += 1;
                }
                lock[c] = r as i32 - 1;
            }
            locks.push(lock);
        } else {
            // Key: 7 - 1 - row of first FILLED.
            let mut key: [i32; 5] = [0; 5];
            for c in 0..board.num_cols {
                let mut r = 0;
                while board.cells[r][c] == EMPTY {
                    r += 1;
                }
                key[c] = 7 - 1 - r as i32;
            }
            keys.push(key);
        }
    }

    let mut solution_one = 0;
    for lock in &locks {
        for key in &keys {
            let is_fit = ((lock[0] + key[0]) < 6)
                && ((lock[1] + key[1]) < 6)
                && ((lock[2] + key[2]) < 6)
                && ((lock[3] + key[3]) < 6)
                && ((lock[4] + key[4]) < 6);
            if is_fit {
                solution_one += 1;
            }
        }
    }

    (Solution::U32(solution_one), Solution::U32(0))
}

use aoc::board::Board;
use itertools::iproduct;
use regex::Regex;

const SAND: char = '.';
const CLAY: char = '#';
const SOURCE: char = '+';
const FALLING_WATER: char = '|';
const STEADY_WATER: char = '~';

pub fn solve17(input: &[String]) -> (i128, i128) {
    let re_x_first = Regex::new(r"^x=(\d+), y=(\d+)..(\d+)$").unwrap();
    let re_y_first = Regex::new(r"^y=(\d+), x=(\d+)..(\d+)$").unwrap();

    let mut instructions = vec![];
    let mut max_row = 0;
    let mut min_row = usize::MAX;
    input.iter().for_each(|line| {
        let instruction = if re_x_first.is_match(line) {
            let captures = re_x_first.captures(line).unwrap();
            (
                captures.get(1).unwrap().as_str().parse::<usize>().unwrap(),
                captures.get(1).unwrap().as_str().parse::<usize>().unwrap(),
                captures.get(2).unwrap().as_str().parse::<usize>().unwrap(),
                captures.get(3).unwrap().as_str().parse::<usize>().unwrap(),
            )
        } else if re_y_first.is_match(line) {
            let captures = re_y_first.captures(line).unwrap();
            (
                captures.get(2).unwrap().as_str().parse::<usize>().unwrap(),
                captures.get(3).unwrap().as_str().parse::<usize>().unwrap(),
                captures.get(1).unwrap().as_str().parse::<usize>().unwrap(),
                captures.get(1).unwrap().as_str().parse::<usize>().unwrap(),
            )
        } else {
            panic!("Invalid input");
        };
        if instruction.3 > max_row {
            max_row = instruction.3;
        }
        if instruction.2 < min_row {
            min_row = instruction.2;
        }
        instructions.push(instruction);
    });

    let mut board = Board::create_empty(max_row + 1 - min_row, 1000, SAND);

    for (c_min, c_max, r_min, r_max) in instructions {
        board.cells[0][500] = SOURCE;
        for (c, r) in iproduct!(c_min..=c_max, r_min..=r_max) {
            board.cells[r - min_row][c] = CLAY;
        }
    }

    expand_source_down(&mut board, 0, 500);

    let solution_one = board.count(STEADY_WATER) + board.count(FALLING_WATER);

    (solution_one as i128, 0 as i128)
}

fn expand_source_down(board: &mut Board, r: usize, c: usize) {
    if r == (board.num_rows - 1) {
        return;
    }

    match board.cells[r + 1][c] {
        SAND => {
            board.cells[r + 1][c] = FALLING_WATER;
            expand_source_down(board, r + 1, c);
        }
        CLAY => {
            expand_source_sideways(board, r, c);
        }
        STEADY_WATER => {
            expand_source_sideways(board, r, c);
        }
        _ => {}
    }
}

fn expand_source_sideways(board: &mut Board, r: usize, c: usize) {
    if board.cells[r][c] == STEADY_WATER {
        // Another stream has already processed this cell.
        return;
    }
    let mut left_stop = usize::MAX;
    let mut right_stop = usize::MAX;
    let mut left_hole = usize::MAX;
    let mut right_hole = usize::MAX;
    for c_left in (0..c).rev() {
        if board.cells[r][c_left] == CLAY {
            left_stop = c_left;
            break;
        } else if board.cells[r + 1][c_left] == SAND {
            left_hole = c_left;
            break;
        }
    }
    for c_right in (c + 1)..=(board.num_cols - 1) {
        if board.cells[r][c_right] == CLAY {
            right_stop = c_right;
            break;
        } else if board.cells[r + 1][c_right] == SAND {
            right_hole = c_right;
            break;
        }
    }

    if left_stop != usize::MAX && right_stop != usize::MAX {
        for c_fill in (left_stop + 1)..right_stop {
            board.cells[r][c_fill] = STEADY_WATER;
        }
        // Go up a level.
        expand_source_sideways(board, r - 1, c);
    } else {
        // Expand sideways then add one or two new downward streams.
        if left_stop == usize::MAX {
            left_stop = left_hole - 1;
        }
        if right_stop == usize::MAX {
            right_stop = right_hole + 1;
        }
        for c_fill in (left_stop + 1)..right_stop {
            board.cells[r][c_fill] = FALLING_WATER;
        }

        if left_hole != usize::MAX {
            expand_source_down(board, r, left_hole);
        }
        if right_hole != usize::MAX {
            expand_source_down(board, r, right_hole);
        }
    }
}

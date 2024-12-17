use aoc::board::Board;
use aoc::solution::{Solution, Solutions};
use itertools::iproduct;

const ROBOT: char = '@';
const SINGLE_BOX: char = 'O';
const EMPTY: char = '.';
const WALL: char = '#';
const LEFT_BOX: char = '[';
const RIGHT_BOX: char = ']';

fn sum_box_distances(board: &Board) -> usize {
    let mut sum = 0;
    for (c, r) in iproduct!(0..board.num_cols, 0..board.num_rows) {
        if board.cells[r][c] == SINGLE_BOX || board.cells[r][c] == LEFT_BOX {
            sum += 100 * r + c;
        }
    }
    sum
}

fn move_robot(board: &mut Board, robot_r: &mut usize, robot_c: &mut usize, dr: i32, dc: i32) {
    board.cells[*robot_r][*robot_c] = EMPTY;
    board.cells[(*robot_r as i32 + dr) as usize][(*robot_c as i32 + dc) as usize] = ROBOT;
    *robot_r = ((*robot_r as i32) + dr) as usize;
    *robot_c = ((*robot_c as i32) + dc) as usize;
}

fn move_single_box(board: &mut Board, robot_r: &mut usize, robot_c: &mut usize, dr: i32, dc: i32) {
    // Find first non-box. If a wall, no-op. If empty, move all the boxes.
    let mut try_r = (*robot_r as i32 + dr) as usize;
    let mut try_c = (*robot_c as i32 + dc) as usize;
    loop {
        if board.cells[try_r][try_c] == EMPTY {
            // No need to actually shuffle boxes, just make the empty cell a box and move the robot over
            // the first box.
            board.cells[try_r][try_c] = SINGLE_BOX;
            move_robot(board, robot_r, robot_c, dr, dc);
            break;
        } else if board.cells[try_r][try_c] == WALL {
            break;
        } else if board.cells[try_r][try_c] == SINGLE_BOX {
            try_r = (try_r as i32 + dr) as usize;
            try_c = (try_c as i32 + dc) as usize;
        }
    }
}

fn move_double_box_sideways(board: &mut Board, robot_r: &mut usize, robot_c: &mut usize, dc: i32) {
    // Find the first non-box. If a wall, no-op. If empty, shuffle all the boxes.
    let mut try_c = (*robot_c as i32 + dc) as usize;

    let (left_check, right_check) = if dc == 1 {
        (LEFT_BOX, RIGHT_BOX)
    } else {
        (RIGHT_BOX, LEFT_BOX)
    };

    loop {
        if board.cells[*robot_r][try_c] == EMPTY {
            // Shuffle the boxes.
            try_c = (try_c as i32 - dc) as usize;
            while board.cells[*robot_r][try_c] != ROBOT {
                board.cells[*robot_r][(try_c as i32 + dc) as usize] = right_check;
                board.cells[*robot_r][try_c] = left_check;
                try_c = ((try_c as i32) - 2 * dc) as usize;
            }
            move_robot(board, robot_r, robot_c, 0, dc);
            break;
        } else if board.cells[*robot_r][try_c] == WALL {
            break;
        } else if board.cells[*robot_r][try_c] == left_check
            && board.cells[*robot_r][(try_c as i32 + dc) as usize] == right_check
        {
            try_c = ((try_c as i32) + 2 * dc) as usize;
        }
    }
}

fn can_move_into(board: &Board, r: usize, dr: i32, c_left: usize) -> bool {
    if board.cells[r][c_left] == EMPTY && board.cells[r][c_left + 1] == EMPTY {
        true
    } else if board.cells[r][c_left] == WALL || board.cells[r][c_left + 1] == WALL {
        false
    } else if board.cells[r][c_left] == LEFT_BOX && board.cells[r][c_left + 1] == RIGHT_BOX {
        can_move_into(board, (r as i32 + dr) as usize, dr, c_left)
    } else {
        let can_move_l = if board.cells[r][c_left] == RIGHT_BOX {
            can_move_into(board, (r as i32 + dr) as usize, dr, c_left - 1)
        } else {
            true
        };
        let can_move_r = if board.cells[r][c_left + 1] == LEFT_BOX {
            can_move_into(board, (r as i32 + dr) as usize, dr, c_left + 1)
        } else {
            true
        };
        can_move_l && can_move_r
    }
}

fn do_move_first(board: &mut Board, r: usize, dr: i32, c_left: usize) -> bool {
    let mut stack: Vec<(usize, usize)> = vec![];
    do_move(true, &mut stack, board, r, dr, c_left);

    // Sort the track by row.
    if dr == 1 {
        stack.sort_by(|a, b| a.0.cmp(&b.0).reverse());
    } else {
        stack.sort_by(|a, b| a.0.cmp(&b.0));
    }

    for (r, c_left) in &stack {
        board.cells[*r][*c_left] = LEFT_BOX;
        board.cells[*r][*c_left + 1] = RIGHT_BOX;
        board.cells[(*r as i32 - dr) as usize][*c_left] = EMPTY;
        board.cells[(*r as i32 - dr) as usize][*c_left + 1] = EMPTY;
    }

    !stack.is_empty()
}

fn do_move(
    is_first: bool,
    stack: &mut Vec<(usize, usize)>,
    board: &mut Board,
    r: usize,
    dr: i32,
    c_left: usize,
) {
    if board.cells[r][c_left] == EMPTY && board.cells[r][c_left + 1] == EMPTY {
        stack.push((r, c_left));
    } else if board.cells[r][c_left] == WALL || board.cells[r][c_left + 1] == WALL {
        // no-op
    } else if board.cells[r][c_left] == LEFT_BOX && board.cells[r][c_left + 1] == RIGHT_BOX {
        do_move(false, stack, board, (r as i32 + dr) as usize, dr, c_left);
        if !is_first {
            stack.push((r, c_left));
        }
    } else {
        if board.cells[r][c_left] == RIGHT_BOX {
            do_move(
                false,
                stack,
                board,
                (r as i32 + dr) as usize,
                dr,
                c_left - 1,
            );
            if !is_first {
                stack.push((r, c_left));
            }
        }
        if board.cells[r][c_left + 1] == LEFT_BOX {
            do_move(
                false,
                stack,
                board,
                (r as i32 + dr) as usize,
                dr,
                c_left + 1,
            );
            if !is_first {
                stack.push((r, c_left));
            }
        }
    }
}

fn move_up_or_down(board: &mut Board, robot_r: &mut usize, robot_c: &mut usize, dr: i32) {
    let dc = 0;
    let try_r = (*robot_r as i32 + dr) as usize;
    let try_c = (*robot_c as i32 + dc) as usize;
    let try_c_left = if board.cells[try_r][(*robot_c as i32 + dc) as usize] == LEFT_BOX {
        *robot_c
    } else if board.cells[try_r][try_c] == RIGHT_BOX {
        *robot_c - 1
    } else {
        unreachable!()
    };

    if can_move_into(board, try_r, dr, try_c_left) && do_move_first(board, try_r, dr, try_c_left) {
        match board.cells[try_r][*robot_c] {
            LEFT_BOX => board.cells[(*robot_r as i32 + dr) as usize][*robot_c + 1] = EMPTY,
            RIGHT_BOX => board.cells[(*robot_r as i32 + dr) as usize][*robot_c - 1] = EMPTY,
            _ => {}
        }
        move_robot(board, robot_r, robot_c, dr, 0);
    }
}

fn make_all_moves(board: &mut Board, moves: &Board) {
    let (mut robot_r, mut robot_c) = board.find(ROBOT);

    for (r, c) in iproduct!(0..moves.num_rows, 0..moves.num_cols) {
        let (dr, dc) = match moves.cells[r][c] {
            '^' => (-1, 0),
            'v' => (1, 0),
            '<' => (0, -1),
            '>' => (0, 1),
            _ => unreachable!(),
        };
        match board.cells[(robot_r as i32 + dr) as usize][(robot_c as i32 + dc) as usize] {
            EMPTY => {
                move_robot(board, &mut robot_r, &mut robot_c, dr, dc);
            }
            SINGLE_BOX => {
                move_single_box(board, &mut robot_r, &mut robot_c, dr, dc);
            }
            LEFT_BOX | RIGHT_BOX => {
                match moves.cells[r][c] {
                    '^' | 'v' => {
                        move_up_or_down(board, &mut robot_r, &mut robot_c, dr);
                    }
                    '<' | '>' => {
                        move_double_box_sideways(board, &mut robot_r, &mut robot_c, dc);
                    }
                    _ => unreachable!(),
                };
            }
            _ => {}
        }
    }
}

fn stretch_board(board: &Board) -> Board {
    let mut wide_board = Board::create_empty(board.num_rows, board.num_cols * 2, EMPTY);

    for (r, c) in iproduct!(0..board.num_rows, 0..board.num_cols) {
        match board.cells[r][c] {
            WALL => {
                wide_board.cells[r][c * 2] = WALL;
                wide_board.cells[r][c * 2 + 1] = WALL;
            }
            SINGLE_BOX => {
                wide_board.cells[r][c * 2] = LEFT_BOX;
                wide_board.cells[r][c * 2 + 1] = RIGHT_BOX;
            }
            ROBOT => {
                wide_board.cells[r][c * 2] = ROBOT;
            }
            _ => {}
        }
    }
    wide_board
}

fn solve(board: &mut Board, moves: &Board) -> usize {
    make_all_moves(board, moves);
    sum_box_distances(board)
}

pub fn solve15(input: &[String]) -> Solutions {
    let mut boards = Board::from_input_multiple(input);
    let moves = boards.pop().unwrap();
    let mut board_one = boards.pop().unwrap();
    let mut board_two = stretch_board(&board_one);

    (
        Solution::USIZE(solve(&mut board_one, &moves)),
        Solution::USIZE(solve(&mut board_two, &moves)),
    )
}

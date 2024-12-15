use aoc::board::Board;
use itertools::iproduct;

const ROBOT: char = '@';
const BOX: char = 'O';
const EMPTY: char = '.';
const WALL: char = '#';

fn sum_box_distances(board: &Board) -> usize {
    let mut sum = 0;
    for (c, r) in iproduct!(0..board.num_cols, 0..board.num_rows) {
        if board.cells[r][c] == BOX {
            sum += 100 * r + c;
        }
    }
    sum
}

fn make_all_moves(board: &mut Board, moves: &Board) {
    let (mut robot_r, mut robot_c) = board.find(ROBOT);
    board.print();

    for (r, c) in iproduct!(0..moves.num_rows, 0..moves.num_cols) {
        // println!("DBG-1 Move {r},{c},             robot {robot_r},{robot_c}");
        let (dr, dc) = match moves.cells[r][c] {
            '^' => (-1, 0),
            'v' => (1, 0),
            '<' => (0, -1),
            '>' => (0, 1),
            _ => unreachable!(),
        };
        match board.cells[(robot_r as i32 + dr) as usize][(robot_c as i32 + dc) as usize] {
            WALL => {} // no-op
            BOX => {
                // Find first non-box. If a wall, no-op. If empty, move all the boxes.
                let mut try_r = robot_r as i32 + dr;
                let mut try_c = robot_c as i32 + dc;
                loop {
                    if board.cells[try_r as usize][try_c as usize] == EMPTY {
                        // Can move all the boxes.
                        break;
                    } else if board.cells[try_r as usize][try_c as usize] == WALL {
                        // Can't move anything.
                        try_r = i32::MAX;
                        try_c = i32::MAX;
                        break;
                    } else if board.cells[try_r as usize][try_c as usize] == BOX {
                        // try next cell
                        try_r += dr;
                        try_c += dc;
                    }
                }
                if try_r != i32::MAX {
                    // Replace empty with box.
                    board.cells[try_r as usize][try_c as usize] = BOX;
                    // Replace first box with robot.
                    board.cells[(robot_r as i32 + dr) as usize][(robot_c as i32 + dc) as usize] =
                        ROBOT;
                    // Replace robot with empty.
                    board.cells[robot_r][robot_c] = EMPTY;
                    robot_r = ((robot_r as i32) + dr) as usize;
                    robot_c = ((robot_c as i32) + dc) as usize;
                }
            }
            EMPTY => {
                board.cells[robot_r][robot_c] = EMPTY;
                board.cells[(robot_r as i32 + dr) as usize][(robot_c as i32 + dc) as usize] = ROBOT;
                robot_r = ((robot_r as i32) + dr) as usize;
                robot_c = ((robot_c as i32) + dc) as usize;
            } // move
            _ => unreachable!(),
        }
    }
}

pub fn solve15(input: &[String]) -> (i128, i128) {
    let mut boards = Board::from_input_multiple(input);
    let moves = boards.pop().unwrap();
    let mut board = boards.pop().unwrap();

    make_all_moves(&mut board, &moves);
    board.print();
    let solution_one = sum_box_distances(&board);
    (solution_one as i128, 0_i128)
}

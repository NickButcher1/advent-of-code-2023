use aoc::board::Board;
use itertools::iproduct;

const EMPTY: char = '.';
const OBSTRUCTION: char = '#';
const GUARD_START: char = '^';
const GUARD_PATH: char = 'X';
const BORDER: char = 'O';

fn parse_input(input: &[String]) -> ((usize, usize), usize, Board) {
    let mut board = Board::from_input(input);
    board.add_border(BORDER);
    let mut guard_pos = board.find(GUARD_START);
    board.cells[guard_pos.0][guard_pos.1] = GUARD_PATH;

    (guard_pos, 0, board)
}

fn find_path(mut guard_pos: (usize, usize), mut guard_direction: usize, board: &mut Board) -> usize {
    let guard_moves: Vec<(isize, isize)> = vec![(-1, 0), (0, 1), (1, 0), (0, -1)]; // degrees is index.
    let max_steps = (board.num_rows -2) * (board.num_cols - 2);
    let mut solution = 0;
    let mut loop_count = 0;
    loop {
        loop_count += 1;
        let next_pos = ((guard_pos.0 as isize + guard_moves[guard_direction].0) as usize, (guard_pos.1 as isize + guard_moves[guard_direction].1) as usize);
        let next_cell = board.cells[next_pos.0][next_pos.1];
        if loop_count > max_steps {
            break;
        } else if next_cell == BORDER {
            solution = board.count(GUARD_PATH);
            break;
        } else if next_cell == OBSTRUCTION {
            guard_direction += 1;
            guard_direction %= 4;
        } else if next_cell == EMPTY || next_cell == GUARD_PATH {
            guard_pos = next_pos;
            board.cells[next_pos.0][next_pos.1] = GUARD_PATH;
        }
    }

    solution as usize
}

pub fn solve06(input: &[String]) -> (i128, i128) {
    let (guard_pos, guard_direction, mut board) = parse_input(input);
    let solution_one = find_path(guard_pos, guard_direction, &mut board);

    let mut solution_two = 0;
    let num_rows = board.num_rows;
    let num_cols = board.num_cols;

    for (c, r) in iproduct!(1..num_cols - 1, 1..num_rows - 1) {
        let (guard_pos, guard_direction, mut board) = parse_input(input);
        if board.cells[r][c] == EMPTY {
            board.cells[r][c] = OBSTRUCTION;
            let steps = find_path(guard_pos, guard_direction, &mut board);
            if steps == 0 {
                solution_two += 1;
            }
        } else if board.cells[r][c] == OBSTRUCTION || board.cells[r][c] == GUARD_PATH {
            // Skip
        }
    }

    (solution_one as i128, solution_two as i128)
}

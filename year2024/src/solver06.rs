use aoc::board::Board;
use itertools::iproduct;

const EMPTY: char = '.';
const OBSTRUCTION: char = '#';
const GUARD_START: char = '^';
const GUARD_PATH: char = 'X';
const BORDER: char = 'O';

const GUARD_MOVES: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn parse_input(input: &[String]) -> ((usize, usize), usize, Board) {
    let mut board = Board::from_input(input);
    board.add_border(BORDER);
    let guard_pos = board.find(GUARD_START);
    board.cells[guard_pos.0][guard_pos.1] = GUARD_PATH;

    (guard_pos, 0, board)
}

fn find_path(
    board: &mut Board,
    max_steps: usize,
    mut guard_pos: (usize, usize),
    mut guard_direction: usize,
) -> usize {
    for _ in 1..=max_steps {
        let (x, y) = guard_pos;
        let (dx, dy) = GUARD_MOVES[guard_direction];
        let next_pos = ((x as isize + dx) as usize, (y as isize + dy) as usize);
        let next_cell = board.cells[next_pos.0][next_pos.1];

        match next_cell {
            BORDER => {
                return board.count(GUARD_PATH) as usize;
            }
            OBSTRUCTION => {
                guard_direction = (guard_direction + 1) % 4;
            }
            EMPTY | GUARD_PATH => {
                guard_pos = next_pos;
                board.cells[next_pos.0][next_pos.1] = GUARD_PATH;
            }
            _ => (),
        }
    }

    0
}

pub fn solve06(input: &[String]) -> (i128, i128) {
    let (guard_pos, guard_direction, original_board) = parse_input(input);
    let max_steps = original_board.count(EMPTY) as usize;

    let mut board_part_one = original_board.clone();
    let solution_one = find_path(&mut board_part_one, max_steps, guard_pos, guard_direction);

    let mut solution_two = 0;
    let num_rows = original_board.num_rows;
    let num_cols = original_board.num_cols;

    for (c, r) in iproduct!(1..num_cols - 1, 1..num_rows - 1) {
        let mut board_part_two = original_board.clone();
        if board_part_two.cells[r][c] == EMPTY {
            board_part_two.cells[r][c] = OBSTRUCTION;
            let steps = find_path(&mut board_part_two, max_steps, guard_pos, guard_direction);
            if steps == 0 {
                solution_two += 1;
            }
        }
    }

    (solution_one as i128, solution_two as i128)
}

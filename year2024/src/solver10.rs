use aoc::board::Board;
use aoc::solution::{Solution, Solutions};
use itertools::iproduct;

const EMPTY: char = '.';

// Return the total number of trails ending at a '9' from the supplied position.
fn expand_from(
    input_board: &Board,
    output_board: &mut Board,
    r: usize,
    c: usize,
    height: char,
) -> i32 {
    if input_board.cells[r][c] == '9' {
        output_board.cells[r][c] = '9';
        return 1;
    }

    let mut total_paths = 0;
    let old_height = height.to_digit(10).unwrap();
    for (rd, cd) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
        let new_r = r as i32 + rd;
        let new_c = c as i32 + cd;
        if new_r >= 0
            && new_c >= 0
            && (new_r as usize) < input_board.num_rows
            && (new_c as usize) < input_board.num_cols
        {
            let new_digit = input_board.cells[new_r as usize][new_c as usize]
                .to_digit(10)
                .unwrap();
            if new_digit == (old_height + 1) {
                total_paths += expand_from(
                    input_board,
                    output_board,
                    new_r as usize,
                    new_c as usize,
                    input_board.cells[new_r as usize][new_c as usize],
                );
            }
        }
    }

    total_paths
}

pub fn solve10(input: &[String]) -> Solutions {
    let board = Board::from_input(input);

    // Part one counts the number of pairs of 0 and 9 that have any path between them.
    // Part one counts the number of unique paths between any 0 and any 9.
    let (solution_one, solution_two) = iproduct!(0..board.num_rows, 0..board.num_cols)
        .filter(|&(r, c)| board.cells[r][c] == '0')
        .map(|(r, c)| {
            let mut output_board = Board::create_empty(board.num_rows, board.num_cols, EMPTY);
            let part_two_paths = expand_from(&board, &mut output_board, r, c, '0');
            let part_one_paths = output_board.count('9');
            (part_one_paths, part_two_paths)
        })
        .fold((0, 0), |(solution_one, solution_two), (one, two)| {
            (solution_one + one, solution_two + two)
        });

    (Solution::U64(solution_one), Solution::I32(solution_two))
}

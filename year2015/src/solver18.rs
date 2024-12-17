use aoc::board::Board;
use aoc::solution::{Solution, Solutions};

const OFF: char = '.';
const ON: char = '#';

pub fn solve18(input: &[String]) -> Solutions {
    (
        Solution::U64(solve(input, false)),
        Solution::U64(solve(input, true)),
    )
}

pub fn solve(input: &[String], is_part_2: bool) -> u64 {
    let mut board = Board::from_input(input);
    board.add_border(OFF);

    if is_part_2 {
        board.set_corners_to(ON, true);
    }

    for _ in 0..100 {
        board.game_of_life_step(OFF, ON);
        if is_part_2 {
            board.set_corners_to(ON, true);
        }
    }

    board.count(ON)
}

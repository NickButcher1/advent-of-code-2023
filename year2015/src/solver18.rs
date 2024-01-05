use aoc::board::Board;

const OFF: char = '.';
const ON: char = '#';

pub fn solve18(input: &[String]) -> (i128, i128) {
    (
        i128::from(solve(input, false)),
        i128::from(solve(input, true)),
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

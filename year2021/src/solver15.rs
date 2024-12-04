use aoc::board::Board;

pub fn solve15(input: &[String]) -> (i128, i128) {
    let board_one = Board::from_input(input);

    let mut board_two = Board::create_empty(board_one.num_rows * 5, board_one.num_cols * 5, '.');

    for big_x in (0..board_two.num_rows).step_by(board_one.num_rows) {
        for big_y in (0..board_two.num_cols).step_by(board_one.num_cols) {
            let increment = (big_x + big_y) / board_one.num_rows;

            for x in 0..board_one.num_rows {
                for y in 0..board_one.num_cols {
                    let new_digit =
                        ((board_one.cells[x][y].to_digit(10).unwrap() + increment as u32 - 1) % 9)
                            + 1;
                    board_two.cells[big_x + x][big_y + y] =
                        char::from_digit(new_digit, 10).unwrap();
                }
            }
        }
    }

    (
        board_one.cheapest_path() as i128,
        board_two.cheapest_path() as i128,
    )
}

use aoc::board::Board;

pub fn solve09(input: &[String]) -> (i128, i128) {
    let mut board = Board::from_input(input);
    board.add_border('A');

    let mut sum_risk_levels = 0;
    for r in 1..board.num_rows - 1 {
        for c in 1..board.num_cols - 1 {
            if board.cells[r][c] < board.cells[r - 1][c]
                && board.cells[r][c] < board.cells[r + 1][c]
                && board.cells[r][c] < board.cells[r][c - 1]
                && board.cells[r][c] < board.cells[r][c + 1]
            {
                sum_risk_levels += 1 + board.cells[r][c].to_digit(10).unwrap();
            }
        }
    }

    board.replace('A', '#');
    board.replace('9', '#');
    board.replace('0', '.');
    board.replace('1', '.');
    board.replace('2', '.');
    board.replace('3', '.');
    board.replace('4', '.');
    board.replace('5', '.');
    board.replace('6', '.');
    board.replace('7', '.');
    board.replace('8', '.');
    board.print();
    (sum_risk_levels as i128, 0)
}

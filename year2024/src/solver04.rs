use aoc::board::Board;

pub fn solve04(input: &[String]) -> (i128, i128) {
    let board = Board::from_input(input);

    let mut xmas_count_1 = 0;

    for x in 0..(board.num_cols) {
        for y in 0..(board.num_rows) {
            if (y < (board.num_cols - 3))
                && board.cells[x][y] == 'X'
                && board.cells[x][y + 1] == 'M'
                && board.cells[x][y + 2] == 'A'
                && board.cells[x][y + 3] == 'S'
            {
                xmas_count_1 += 1;
            }

            if (x < (board.num_rows - 3))
                && board.cells[x][y] == 'X'
                && board.cells[x + 1][y] == 'M'
                && board.cells[x + 2][y] == 'A'
                && board.cells[x + 3][y] == 'S'
            {
                xmas_count_1 += 1;
            }

            if (x < (board.num_cols - 3))
                && (y < (board.num_rows - 3))
                && board.cells[x][y] == 'X'
                && board.cells[x + 1][y + 1] == 'M'
                && board.cells[x + 2][y + 2] == 'A'
                && board.cells[x + 3][y + 3] == 'S'
            {
                xmas_count_1 += 1;
            }

            if (x < (board.num_cols - 3))
                && y >= 3
                && board.cells[x][y] == 'X'
                && board.cells[x + 1][y - 1] == 'M'
                && board.cells[x + 2][y - 2] == 'A'
                && board.cells[x + 3][y - 3] == 'S'
            {
                xmas_count_1 += 1;
            }

            if (y < (board.num_cols - 3))
                && board.cells[x][y] == 'S'
                && board.cells[x][y + 1] == 'A'
                && board.cells[x][y + 2] == 'M'
                && board.cells[x][y + 3] == 'X'
            {
                xmas_count_1 += 1;
            }

            if (x < (board.num_rows - 3))
                && board.cells[x][y] == 'S'
                && board.cells[x + 1][y] == 'A'
                && board.cells[x + 2][y] == 'M'
                && board.cells[x + 3][y] == 'X'
            {
                xmas_count_1 += 1;
            }

            if (x < (board.num_cols - 3))
                && (y < (board.num_rows - 3))
                && board.cells[x][y] == 'S'
                && board.cells[x + 1][y + 1] == 'A'
                && board.cells[x + 2][y + 2] == 'M'
                && board.cells[x + 3][y + 3] == 'X'
            {
                xmas_count_1 += 1;
            }

            if (x < (board.num_cols - 3))
                && y >= 3
                && board.cells[x][y] == 'S'
                && board.cells[x + 1][y - 1] == 'A'
                && board.cells[x + 2][y - 2] == 'M'
                && board.cells[x + 3][y - 3] == 'X'
            {
                xmas_count_1 += 1;
            }
        }
    }

    let mut xmas_count_2 = 0;
    for x in 1..(board.num_cols - 1) {
        for y in 1..(board.num_rows - 1) {
            if board.cells[x][y] == 'A' {
                if (board.cells[x - 1][y - 1] == 'M'
                    && board.cells[x + 1][y + 1] == 'S'
                    && board.cells[x - 1][y + 1] == 'M'
                    && board.cells[x + 1][y - 1] == 'S')
                    || (board.cells[x - 1][y - 1] == 'M'
                        && board.cells[x + 1][y + 1] == 'S'
                        && board.cells[x - 1][y + 1] == 'S'
                        && board.cells[x + 1][y - 1] == 'M')
                    || (board.cells[x - 1][y - 1] == 'S'
                        && board.cells[x + 1][y + 1] == 'M'
                        && board.cells[x - 1][y + 1] == 'S'
                        && board.cells[x + 1][y - 1] == 'M')
                    || (board.cells[x - 1][y - 1] == 'S'
                        && board.cells[x + 1][y + 1] == 'M'
                        && board.cells[x - 1][y + 1] == 'M'
                        && board.cells[x + 1][y - 1] == 'S')
                {
                    xmas_count_2 += 1;
                }
            }
        }
    }

    (xmas_count_1 as i128, xmas_count_2 as i128)
}

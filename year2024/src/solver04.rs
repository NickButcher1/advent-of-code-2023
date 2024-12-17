use aoc::board::Board;
use aoc::solution::{Solution, Solutions};

#[allow(clippy::needless_range_loop)]
fn count_xmas(board: &Board, x: usize, y: usize, dx: isize, dy: isize) -> usize {
    let xmas = ["XMAS", "SAMX"];

    xmas.iter()
        .filter(|&&xmas| {
            let chars: Vec<char> = xmas.chars().collect();
            let len = chars.len();

            for i in 0..len {
                let nx = x as isize + i as isize * dx;
                let ny = y as isize + i as isize * dy;

                if nx < 0
                    || ny < 0
                    || nx >= board.num_cols as isize
                    || ny >= board.num_rows as isize
                {
                    return false;
                }

                if board.cells[nx as usize][ny as usize] != chars[i] {
                    return false;
                }
            }

            true
        })
        .count()
}

fn is_mas_cross(board: &Board, x: usize, y: usize) -> bool {
    let valid_m_and_s = [
        (('M', 'S'), ('M', 'S')),
        (('M', 'S'), ('S', 'M')),
        (('S', 'M'), ('S', 'M')),
        (('S', 'M'), ('M', 'S')),
    ];

    valid_m_and_s
        .iter()
        .any(|&(left_diagonal, right_diagonal)| {
            board.cells[x - 1][y - 1] == left_diagonal.0
                && board.cells[x + 1][y + 1] == left_diagonal.1
                && board.cells[x - 1][y + 1] == right_diagonal.0
                && board.cells[x + 1][y - 1] == right_diagonal.1
        })
}

pub fn solve04(input: &[String]) -> Solutions {
    let board = Board::from_input(input);

    let mut xmas_count_1 = 0;

    for x in 0..board.num_cols {
        for y in 0..board.num_rows {
            xmas_count_1 += count_xmas(&board, x, y, 0, 1) + // Horizontal
            count_xmas(&board, x, y, 1, 0) + // Vertical
            count_xmas(&board, x, y, 1, 1) + // Diagonal Down-Right
            count_xmas(&board, x, y, 1, -1); // Diagonal Up-Right
        }
    }

    let mut xmas_count_2 = 0;
    for x in 1..(board.num_cols - 1) {
        for y in 1..(board.num_rows - 1) {
            if board.cells[x][y] == 'A' && is_mas_cross(&board, x, y) {
                xmas_count_2 += 1;
            }
        }
    }

    (Solution::USIZE(xmas_count_1), Solution::I32(xmas_count_2))
}

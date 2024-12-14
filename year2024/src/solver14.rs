use aoc::board::Board;
use aoc::int_board::IntBoard;
use aoc::moving_point::read_points_2024_day_14;
use itertools::iproduct;

const EMPTY: char = '.';

#[allow(dead_code)]
fn print_tree(board: &IntBoard) {
    let mut tree_board = Board::create_empty(board.num_rows, board.num_cols, EMPTY);
    for (c, r) in iproduct!(0..board.num_cols, 0..board.num_rows) {
        tree_board.cells[r][c] = if board.cells[r][c] > 0 && board.cells[r][c] < 10 {
            (board.cells[r][c] as u8 + b'0') as char
        } else {
            ' '
        };
    }
    tree_board.print();
}

fn calculate_position_at_time(px: isize, vx: isize, seconds: isize, board_size: isize) -> isize {
    (((px + seconds * vx) % board_size) + board_size) % board_size
}

pub fn solve14(input: &[String]) -> (i128, i128) {
    let robots = read_points_2024_day_14(input);

    let (num_rows, num_cols) = if robots.len() == 12 {
        (7, 11)
    } else {
        (103, 101)
    };

    let fold_r = (num_rows as isize - 1) / 2;
    let fold_c = (num_cols as isize - 1) / 2;

    let mut solution_one = 0;
    let mut solution_two = 0;

    // Hardcoded upper limit found by trial and error.
    for seconds in 1..=10_000 {
        // Measure average absolute distance from the centre of each point.
        let mut centrality_total = 0;

        let mut finish_board = IntBoard::create_empty(num_rows, num_cols);
        for r in &robots {
            let px = calculate_position_at_time(r.px, r.vx, seconds, num_cols as isize);
            let py = calculate_position_at_time(r.py, r.vy, seconds, num_rows as isize);

            finish_board.cells[py as usize][px as usize] += 1;

            centrality_total += (fold_c - px).abs() + (fold_r - py).abs();
        }

        if seconds == 100 {
            let mut quadrants = [0; 4];
            for (c, r) in iproduct!(0..num_cols, 0..num_rows) {
                let index = match ((r as isize).cmp(&fold_r), (c as isize).cmp(&fold_c)) {
                    (std::cmp::Ordering::Less, std::cmp::Ordering::Less) => 0,
                    (std::cmp::Ordering::Less, std::cmp::Ordering::Greater) => 1,
                    (std::cmp::Ordering::Greater, std::cmp::Ordering::Less) => 2,
                    (std::cmp::Ordering::Greater, std::cmp::Ordering::Greater) => 3,
                    _ => usize::MAX,
                };
                if index != usize::MAX {
                    quadrants[index] += finish_board.cells[r][c];
                }
            }
            solution_one = quadrants.iter().product();
        }

        let centrality_average = centrality_total / (2 * robots.len() as isize);
        // Hardcoded limit found by trial and error.
        if centrality_average < 18 {
            solution_two = seconds;
            // print_tree(&finish_board);
            break;
        }
    }

    (solution_one as i128, solution_two as i128)
}

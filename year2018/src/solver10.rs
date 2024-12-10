use aoc::board::Board;
use aoc::moving_point::{max_x_y, min_x_y, read_points, tick, MovingPoint};

const EMPTY: char = '.';
const FULL: char = '#';

fn area_from_points(points: &[MovingPoint]) -> (isize, isize) {
    let (min_x, min_y) = min_x_y(points);
    let (max_x, max_y) = max_x_y(points);
    (max_x - min_x + 1, max_y - min_y + 1)
}

fn board_from_points(points: &[MovingPoint]) -> Board {
    let (min_x, min_y) = min_x_y(points);
    let (max_x, max_y) = max_x_y(points);
    let mut board = Board::create_empty(
        (max_x - min_x + 1) as usize,
        (max_y - min_y + 1) as usize,
        EMPTY,
    );

    points.iter().for_each(|point| {
        board.cells[(point.px - min_x) as usize][(point.py - min_y) as usize] = FULL;
    });

    board.flip();
    board
}

pub fn solve10(input: &[String]) -> (i128, i128) {
    let mut points = read_points(input);
    let mut prev_points = points.clone();
    let mut prev_num_rows = isize::MAX;
    let mut step = 0;

    // Assume board will contract until it displays the message, then expand again. Stop as soon as we detect that
    // expansion, then show the previous board.
    loop {
        tick(&mut points);
        let (num_rows, _) = area_from_points(&points);
        if num_rows > prev_num_rows {
            let board = board_from_points(&prev_points);
            board.print();
            return (-1, step);
        }
        step += 1;
        prev_points = points.clone();
        prev_num_rows = num_rows;
    }
}

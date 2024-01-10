use aoc::board::Board;
use regex::Regex;

const EMPTY: char = '.';
const FULL: char = '#';

#[derive(Clone, Debug)]
struct Point {
    px: isize,
    py: isize,
    vx: isize,
    vy: isize,
}

fn read_points(input: &[String]) -> Vec<Point> {
    let re = Regex::new(r"position=<( *)(-?\d+),( *)(-?\d+)> velocity=<( *)(-?\d+),( *)(-?\d+)>$")
        .unwrap();

    input
        .iter()
        .map(|line| {
            let captures = re.captures(line).unwrap();
            Point {
                px: captures.get(2).unwrap().as_str().parse::<isize>().unwrap(),
                py: captures.get(4).unwrap().as_str().parse::<isize>().unwrap(),
                vx: captures.get(6).unwrap().as_str().parse::<isize>().unwrap(),
                vy: captures.get(8).unwrap().as_str().parse::<isize>().unwrap(),
            }
        })
        .collect()
}

fn area_from_points(points: &[Point]) -> (isize, isize) {
    let min_x = points.iter().min_by_key(|point| point.px).unwrap().px;
    let min_y = points.iter().min_by_key(|point| point.py).unwrap().py;
    let max_x = points.iter().max_by_key(|point| point.px).unwrap().px;
    let max_y = points.iter().max_by_key(|point| point.py).unwrap().py;
    (max_x - min_x + 1, max_y - min_y + 1)
}

fn board_from_points(points: &[Point]) -> Board {
    let min_x = points.iter().min_by_key(|point| point.px).unwrap().px;
    let min_y = points.iter().min_by_key(|point| point.py).unwrap().py;
    let max_x = points.iter().max_by_key(|point| point.px).unwrap().px;
    let max_y = points.iter().max_by_key(|point| point.py).unwrap().py;
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

fn tick(points: &mut [Point]) {
    points.iter_mut().for_each(|point| {
        point.px += point.vx;
        point.py += point.vy;
    });
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

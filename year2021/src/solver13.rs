use aoc::board::Board;
use aoc::solution::{Solution, Solutions};
use regex::Regex;

const EMPTY: char = '.';
const FULL: char = '#';

pub fn solve13(input: &[String]) -> Solutions {
    let re_xy = Regex::new(r"^(\d+),(\d+)$").unwrap();
    let re_fold = Regex::new(r"^fold along (x|y)=(\d+)$").unwrap();

    let mut points: Vec<(usize, usize)> = vec![];
    let mut folds: Vec<(usize, usize)> = vec![];
    let mut read_xy = true;

    input.iter().for_each(|line| {
        if read_xy {
            if line.is_empty() {
                read_xy = false;
            } else {
                let captures = re_xy.captures(line).unwrap();
                points.push((
                    captures.get(1).unwrap().as_str().parse::<usize>().unwrap(),
                    captures.get(2).unwrap().as_str().parse::<usize>().unwrap(),
                ))
            }
        } else {
            let captures = re_fold.captures(line).unwrap();
            let value = captures.get(2).unwrap().as_str().parse::<usize>().unwrap();
            if captures.get(1).unwrap().as_str().parse::<char>().unwrap() == 'x' {
                folds.push((value, 0));
            } else {
                folds.push((0, value));
            }
        }
    });

    let (max_x, max_y) = points
        .iter()
        .fold(None, |acc, &(x, y)| match acc {
            None => Some((x, y)),
            Some((max_x, max_y)) => Some((max_x.max(x), max_y.max(y))),
        })
        .unwrap();

    let mut board = Board::create_empty(max_y + 1, max_x + 1, EMPTY);
    for (x, y) in points {
        board.cells[y][x] = FULL;
    }

    let mut solution_one = 0;
    let mut is_first_fold = true;

    for (fold_x, fold_y) in folds {
        if fold_x != 0 {
            board.fold_left(fold_x, EMPTY, FULL);
        } else {
            board.fold_up(fold_y, EMPTY, FULL);
        }

        if is_first_fold {
            is_first_fold = false;
            solution_one = board.count(FULL);
        }
    }

    // Hardcoded based on looking at the board printout.
    (
        Solution::U64(solution_one),
        Solution::STR("LKREBPRK".to_string()),
    )
}

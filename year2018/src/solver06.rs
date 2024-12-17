use aoc::board::Board;
use aoc::solution::{Solution, Solutions};
use itertools::iproduct;
use regex::Regex;

const EMPTY: char = '.';

fn parse_line(line: &str, re: &Regex) -> (usize, usize) {
    let captures = re.captures(line).unwrap();
    (
        captures.get(1).unwrap().as_str().parse::<usize>().unwrap(),
        captures.get(2).unwrap().as_str().parse::<usize>().unwrap(),
    )
}

fn taxicab_distance(x1: usize, y1: usize, x2: usize, y2: usize) -> usize {
    (x1 as isize - x2 as isize).abs_diff(0) + (y1 as isize - y2 as isize).abs_diff(0)
}

// I should write a version of Board that uses integers!
fn index_to_char(index: usize) -> char {
    if index < 26 {
        std::char::from_u32('A' as u32 + index as u32).unwrap()
    } else {
        std::char::from_u32('a' as u32 + index as u32 - 26).unwrap()
    }
}

fn char_to_index(character: char) -> usize {
    if character.is_ascii_uppercase() {
        character as usize - 'A' as usize
    } else {
        character as usize - 'a' as usize + 26
    }
}

pub fn solve06(input: &[String]) -> Solutions {
    let re = Regex::new(r"^(\d+), (\d+)$").unwrap();

    let mut board_one = Board::create_empty(400, 400, EMPTY);
    let mut board_two = Board::create_empty(400, 400, EMPTY);

    let points = input
        .iter()
        .map(|line| parse_line(line, &re))
        .collect::<Vec<(usize, usize)>>();

    let mut count_ids: Vec<i32> = vec![0; 52];
    for (x, y) in iproduct!(0..board_one.num_cols, 0..board_one.num_rows) {
        let mut closest_points = points
            .iter()
            .enumerate()
            .map(|(index, point)| (taxicab_distance(x, y, point.0, point.1), index))
            .collect::<Vec<_>>();
        closest_points.sort_by_key(|k| k.0);

        let total_taxicab_distance_to_all_points = closest_points
            .iter()
            .map(|(distance, _)| *distance)
            .sum::<usize>();
        if total_taxicab_distance_to_all_points < 10_000 {
            board_two.cells[y][x] = '#';
        }

        if closest_points[0].0 == closest_points[1].0 {
            board_one.cells[y][x] = EMPTY;
        } else {
            board_one.cells[y][x] = index_to_char(closest_points[0].1);
            count_ids[closest_points[0].1] += 1;
        }
    }

    // Eliminate counts for any ID on the edge - these go to infinity so aren't wanted.
    for (x, y) in iproduct!(0..board_one.num_cols, 0..board_one.num_rows) {
        if (x == 0 || y == 0 || x == (board_one.num_cols - 1) || y == (board_one.num_rows - 1))
            && board_one.cells[y][x] != EMPTY
        {
            let index = char_to_index(board_one.cells[y][x]);
            count_ids[index] = 0;
        }
    }

    let solution_one = count_ids.iter().max().unwrap_or(&0);
    let solution_two = board_two.count('#');

    (Solution::I32(*solution_one), Solution::U64(solution_two))
}

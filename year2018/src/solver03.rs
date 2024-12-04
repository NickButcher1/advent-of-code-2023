use aoc::board::Board;
use itertools::iproduct;
use regex::Regex;

const EMPTY: char = '.';
const SINGLE: char = 'O';
const OVERLAP: char = 'X';

fn parse_line(line: &str, re: &Regex) -> (u32, usize, usize, usize, usize) {
    let captures = re.captures(line).unwrap();
    (
        captures.get(1).unwrap().as_str().parse::<u32>().unwrap(),
        captures.get(2).unwrap().as_str().parse::<usize>().unwrap(),
        captures.get(3).unwrap().as_str().parse::<usize>().unwrap(),
        captures.get(4).unwrap().as_str().parse::<usize>().unwrap(),
        captures.get(5).unwrap().as_str().parse::<usize>().unwrap(),
    )
}

pub fn solve03(input: &[String]) -> (i128, i128) {
    let re = Regex::new(r"^#(\d+) @ (\d+),(\d+): (\d+)x(\d+)$").unwrap();

    let mut board = Board::create_empty(1000, 1000, EMPTY);

    input.iter().for_each(|line| {
        let (_, corner_x, corner_y, w, h) = parse_line(line, &re);

        for (x, y) in iproduct!(corner_x..corner_x + w, corner_y..corner_y + h) {
            board.cells[y][x] = match board.cells[y][x] {
                EMPTY => SINGLE,
                SINGLE => OVERLAP,
                _ => board.cells[y][x],
            };
        }
    });
    let solution_one = board.count(OVERLAP);

    let solution_two = input
        .iter()
        .find_map(|line| {
            let (id, corner_x, corner_y, w, h) = parse_line(line, &re);
            (!iproduct!(corner_x..corner_x + w, corner_y..corner_y + h)
                .any(|(x, y)| board.cells[y][x] == OVERLAP))
            .then_some(id)
        })
        .unwrap();

    (solution_one as i128, solution_two as i128)
}

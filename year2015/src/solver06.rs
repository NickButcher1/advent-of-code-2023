use aoc::board::Board;
use aoc::solution::{Solution, Solutions};
use regex::Regex;

const OFF: char = '.';
const ON: char = '#';

#[allow(clippy::needless_range_loop)]
pub fn solve06(input: &[String]) -> Solutions {
    let mut part_1_board = Board::create_empty(1000, 1000, OFF);
    let mut part_2_board: Vec<Vec<usize>> = vec![vec![0; 1000]; 1000];

    let re = Regex::new(r"^(turn on|turn off|toggle) (\d+),(\d+) through (\d+),(\d+)$").unwrap();

    for line in input {
        let captures = re.captures(line).unwrap();
        let action = captures.get(1).unwrap().as_str();
        let r_min = captures.get(2).unwrap().as_str().parse::<usize>().unwrap();
        let c_min = captures.get(3).unwrap().as_str().parse::<usize>().unwrap();
        let r_max = captures.get(4).unwrap().as_str().parse::<usize>().unwrap();
        let c_max = captures.get(5).unwrap().as_str().parse::<usize>().unwrap();

        for r in r_min..=r_max {
            for c in c_min..=c_max {
                part_1_board.cells[r][c] = match action {
                    "turn on" => ON,
                    "turn off" => OFF,
                    "toggle" => {
                        if part_1_board.cells[r][c] == ON {
                            OFF
                        } else {
                            ON
                        }
                    }
                    _ => unreachable!(),
                };

                part_2_board[r][c] = match action {
                    "turn on" => part_2_board[r][c] + 1,
                    "turn off" => {
                        if part_2_board[r][c] == 0 {
                            0
                        } else {
                            part_2_board[r][c] - 1
                        }
                    }
                    "toggle" => part_2_board[r][c] + 2,
                    _ => unreachable!(),
                };
            }
        }
    }

    let mut part_2_sum: usize = 0;
    for row in part_2_board.iter().take(1000) {
        part_2_sum += row.iter().sum::<usize>();
    }

    (
        Solution::U64(part_1_board.count(ON)),
        Solution::USIZE(part_2_sum),
    )
}

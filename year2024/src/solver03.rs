use aoc::solution::{Solution, Solutions};
use regex::Regex;

pub fn solve(input: &[String], is_part_one: bool) -> usize {
    let re_mul = Regex::new(r"^mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let re_do = Regex::new(r"^do\(\)").unwrap();
    let re_dont = Regex::new(r"^don\'t\(\)").unwrap();

    let mut enabled = true;
    let mut total = 0;

    for input_line in input {
        let mut idx = 0;

        while let Some((byte_idx, _)) = input_line.char_indices().nth(idx) {
            let substring = &input_line[byte_idx..];
            if re_mul.is_match(substring) {
                let captures = re_mul.captures(substring).unwrap();
                let x = captures.get(1).unwrap().as_str().parse::<usize>().unwrap();
                let y = captures.get(2).unwrap().as_str().parse::<usize>().unwrap();

                if enabled || is_part_one {
                    total += x * y;
                }
            } else if re_do.is_match(substring) {
                enabled = true;
            } else if re_dont.is_match(substring) {
                enabled = false;
            }

            idx += 1;
        }
    }

    total
}

pub fn solve03(input: &[String]) -> Solutions {
    (
        Solution::USIZE(solve(input, true)),
        Solution::USIZE(solve(input, false)),
    )
}

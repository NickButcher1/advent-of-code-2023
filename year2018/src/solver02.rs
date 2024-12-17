use aoc::solution::{Solution, Solutions};
use itertools::Itertools;
use std::collections::HashMap;

fn solve_part_one(input: &[String]) -> i32 {
    let mut two_count = 0;
    let mut three_count = 0;

    input.iter().for_each(|line| {
        let mut chars: HashMap<char, i32> = HashMap::new();
        line.chars().for_each(|c| {
            *chars.entry(c).or_insert(0) += 1;
        });

        if chars.values().contains(&2) {
            two_count += 1
        }
        if chars.values().contains(&3) {
            three_count += 1
        }
    });

    two_count * three_count
}

fn solve_part_two(input: &[String]) {
    for i in 0..(input.len() - 1) {
        for j in i..input.len() {
            let (diff_count, last_diff_index) = input[i]
                .chars()
                .zip(input[j].chars())
                .enumerate()
                .filter(|(_, (c1, c2))| c1 != c2)
                .map(|(index, _)| index)
                .fold((0, 0), |(mut count, _), index| {
                    count += 1;
                    (count, index)
                });

            if diff_count == 1 {
                let _solution_part_two = input[i]
                    .chars()
                    .enumerate()
                    .filter(|(i, _)| *i != last_diff_index)
                    .map(|(_, c)| c)
                    .collect::<String>();
                // println!("{solution_part_two}");
                return;
            }
        }
    }
}

pub fn solve02(input: &[String]) -> Solutions {
    solve_part_two(input);
    (Solution::I32(solve_part_one(input)), Solution::I32(0))
}

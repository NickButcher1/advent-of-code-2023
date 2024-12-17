use aoc::input::string_to_vec_u64;
use aoc::solution::{Solution, Solutions};

pub fn solve02(input: &[String]) -> Solutions {
    let (part_1, part_2) = input.iter().fold((0, 0), |(part_1, mut part_2), line| {
        let numbers = string_to_vec_u64(line, '\t');
        let max_min_diff = numbers.iter().max().unwrap() - numbers.iter().min().unwrap();

        for i in 0..numbers.len() {
            for j in 0..numbers.len() {
                if numbers[i] > numbers[j] && numbers[i] % numbers[j] == 0 {
                    part_2 += numbers[i] / numbers[j];
                }
            }
        }

        (part_1 + max_min_diff, part_2)
    });

    (Solution::U64(part_1), Solution::U64(part_2))
}

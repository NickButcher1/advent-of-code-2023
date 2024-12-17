use aoc::input::string_to_vec_usize;
use aoc::solution::{Solution, Solutions};

pub fn solve06(input: &[String]) -> Solutions {
    let inputs = string_to_vec_usize(&input[0], ',');
    let mut lanternfish = inputs.iter().fold([0; 9], |mut counts, &input| {
        counts[input] += 1;
        counts
    });

    for _ in 1..=80 {
        lanternfish.rotate_left(1);
        lanternfish[6] += lanternfish[8];
    }

    let solution_one = lanternfish.iter().sum::<usize>();

    for _ in 81..=256 {
        lanternfish.rotate_left(1);
        lanternfish[6] += lanternfish[8];
    }

    let solution_two = lanternfish.iter().sum::<usize>();

    (Solution::USIZE(solution_one), Solution::USIZE(solution_two))
}

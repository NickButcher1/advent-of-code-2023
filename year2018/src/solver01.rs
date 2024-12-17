use aoc::input::to_vec_i32;
use aoc::solution::{Solution, Solutions};
use std::collections::HashSet;

pub fn solve01(input: &[String]) -> Solutions {
    let inputs = to_vec_i32(input);
    let solution_one = inputs.iter().sum::<i32>();

    let mut found: HashSet<i32> = HashSet::new();
    let mut running_total = 0;

    for i in 0..i32::MAX {
        running_total += inputs[i as usize % inputs.len()];
        if found.contains(&running_total) {
            break;
        }
        found.insert(running_total);
    }

    (Solution::I32(solution_one), Solution::I32(running_total))
}

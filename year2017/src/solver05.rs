use aoc::input::to_vec_isize;
use aoc::solution::{Solution, Solutions};

pub fn solve05(input: &[String]) -> Solutions {
    let mut jumps = to_vec_isize(input);
    let mut idx: isize = 0;
    let mut part_1_steps = 0;

    while idx >= 0 && idx < jumps.len() as isize {
        part_1_steps += 1;
        jumps[idx as usize] += 1;
        idx += jumps[idx as usize] - 1;
    }

    jumps = to_vec_isize(input);
    idx = 0;
    let mut part_2_steps = 0;

    while idx >= 0 && idx < jumps.len() as isize {
        part_2_steps += 1;
        if jumps[idx as usize] >= 3 {
            jumps[idx as usize] -= 1;
            idx += jumps[idx as usize] + 1;
        } else {
            jumps[idx as usize] += 1;
            idx += jumps[idx as usize] - 1;
        }
    }

    (Solution::I32(part_1_steps), Solution::I32(part_2_steps))
}

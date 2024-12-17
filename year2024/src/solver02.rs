use aoc::input::string_to_vec_u64;
use aoc::solution::{Solution, Solutions};

fn is_safe(levels: Vec<u64>) -> bool {
    let is_increasing = levels[1] > levels[0];
    for i in 1..levels.len() {
        let diff = levels[i] as i64 - levels[i - 1] as i64;
        match (diff, is_increasing) {
            (1..=3, true) | (-3..=-1, false) => continue,
            _ => return false,
        }
    }
    true
}

pub fn solve02(input: &[String]) -> Solutions {
    let safe_count_part_one = input
        .iter()
        .map(|line| string_to_vec_u64(line, ' '))
        .filter(|levels| is_safe(levels.clone()))
        .count();

    let safe_count_part_two = input
        .iter()
        .map(|line| string_to_vec_u64(line, ' '))
        .filter(|levels| {
            (0..levels.len()).any(|i| {
                let mut new_levels = levels.clone();
                new_levels.remove(i);
                is_safe(new_levels)
            })
        })
        .count();

    (
        Solution::USIZE(safe_count_part_one),
        Solution::USIZE(safe_count_part_two),
    )
}

use aoc::input::to_vec_u64;
use std::cmp::Ordering;

fn solve_part_1(entries: &[u64]) -> u64 {
    for i in 0..entries.len() {
        for j in (0..entries.len()).rev() {
            let sum = entries[i] + entries[j];

            match sum.cmp(&2020) {
                Ordering::Equal => return entries[i] * entries[j],
                // entries[i] + any remaining entries[j] will be less than 2020 so jump to the next bigger entries[i].
                Ordering::Less => break,
                Ordering::Greater => {}
            }
        }
    }
    unreachable!();
}

fn solve_part_2(entries: &[u64]) -> u64 {
    for i in 0..entries.len() {
        for j in (0..entries.len()).rev() {
            let sum = entries[i] + entries[j];
            if sum < 2020 {
                for k in 0..entries.len() {
                    if sum + entries[k] == 2020 {
                        return entries[i] * entries[j] * entries[k];
                    }
                }
            }
        }
    }
    unreachable!();
}

pub fn solve01(input: &[String]) -> (i128, i128) {
    let mut entries = to_vec_u64(input);
    entries.sort();
    (
        i128::from(solve_part_1(&entries)),
        i128::from(solve_part_2(&entries)),
    )
}

use aoc::solution::{Solution, Solutions};

fn solve_for(depths: &[u64]) -> i128 {
    depths
        .iter()
        .zip(depths.iter().skip(1))
        .filter(|(first, second)| first < second)
        .count() as i128
}

pub fn solve01(input: &[String]) -> Solutions {
    let depths: Vec<u64> = input
        .iter()
        .map(|line| line.parse::<u64>().unwrap())
        .collect();

    let sliding_depths: Vec<u64> = depths
        .iter()
        .zip(depths.iter().skip(1))
        .zip(depths.iter().skip(2))
        .map(|((first, second), third)| first + second + third)
        .collect();

    (
        Solution::I128(solve_for(&depths)),
        Solution::I128(solve_for(&sliding_depths)),
    )
}

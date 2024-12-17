use aoc::solution::{Solution, Solutions};

pub fn solve20(input: &[String]) -> Solutions {
    let target = input[0].parse::<usize>().unwrap();
    (
        Solution::USIZE(solve(target, usize::MAX, 10)),
        Solution::USIZE(solve(target, 50, 11)),
    )
}

fn solve(target: usize, elf_steps: usize, presents_multiplier: usize) -> usize {
    let mut presents_per_house: Vec<usize> = vec![0; target / 10];

    for elf in 1..target / 10 {
        for house in (elf..target / 10).step_by(elf).take(elf_steps) {
            presents_per_house[house] += elf * presents_multiplier;
        }
    }

    for (house, presents) in presents_per_house
        .iter()
        .enumerate()
        .take(target / 10)
        .skip(1)
    {
        if *presents >= target {
            return house;
        }
    }

    unreachable!();
}

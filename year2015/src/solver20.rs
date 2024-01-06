pub fn solve20(input: &[String]) -> (i128, i128) {
    let target = input[0].parse::<usize>().unwrap();
    (
        solve(target, usize::MAX, 10) as i128,
        solve(target, 50, 11) as i128,
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

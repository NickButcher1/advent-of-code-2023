fn solve_for(depths: &Vec<u64>) -> i128 {
    depths
        .iter()
        .zip(depths.iter().skip(1))
        .filter(|(first, second)| first < second)
        .count() as i128
}

pub fn solve01(input: &[String]) -> (i128, i128) {
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

    (solve_for(&depths), solve_for(&sliding_depths))
}

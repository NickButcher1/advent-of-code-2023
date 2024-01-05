pub fn solve20(input: &[String]) -> (i128, i128) {
    let target = input[0].parse::<usize>().unwrap();
    let mut presents_per_house: Vec<usize> = vec![0; target / 10];

    for elf in 1..target / 10 {
        for house in (elf..target / 10).step_by(elf) {
            presents_per_house[house] += elf * 10;
        }
    }

    let mut part_1 = 0;
    for house in 1..target / 10 {
        if presents_per_house[house] >= target {
            part_1 = house;
            break;
        }
    }

    (part_1 as i128, 0)
}

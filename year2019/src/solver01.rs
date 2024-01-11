use aoc::input::to_vec_i64;

fn fuel_for_mass(mass: i64) -> i64 {
    (mass / 3) - 2
}

fn fuel_for_masses(masses: &[i64]) -> Vec<i64> {
    masses
        .iter()
        .map(|mass| fuel_for_mass(*mass))
        .filter(|mass| *mass > 0)
        .collect()
}

pub fn solve01(input: &[String]) -> (i128, i128) {
    let mut fuels: Vec<i64> = fuel_for_masses(&to_vec_i64(input));
    let part_1: i64 = fuels.iter().sum();

    let mut part_2: i64 = part_1;
    while !fuels.is_empty() {
        fuels = fuel_for_masses(&fuels);
        part_2 += fuels.iter().sum::<i64>();
    }

    (part_1 as i128, part_2 as i128)
}

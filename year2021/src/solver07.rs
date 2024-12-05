use aoc::input::string_to_vec_i32;

pub fn solve07(input: &[String]) -> (i128, i128) {
    let positions = string_to_vec_i32(&input[0], ',');
    let max_position = *positions.iter().max().unwrap();

    let cheapest_fuel_cost_one = (0..max_position)
        .map(|i| {
            let fuel_cost = positions
                .iter()
                .map(|&position| (position - i).abs())
                .sum::<i32>();
            fuel_cost
        })
        .min()
        .unwrap();

    let cheapest_fuel_cost_two = (0..max_position)
        .map(|i| {
            let fuel_cost = positions
                .iter()
                .map(|&position| {
                    let n = (position - i).abs();
                    n * (n + 1) / 2
                })
                .sum::<i32>();
            fuel_cost
        })
        .min()
        .unwrap();

    (
        cheapest_fuel_cost_one as i128,
        cheapest_fuel_cost_two as i128,
    )
}

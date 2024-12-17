use aoc::input::string_to_vec_i32;
use aoc::solution::{Solution, Solutions};

pub fn solve07(input: &[String]) -> Solutions {
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
        Solution::I32(cheapest_fuel_cost_one),
        Solution::I32(cheapest_fuel_cost_two),
    )
}

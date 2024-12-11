use aoc::input::string_to_vec_u64;
use std::collections::HashMap;

fn tick_stone(stone: u64) -> (u64, u64) {
    if stone == 0 {
        (1, u64::MAX)
    } else {
        let stone_str = stone.to_string();
        if stone_str.len() % 2 == 0 {
            let half_len = stone_str.len() / 2;
            (
                (&stone_str[..half_len]).parse().unwrap(),
                (&stone_str[half_len..]).parse().unwrap(),
            )
        } else {
            (stone * 2024, u64::MAX)
        }
    }
}

pub fn solve_one(input: &[String], depth: u64, cache_one: &mut HashMap<u64, Vec<u64>>) -> u64 {
    let mut input_stones = string_to_vec_u64(&input[0], ' ');

    let mut stones: HashMap<u64, usize> =
        input_stones.iter().fold(HashMap::new(), |mut map, &num| {
            *map.entry(num).or_insert(0) += 1;
            map
        });

    for blink in 1..=depth {
        let mut new_stones: HashMap<u64, usize> = HashMap::new();

        for (stone, count) in stones {
            let mut add_stones: Vec<u64> = if cache_one.contains_key(&stone) {
                cache_one[&stone].clone()
            } else if stone == 0 {
                cache_one.insert(0, vec![1]);
                vec![1]
            } else {
                let stone_str = stone.to_string();
                if stone_str.len() % 2 == 0 {
                    let half_len = stone_str.len() / 2;
                    let new_1 = (&stone_str[..half_len]).parse().unwrap();
                    let new_2 = (&stone_str[half_len..]).parse().unwrap();
                    cache_one.insert(stone, vec![new_1, new_2]);
                    vec![new_1, new_2]
                } else {
                    cache_one.insert(stone, vec![stone * 2024]);
                    vec![stone * 2024]
                }
            };
            for add_s in add_stones {
                *new_stones.entry(add_s).or_insert(0) += count;
            }
        }
        stones = new_stones;
    }

    stones.values().fold(0, |acc, &x| acc + x as u64)
}

pub fn solve11(input: &[String]) -> (i128, i128) {
    let mut cache_one: HashMap<u64, Vec<u64>> = HashMap::new();
    let solution_one = solve_one(input, 25, &mut cache_one);
    let solution_two = solve_one(input, 75, &mut cache_one);
    (solution_one as i128, solution_two as i128)
}

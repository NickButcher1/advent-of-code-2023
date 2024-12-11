use aoc::input::string_to_vec_u64;
use std::collections::HashMap;

fn tick_stone(stone: u64, cache: &mut HashMap<u64, Vec<u64>>) -> Vec<u64> {
    if stone == 0 {
        cache.insert(0, vec![1]);
        vec![1]
    } else {
        let stone_str = stone.to_string();
        if stone_str.len() % 2 == 0 {
            let half_len = stone_str.len() / 2;
            let new_1 = (&stone_str[..half_len]).parse().unwrap();
            let new_2 = (&stone_str[half_len..]).parse().unwrap();
            cache.insert(stone, vec![new_1, new_2]);
            vec![new_1, new_2]
        } else {
            cache.insert(stone, vec![stone * 2024]);
            vec![stone * 2024]
        }
    }
}

pub fn solve(input: &[String], depth: u64, cache: &mut HashMap<u64, Vec<u64>>) -> u64 {
    let input_stones = string_to_vec_u64(&input[0], ' ');

    let mut stones: HashMap<u64, usize> =
        input_stones.iter().fold(HashMap::new(), |mut map, &num| {
            *map.entry(num).or_insert(0) += 1;
            map
        });

    for _ in 1..=depth {
        let mut new_stones: HashMap<u64, usize> = HashMap::new();

        for (stone, count) in &stones {
            let stones_to_add: Vec<u64> = if cache.contains_key(&stone) {
                cache[&stone].clone()
            } else {
                tick_stone(*stone, cache)
            };
            for stone_to_add in stones_to_add {
                *new_stones.entry(stone_to_add).or_insert(0) += count;
            }
        }

        stones = new_stones;
    }

    stones.values().fold(0, |acc, &x| acc + x as u64)
}

pub fn solve11(input: &[String]) -> (i128, i128) {
    let mut cache: HashMap<u64, Vec<u64>> = HashMap::new();
    let solution_one = solve(input, 25, &mut cache);
    let solution_two = solve(input, 75, &mut cache);
    (solution_one as i128, solution_two as i128)
}

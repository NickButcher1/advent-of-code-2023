use crate::common::string_to_vec_u64_ignore_prefix;
use std::cmp;

pub fn solve05(input: Vec<String>) -> (i128, i128) {
    let mut seeds: Vec<u64> = Vec::new();
    let mut maps: Vec<Vec<Vec<u64>>> = vec![vec![vec![]]];
    let mut current_map_id: i32 = -1;

    for line in input {
        if line.starts_with("seeds:") {
            seeds = string_to_vec_u64_ignore_prefix("seeds:", &line);
        } else if line.contains(" map:") {
            current_map_id += 1;
            maps.push(Vec::new());
            maps[current_map_id as usize] = Vec::new();
        } else if !line.is_empty() {
            let block: Vec<u64> = line.split(' ').map(|x| x.parse::<u64>().unwrap()).collect();
            maps[current_map_id as usize].push(block);
        }
    }

    // Part 1.
    let mut min_location: u64 = 999999999999999;
    for seed in &seeds {
        min_location = cmp::min(seed_to_location(*seed, &maps), min_location)
    }

    // Part 2.
    let mut part_2_location = 0;
    let mut found = false;
    maps.reverse();

    while !found {
        part_2_location += 1;
        found = try_location(&seeds, part_2_location, &maps);
    }

    (min_location as i128, part_2_location as i128)
}

fn map_a_to_b(value: u64, map: &Vec<Vec<u64>>) -> u64 {
    for block in map {
        let dest = block[0];
        let src = block[1];
        let block_len = block[2];
        if value >= src && value <= (src + block_len - 1) {
            return value + dest - src;
        }
    }
    value
}

fn seed_to_location(seed: u64, maps: &Vec<Vec<Vec<u64>>>) -> u64 {
    let mut current_value = seed;
    for map in maps {
        current_value = map_a_to_b(current_value, map)
    }
    current_value
}

fn map_b_to_a(value: u64, map: &Vec<Vec<u64>>) -> u64 {
    for block in map {
        let dest = block[0];
        let src = block[1];
        let block_len = block[2];

        if value >= dest && value <= (dest + block_len - 1) {
            return value - dest + src;
        }
    }

    value
}

fn try_location(seeds: &Vec<u64>, location: u64, maps: &Vec<Vec<Vec<u64>>>) -> bool {
    let mut current_value = location;
    for map in maps {
        current_value = map_b_to_a(current_value, map);
    }

    test_valid_seed(seeds, current_value)
}

fn test_valid_seed(seeds: &Vec<u64>, value: u64) -> bool {
    let mut valid_seed = false;

    for i in (0..seeds.len()).step_by(2) {
        let first_seed = seeds[i];
        let block_len = seeds[i + 1];

        if value >= first_seed && value <= (first_seed + block_len - 1) {
            valid_seed = true;
            break;
        }
    }

    valid_seed
}

use aoc::common::string_to_vec_u64_ignore_prefix;
use std::cmp;

struct RangeMap {
    destination_range_start: u64,
    source_range_start: u64,
    len: u64,
}

type Map = Vec<RangeMap>;
type Maps = Vec<Map>;

pub fn solve05(input: &[String]) -> (i128, i128) {
    let mut seeds: Vec<u64> = Vec::new();
    let mut maps: Maps = vec![vec![]];
    let mut current_map_id: i32 = -1;

    for line in input {
        if line.starts_with("seeds:") {
            seeds = string_to_vec_u64_ignore_prefix("seeds:", line);
        } else if line.contains(" map:") {
            current_map_id += 1;
            maps.push(Vec::new());
            maps[current_map_id as usize] = Vec::new();
        } else if !line.is_empty() {
            let block: Vec<u64> = line.split(' ').map(|x| x.parse::<u64>().unwrap()).collect();
            let range = RangeMap {
                destination_range_start: block[0],
                source_range_start: block[1],
                len: block[2],
            };
            maps[current_map_id as usize].push(range);
        }
    }

    // Part 1.
    let mut min_location: u64 = 999_999_999_999_999;
    for seed in &seeds {
        min_location = cmp::min(seed_to_location(*seed, &maps), min_location);
    }

    // Part 2.
    let mut part_2_location = 0;
    let mut found = false;
    maps.reverse();

    while !found {
        part_2_location += 1;
        found = is_valid_location(&seeds, part_2_location, &maps);
    }

    (min_location as i128, part_2_location as i128)
}

fn map_a_to_b(value: u64, map: &Map) -> u64 {
    for range_map in map {
        if value >= range_map.source_range_start
            && value <= (range_map.source_range_start + range_map.len - 1)
        {
            return value + range_map.destination_range_start - range_map.source_range_start;
        }
    }
    value
}

fn seed_to_location(seed: u64, maps: &Maps) -> u64 {
    let mut current_value = seed;
    for range_map in maps {
        current_value = map_a_to_b(current_value, range_map);
    }
    current_value
}

fn map_b_to_a(value: u64, map: &Map) -> u64 {
    for range_map in map {
        if value >= range_map.destination_range_start
            && value <= (range_map.destination_range_start + range_map.len - 1)
        {
            return value - range_map.destination_range_start + range_map.source_range_start;
        }
    }

    value
}

fn is_valid_location(seeds: &Vec<u64>, location: u64, maps: &Maps) -> bool {
    let mut current_value = location;
    for range_map in maps {
        current_value = map_b_to_a(current_value, range_map);
    }

    is_valid_seed(seeds, current_value)
}

fn is_valid_seed(seeds: &Vec<u64>, seed_to_test: u64) -> bool {
    let mut valid_seed = false;

    for i in (0..seeds.len()).step_by(2) {
        let first_seed = seeds[i];
        let block_len = seeds[i + 1];

        if seed_to_test >= first_seed && seed_to_test <= (first_seed + block_len - 1) {
            valid_seed = true;
            break;
        }
    }

    valid_seed
}

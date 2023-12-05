use std::cmp;

pub fn solve05(input: Vec<String>) -> (i128, i128) {
    let total_part_2 = 0;

    let mut seeds: Vec<u64> = Vec::new();
    let mut maps: Vec<Vec<Vec<u64>>> = vec![vec![vec![]]];
    let mut current_map_id: i32 = -1;

    for line in input {
        if line.starts_with("seeds:") {
            let split_1: Vec<&str> = line.split("seeds: ").collect();
            seeds = split_1[1]
                .split(' ')
                .map(|x| x.parse::<u64>().unwrap())
                .collect();
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
    for seed in seeds {
        min_location = cmp::min(seed_to_location(seed, &maps), min_location)
    }

    (min_location as i128, total_part_2 as i128)
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

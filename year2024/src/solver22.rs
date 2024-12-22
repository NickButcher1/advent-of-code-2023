use aoc::input::to_vec_u64;
use aoc::solution::{Solution, Solutions};
use std::collections::{HashMap, HashSet};

// The module clears the top bytes, leaving the bottom three bytes alone.
const MODULO: u64 = 16777216;

fn evolve_once(input: u64) -> u64 {
    let x = (input ^ (input << 6)) % MODULO;
    let y = (x ^ (x >> 5)) % MODULO;
    (y ^ (y << 11)) % MODULO
}

pub fn solve22(input: &[String]) -> Solutions {
    let initial_starting_secrets = to_vec_u64(input);

    // Iterate over every Buyer, and then over every sequence/price combination for that buyer.
    // Store them all in a HashMap. Then it's just a case of finding the highest value.
    // Take care to only store the first occurrence for each buyer!
    let mut total_price_for_sequence: HashMap<(i32, i32, i32, i32), i32> = HashMap::new();
    let mut solution_one = 0;

    for initial_starting_secret in initial_starting_secrets {
        let mut secret = initial_starting_secret;
        let mut prev_price = (initial_starting_secret % 10) as i32;
        let mut price_diffs: Vec<i32> = vec![0];
        let mut sequences_for_this_buyer: HashSet<(i32, i32, i32, i32)> = HashSet::new();

        for i in 1..=2000 {
            secret = evolve_once(secret);
            let price = (secret % 10) as i32;
            price_diffs.push(price - prev_price);

            // Ignore comparison until we have four diffs.
            if i >= 4 {
                let sequence = (
                    price_diffs[i - 3],
                    price_diffs[i - 2],
                    price_diffs[i - 1],
                    price_diffs[i],
                );

                if !sequences_for_this_buyer.contains(&sequence) {
                    sequences_for_this_buyer.insert(sequence);
                    *total_price_for_sequence.entry(sequence).or_insert(0) += price;
                }
            }

            prev_price = price;
        }
        solution_one += secret;
    }

    (
        Solution::U64(solution_one),
        Solution::I32(*total_price_for_sequence.values().max().unwrap()),
    )
}

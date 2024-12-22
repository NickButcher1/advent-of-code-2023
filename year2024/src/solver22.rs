use aoc::input::to_vec_u64;
use aoc::solution::{Solution, Solutions};
use std::collections::{HashMap, HashSet};

struct Buyer {
    prices: [i32; 2001],
    price_diffs: [i32; 2001],
}

fn evolve_once(input: u64) -> u64 {
    let a = input ^ (input * 64);
    let b = a % 16777216;
    let c = b ^ (b / 32);
    let d = c % 16777216;
    let e = d ^ (d * 2048);
    e % 16777216
}

pub fn solve22(input: &[String]) -> Solutions {
    let initial_starting_secret = to_vec_u64(input);

    let mut solution_one = 0;

    let mut buyers: Vec<Buyer> = vec![];

    for initial_starting_secret in initial_starting_secret {
        let mut secret_numbers = vec![initial_starting_secret];
        let mut prices: Vec<i32> = vec![(initial_starting_secret % 10) as i32];
        let mut price_diffs: Vec<i32> = vec![0];
        let mut secret = initial_starting_secret;

        for _i in 1..=2000 {
            secret = evolve_once(secret);
            let price = (secret % 10) as i32;
            price_diffs.push(price - prices.last().unwrap());
            prices.push(price);
            secret_numbers.push(secret);
        }
        solution_one += secret;

        buyers.push(Buyer {
            prices: prices.try_into().unwrap(),
            price_diffs: price_diffs.try_into().unwrap(),
        });
    }

    // Iterate over every Buyer, and then over every sequence/price combination for that buyer.
    // Store them all in a HashMap. Then it's just a case of finding the highest value.
    // Take care to only store the first occurrence for each buyer!
    let mut total_price_for_sequence: HashMap<(i32, i32, i32, i32), i32> = HashMap::new();

    for buyer in &buyers {
        let mut sequences_for_this_buyer: HashSet<(i32, i32, i32, i32)> = HashSet::new();
        // Diffs start at first index because there is no diff for the first secret number.
        // Last sequence is from 1997-2000.
        for i in 1..=1997 {
            // Sequence is from i to i+3. Price is at i+3.
            let sequence = (
                buyer.price_diffs[i],
                buyer.price_diffs[i + 1],
                buyer.price_diffs[i + 2],
                buyer.price_diffs[i + 3],
            );
            if !sequences_for_this_buyer.contains(&sequence) {
                sequences_for_this_buyer.insert(sequence);
                *total_price_for_sequence.entry(sequence).or_insert(0) += buyer.prices[i + 3];
            }
        }
    }

    (
        Solution::U64(solution_one),
        Solution::I32(*total_price_for_sequence.values().max().unwrap()),
    )
}

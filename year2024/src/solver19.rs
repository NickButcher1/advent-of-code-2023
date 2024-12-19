use aoc::solution::{Solution, Solutions};
use std::cmp::min;
use std::collections::{HashMap, HashSet};

fn count_ways<'a>(
    cache: &mut HashMap<&'a str, i64>,
    towels: &HashSet<&str>,
    max_towel_len: usize,
    fragment: &'a str,
) -> i64 {
    let cached_num_ways = cache.get(fragment);
    if cached_num_ways.is_some() {
        let new_num_ways = cached_num_ways.unwrap();
        return *new_num_ways;
    }

    let mut this_num_ways = 0;
    let min_match_len = 1;
    let max_match_len = min(fragment.len(), max_towel_len); // inclusive

    for j in min_match_len..=max_match_len {
        let try_fragment = &fragment[0..j];
        let new_untested_fragment = &fragment[j..];

        if towels.contains(&try_fragment) {
            if new_untested_fragment.is_empty() {
                this_num_ways += 1;
            } else {
                let new_ways = count_ways(cache, towels, max_towel_len, new_untested_fragment);
                this_num_ways += new_ways;
            }
        }
    }

    cache.insert(fragment, this_num_ways);

    this_num_ways
}

pub fn solve19(input: &[String]) -> Solutions {
    // TODO Put this in input.rs.
    let towels: HashSet<&str> = input[0].split(", ").filter(|s| !s.is_empty()).collect();
    let max_towel_len = towels.iter().map(|s| s.len()).max().unwrap_or(0);
    let designs: Vec<&String> = input.iter().skip(2).collect();

    let mut total_num_possible = 0;
    let mut total_num_ways = 0;
    let mut cache: HashMap<&str, i64> = HashMap::new();

    for design in designs {
        let this_design_ways = count_ways(&mut cache, &towels, max_towel_len, design);
        total_num_ways += this_design_ways;
        if this_design_ways != 0 {
            total_num_possible += 1;
        }
    }

    (
        Solution::I64(total_num_possible),
        Solution::I64(total_num_ways),
    )
}

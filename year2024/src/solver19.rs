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

    let max_match_len = min(fragment.len(), max_towel_len);
    let this_num_ways = (1..=max_match_len)
        .filter_map(|j| {
            let (try_fragment, new_untested_fragment) = fragment.split_at(j);

            towels.contains(try_fragment).then(|| {
                if new_untested_fragment.is_empty() {
                    1
                } else {
                    count_ways(cache, towels, max_towel_len, new_untested_fragment)
                }
            })
        })
        .sum();

    cache.insert(fragment, this_num_ways);

    this_num_ways
}

pub fn solve19(input: &[String]) -> Solutions {
    let towels: HashSet<&str> = input[0].split(", ").filter(|s| !s.is_empty()).collect();
    let max_towel_len = towels.iter().map(|s| s.len()).max().unwrap_or(0);
    let designs: Vec<&String> = input.iter().skip(2).collect();

    let mut cache: HashMap<&str, i64> = HashMap::new();

    let (total_num_possible, total_num_ways) = designs
        .iter()
        .map(|design| count_ways(&mut cache, &towels, max_towel_len, design))
        .fold((0, 0), |(possible, ways), this_design_ways| {
            (
                possible + (this_design_ways > 0) as i32,
                ways + this_design_ways,
            )
        });

    (
        Solution::I32(total_num_possible),
        Solution::I64(total_num_ways),
    )
}

use aoc::solution::{Solution, Solutions};
use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;

fn read_polymer_from_input(input: &str) -> HashMap<String, usize> {
    let pairs: Vec<String> = input
        .chars()
        .collect::<Vec<char>>()
        .windows(2)
        .map(|pair| pair.iter().collect())
        .collect();

    let pair_counts = pairs.iter().fold(HashMap::new(), |mut acc, pair| {
        *acc.entry(pair.clone()).or_insert(0) += 1;
        acc
    });

    pair_counts
}

pub fn read_transforms_from_input(input: &[String]) -> HashMap<String, Vec<String>> {
    let re = Regex::new(r"([A-Z])([A-Z]) -> ([A-Z])$").unwrap();

    let mut transforms = HashMap::new();
    input.iter().skip(2).for_each(|line| {
        let captures = re.captures(line).unwrap();
        let key_1 = captures.get(1).unwrap().as_str().parse::<char>().unwrap();
        let key_2 = captures.get(2).unwrap().as_str().parse::<char>().unwrap();
        let new_char = captures.get(3).unwrap().as_str().parse::<char>().unwrap();
        transforms.insert(
            format!("{key_1}{key_2}"),
            vec![format!("{key_1}{new_char}"), format!("{new_char}{key_2}")],
        );
    });
    transforms
}

fn transform_polymer(
    polymer: HashMap<String, usize>,
    transforms: &HashMap<String, Vec<String>>,
) -> HashMap<String, usize> {
    let mut new_polymer: HashMap<String, usize> = HashMap::new();

    for key in polymer.keys() {
        for new_key in &transforms[key] {
            let old_count = polymer.get(key).unwrap_or(&0);
            *new_polymer.entry(new_key.to_string()).or_insert(0) += old_count;
        }
    }

    new_polymer
}

pub fn solve(input: &[String], depth: usize) -> usize {
    let mut polymer = read_polymer_from_input(&input[0]);
    let transforms = read_transforms_from_input(input);

    for _ in 1..=depth {
        polymer = transform_polymer(polymer, &transforms);
    }

    let mut chars: HashMap<char, usize> = HashMap::new();

    for (p, count) in polymer {
        for c in p.chars() {
            *chars.entry(c).or_insert(0) += count;
        }
    }

    // All characters are double counted because they are each part of two character pairs, except
    // for the first and last characters. So add one to those two,  then half all values.
    let original_polymer = &input[0];
    let first_char_of_polymer = original_polymer.chars().next().unwrap();
    let last_char_of_polymer = original_polymer.chars().last().unwrap();

    *chars.entry(first_char_of_polymer).or_insert(0) += 1;
    *chars.entry(last_char_of_polymer).or_insert(0) += 1;

    for count in chars.values_mut() {
        *count /= 2;
    }

    let mut values = chars.values().sorted();
    let first_value = values.next().unwrap();
    values.last().unwrap() - first_value
}

pub fn solve14(input: &[String]) -> Solutions {
    (
        Solution::USIZE(solve(input, 10)),
        Solution::USIZE(solve(input, 40)),
    )
}

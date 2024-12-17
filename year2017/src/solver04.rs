use aoc::solution::{Solution, Solutions};
use std::collections::HashSet;

pub fn solve04(input: &[String]) -> Solutions {
    let part_1 = input.iter().fold(0, |acc, line| {
        let strings_list: Vec<&str> = line.split(' ').collect();
        let strings_set: HashSet<&str> = strings_list.clone().into_iter().collect();
        if strings_list.len() == strings_set.len() {
            acc + 1
        } else {
            acc
        }
    });

    let part_2 = input.iter().fold(0, |acc, line| {
        let strings_list_unsorted: Vec<&str> = line.split(' ').collect();
        let strings_list: Vec<String> = strings_list_unsorted
            .clone()
            .into_iter()
            .map(|s| {
                let mut chars: Vec<char> = s.chars().collect();
                chars.sort_unstable();
                chars.into_iter().collect()
            })
            .collect();
        let strings_set: HashSet<String> = strings_list.clone().into_iter().collect();
        if strings_list.len() == strings_set.len() {
            acc + 1
        } else {
            acc
        }
    });

    (Solution::I32(part_1), Solution::I32(part_2))
}

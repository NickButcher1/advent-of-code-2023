use aoc::solution::{Solution, Solutions};
use itertools::Itertools;
use regex::Regex;
use std::collections::{HashMap, HashSet};

struct Computer<'a> {
    name: &'a str,
    is_t: bool,
    to: Vec<&'a str>,
}

fn parse_input(input: &[String]) -> HashMap<&str, Computer> {
    let re = Regex::new(r"^([a-z][a-z])-([a-z][a-z])$").unwrap();
    let mut computers: HashMap<&str, Computer> = HashMap::new();

    for line in input {
        let capture = re.captures(line).unwrap();
        let c1 = capture.get(1).unwrap().as_str();
        let c2 = capture.get(2).unwrap().as_str();

        computers
            .entry(c1)
            .and_modify(|computer| computer.to.push(c2))
            .or_insert(Computer {
                name: c1,
                is_t: c1.starts_with('t'),
                to: vec![c2],
            });

        computers
            .entry(c2)
            .and_modify(|computer| computer.to.push(c1))
            .or_insert(Computer {
                name: c2,
                is_t: c2.starts_with('t'),
                to: vec![c1],
            });
    }

    computers
}

pub fn solve23(input: &[String]) -> Solutions {
    let computers = parse_input(input);

    // Build a list of all the sets of three computers that are all connected to each other.
    // For part one, the solution is the number of those sets that contain at least one name
    // beginning with 't'.
    //
    // To ensure unique sets, only select computer names in alphabetical order.
    let mut solution_one = 0;
    let mut all_connected: Vec<HashSet<&str>> = vec![];

    for c1 in computers.values() {
        for c2_name in &c1.to {
            let c2 = computers.get(c2_name).unwrap();
            if c2.name > c1.name {
                for c3_name in &c2.to {
                    if *c3_name > c2.name {
                        let c3 = computers.get(c3_name).unwrap();
                        if c3.to.contains(&c1.name) {
                            if c1.is_t || c2.is_t || c3.is_t {
                                solution_one += 1;
                            }
                            let mut all_connected_set = HashSet::new();
                            all_connected_set.insert(c1.name);
                            all_connected_set.insert(c2.name);
                            all_connected_set.insert(c3.name);
                            all_connected.push(all_connected_set);
                        }
                    }
                }
            }
        }
    }

    // For part two, attempt to extend each set with further computers until all sets are complete.
    // Each loop attempts to extend each existing set with one further computer. If a loop makes no
    // additions we know that all sets are complete (and there should be only one set at this
    // point).
    loop {
        let mut new_all_connected: Vec<HashSet<&str>> = vec![];
        for all_connected_set in &mut all_connected {
            // Try adding every computer that isn't already in the set. Stop as soon as we find one
            // we can add.
            for key in computers.keys() {
                if !all_connected_set.contains(key) {
                    let mut all_matched = true;
                    for test_key in all_connected_set.iter() {
                        let test_computer = computers.get(test_key).unwrap();
                        if !test_computer.to.contains(key) {
                            all_matched = false;
                            break;
                        }
                    }
                    if all_matched {
                        all_connected_set.insert(key);
                        if !new_all_connected.contains(all_connected_set) {
                            new_all_connected.push(all_connected_set.clone());
                        }
                        break;
                    }
                }
            }
        }

        if new_all_connected.is_empty() {
            break;
        }
        all_connected = new_all_connected;
    }

    let solution_two = all_connected[0]
        .iter()
        .collect::<Vec<_>>()
        .into_iter()
        .sorted()
        .join(",");

    (Solution::U32(solution_one), Solution::STR(solution_two))
}

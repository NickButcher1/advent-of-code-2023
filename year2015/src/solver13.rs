use aoc::solution::{Solution, Solutions};
use itertools::Itertools;
use regex::Regex;
use std::collections::{HashMap, HashSet};

const ME: &str = "ME";

pub fn solve13(input: &[String]) -> Solutions {
    let mut mapping: HashMap<(&str, &str), isize> = HashMap::new();
    let mut names_set: HashSet<&str> = HashSet::new();
    let re =
        Regex::new(r"^(\w+) would (\w+) (\d+) happiness units by sitting next to (\w+).$").unwrap();

    for line in input {
        let captures = re.captures(line).unwrap();
        let person = captures.get(1).unwrap().as_str();
        let is_gain = captures.get(2).unwrap().as_str() == "gain";
        let units = captures.get(3).unwrap().as_str().parse::<isize>().unwrap();
        let other_person = captures.get(4).unwrap().as_str();
        let happiness_delta = if is_gain { units } else { 0 - units };
        mapping.insert((person, other_person), happiness_delta);
        names_set.insert(person);
    }
    let mut names: Vec<&str> = names_set.into_iter().collect();

    let part_1 = solve(&names, &mapping) as i128;

    names.push(ME);
    for name in &names {
        mapping.insert((ME, name), 0);
        mapping.insert((name, ME), 0);
    }

    let part_2 = solve(&names, &mapping) as i128;

    (Solution::I128(part_1), Solution::I128(part_2))
}

fn solve(names: &[&str], mapping: &HashMap<(&str, &str), isize>) -> isize {
    // Omit one name, we'll fix it in one position - this is more efficient than trying that name in every position.
    let mut part_1 = isize::MIN;
    let permutations = names[1..].iter().permutations(names.len() - 1);
    for p in permutations {
        let mut happiness = 0;
        for i in 0..p.len() - 1 {
            happiness += mapping[&(*p[i], *p[i + 1])] + mapping[&(*p[i + 1], *p[i])];
        }
        happiness += mapping[&(names[0], *p[0])] + mapping[&(*p[0], names[0])];
        happiness += mapping[&(names[0], *p[p.len() - 1])] + mapping[&(*p[p.len() - 1], names[0])];

        if happiness > part_1 {
            part_1 = happiness;
        }
    }
    part_1
}

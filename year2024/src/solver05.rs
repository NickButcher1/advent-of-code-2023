use aoc::input::string_to_vec_u32;
use regex::Regex;

fn read_input(input: &[String]) -> (Vec<(u32, u32)>, Vec<Vec<u32>>) {
    let re_rules = Regex::new(r"^(\d+)\|(\d+)$").unwrap();

    let mut read_rules = true;
    let mut rules: Vec<(u32, u32)> = vec![];
    let mut updates: Vec<Vec<u32>> = vec![];

    for line in input {
        if read_rules {
            if line.is_empty() {
                read_rules = false;
            } else {
                let captures = re_rules.captures(line).unwrap();
                rules.push((
                    captures.get(1).unwrap().as_str().parse::<u32>().unwrap(),
                    captures.get(2).unwrap().as_str().parse::<u32>().unwrap(),
                ))
            }
        } else {
            updates.push(string_to_vec_u32(line, ','));
        }
    }

    (rules, updates)
}
pub fn solve05(input: &[String]) -> (i128, i128) {
    let (rules, updates) = read_input(input);

    let (valid_updates, invalid_updates): (Vec<_>, Vec<_>) =
        updates.into_iter().partition(|update| {
            rules.iter().all(|(rule_p1, rule_p2)| {
                if !update.contains(rule_p1) || !update.contains(rule_p2) {
                    return true;
                }

                let p1_pos = update
                    .iter()
                    .position(|&x| x == *rule_p1)
                    .unwrap_or(usize::MAX);
                let p2_pos = update
                    .iter()
                    .position(|&x| x == *rule_p2)
                    .unwrap_or(usize::MAX);

                p1_pos < p2_pos
            })
        });

    let solution_one = valid_updates
        .iter()
        .map(|update| update[(update.len() - 1) / 2])
        .sum::<u32>();

    let solution_two = invalid_updates
        .iter()
        .map(|update| clean_update(update, &rules))
        .map(|cleaned_update| cleaned_update[(cleaned_update.len() - 1) / 2])
        .sum::<u32>();

    (solution_one as i128, solution_two as i128)
}

// Recursive function to sort a vec according to the rules.
// Put the first input item in a middle group, then use the rules to put all other input items into
// either a left group or right group.
// If the left or right group has multiple elements, then recursively split those.
fn clean_update(input_vec: &[u32], rules: &Vec<(u32, u32)>) -> Vec<u32> {
    assert!(input_vec.len() >= 2);
    let split_on_page = input_vec[0];
    let mid_group: Vec<u32> = vec![split_on_page];
    let mut left_group: Vec<u32> = vec![];
    let mut right_group: Vec<u32> = vec![];

    for page in input_vec.iter().skip(1) {
        for (rule_p1, rule_p2) in rules {
            if input_vec.contains(rule_p1) && input_vec.contains(rule_p2) {
                if split_on_page == *rule_p1 && *page == *rule_p2 {
                    right_group.push(*page)
                } else if split_on_page == *rule_p2 && *page == *rule_p1 {
                    left_group.push(*page);
                }
            }
        }
    }

    if left_group.len() >= 2 {
        left_group = clean_update(&left_group, rules);
    }
    if right_group.len() >= 2 {
        right_group = clean_update(&right_group, rules);
    }
    let flattened_output: Vec<u32> = vec![left_group, mid_group, right_group]
        .into_iter()
        .flatten()
        .collect();
    flattened_output
}

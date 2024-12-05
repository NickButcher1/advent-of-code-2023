use regex::Regex;
use aoc::input::string_to_vec_u32;

pub fn solve05(input: &[String]) -> (i128, i128) {
    let re_rules = Regex::new(r"^(\d+)\|(\d+)$").unwrap();

    let mut read_rules = true;
    let mut rules: Vec<(u32,u32)> = vec![];
    let mut updates: Vec<Vec<u32>> = vec![];
    let mut invalid_updates: Vec<Vec<u32>> = vec![];

    for line in input {
        if read_rules {
            if line.is_empty() {
                read_rules = false;
            } else {
                let captures = re_rules.captures(line).unwrap();
                rules.push(
                    (
                        captures.get(1).unwrap().as_str().parse::<u32>().unwrap(),
                        captures.get(2).unwrap().as_str().parse::<u32>().unwrap(),
                    )
                )
            }
        } else {
            updates.push(string_to_vec_u32(line, ','));
        }
    }

    // println!("RULES: {rules:?}");
    // println!("UPDATES: {updates:?}");

    let mut valid_middle_pages: Vec<u32> = vec![];
    for update in updates {
        let mut is_update_valid = true;
        for (rule_p1, rule_p2) in &rules {
            if update.contains(&rule_p1) && update.contains(&rule_p2) {
                let p1_pos_in_update = update.iter().position(|&x| x == *rule_p1).unwrap_or_else(|| usize::MAX);
                let p2_pos_in_update = update.iter().position(|&x| x == *rule_p2).unwrap_or_else(|| usize::MAX);
                if p1_pos_in_update > p2_pos_in_update {
                    is_update_valid = false;
                    break;
                }
            }
        }
        if is_update_valid {
            let middle_index = (update.len() - 1) / 2;
            valid_middle_pages.push(update[middle_index]);
            // println!("UPDATE: {update:?} VALID, MIDDLE INDEX: {middle_index} ");
        } else {
            invalid_updates.push(update.clone());
            // println!("UPDATE: {update:?} NOT VALID ");
        }
    }

    let solution_one = valid_middle_pages.iter().sum::<u32>();

    // PART TWO

    // Can I brute force it?
    // 78 invalid updates.
    let mut valid_middle_pages_two: Vec<u32> = vec![];
    for update in invalid_updates {
        println!("INVALID UPDATE: {update:?}");
        let cleaned_update = clean_update(&update, &rules);
        println!("    {cleaned_update:?}");

        let middle_index = (cleaned_update.len() - 1) / 2;
        println!("UPDATE: {update:?} VALID, MIDDLE INDEX: {middle_index} ");
        valid_middle_pages_two.push(cleaned_update[middle_index]);
    }

    let solution_two = valid_middle_pages_two.iter().sum::<u32>();

    (solution_one as i128, solution_two as i128)
}

// Split inta vec into three Vecs. Put the first item in its own vec, then put all the others in
// either a before group or after group, according to the rules.
fn clean_update(input_vec: &Vec<u32>, rules: &Vec<(u32,u32)>) -> Vec<u32> {
    assert!(input_vec.len() >= 2);
    let split_on_page = input_vec[0];
    let mid_group: Vec<u32> = vec![split_on_page];
    let mut left_group: Vec<u32> = vec![];
    let mut right_group: Vec<u32> = vec![];

    for (i, page) in input_vec.iter().skip(1).enumerate() {
        for (rule_p1, rule_p2) in rules {
            // println!("  TRY PAGE {page} with rule {rule_p1}|{rule_p2}");
            if input_vec.contains(&rule_p1) && input_vec.contains(&rule_p2) {
                // This rule applies.
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
    println!("    Return L  {left_group:?}");
    println!("    Return M  {mid_group:?}");
    println!("    Return R  {right_group:?}");
    let flattened_vec: Vec<u32> = vec![left_group, mid_group, right_group].into_iter().flat_map(|v| v).collect();
    println!("    Flattened {flattened_vec:?}");
    return flattened_vec;
}

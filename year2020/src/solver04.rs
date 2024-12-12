use regex::Regex;
use std::collections::HashMap;

pub fn solve(input: &[String], is_part_two: bool) -> usize {
    let re = Regex::new(r"(?m)(cid|hcl|iyr|eyr|ecl|pid|byr|hgt):(\S+)").unwrap();
    let inputs: Vec<&[String]> = input.split(|s| s.is_empty()).collect();

    inputs
        .iter()
        .filter_map(|input| {
            let passport: HashMap<_, _> = input
                .iter()
                .flat_map(|line| re.captures_iter(line))
                .map(|cap| (cap.get(1).unwrap().as_str(), cap.get(2).unwrap().as_str()))
                .collect();

            if passport.contains_key("byr")
                && passport.contains_key("iyr")
                && passport.contains_key("eyr")
                && passport.contains_key("hgt")
                && passport.contains_key("hcl")
                && passport.contains_key("ecl")
                && passport.contains_key("pid")
            {
                if !is_part_two || (true) {
                    println!("ACCEPT: {passport:?}");
                    Some(passport)
                } else {
                    println!("REJECT: {passport:?}");
                    None
                }
            } else {
                println!("REJECT: {passport:?}");
                None
            }
        })
        .count()
}

pub fn solve04(input: &[String]) -> (i128, i128) {
    (solve(input, false) as i128, solve(input, true) as i128)
}

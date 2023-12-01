fn solve_part_1(input: Vec<String>) -> i128 {
    let mut numbers = Vec::new();
    let mut first_digit = 0;
    let mut last_digit = 0;

    for line in input {
        for c in line.chars() {
            if c.is_ascii_digit() {
                first_digit = c.to_digit(10).unwrap();
                break;
            }
        }

        for c in line.chars().rev() {
            if c.is_ascii_digit() {
                last_digit = c.to_digit(10).unwrap();
                break;
            }
        }

        numbers.push(first_digit * 10 + last_digit);
    }
    let total: u32 = numbers.iter().sum();
    total as i128
}

fn get_first_digit_from_substring(substring: &str, number_map: &Vec<(&str, u32)>) -> u32 {
    for (number_str, number_int) in number_map {
        if substring.starts_with(*number_str) {
            return *number_int;
        }
    }

    0
}

fn get_last_digit_from_substring(substring: &str, number_map: &Vec<(&str, u32)>) -> u32 {
    for (number_str, number_int) in number_map {
        if substring.ends_with(*number_str) {
            return *number_int;
        }
    }

    0
}

fn solve_part_2(input: Vec<String>) -> i128 {
    let number_map = vec![
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
    ];

    let mut numbers = Vec::new();
    let mut first_digit = 0;
    let mut last_digit = 0;

    for line in input {
        for c in 0..=line.len() {
            let substring = line[c..].to_string();
            first_digit = get_first_digit_from_substring(&substring, &number_map);
            if first_digit != 0 {
                break;
            }
        }

        for c in 0..=line.len() {
            let substring = line[..(line.len() - c)].to_string();
            last_digit = get_last_digit_from_substring(&substring, &number_map);
            if last_digit != 0 {
                break;
            }
        }

        numbers.push(first_digit * 10 + last_digit);
    }
    let total: u32 = numbers.iter().sum();
    total as i128
}

pub fn solve01(input: Vec<String>) -> (i128, i128) {
    (solve_part_1(input.clone()), solve_part_2(input))
}

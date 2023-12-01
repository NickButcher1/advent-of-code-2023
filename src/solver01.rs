fn solve_part_1(input: Vec<String>) -> i128 {
    let mut numbers = Vec::new();
    let mut first_digit = 0;
    let mut last_digit = 0;

    for line in input {
        for c in line.chars() {
            if c.is_digit(10) {
                first_digit = c.to_digit(10).unwrap();
                break;
            }
        }

        for c in line.chars().rev() {
            if c.is_digit(10) {
                last_digit = c.to_digit(10).unwrap();
                break;
            }
        }

        numbers.push(first_digit * 10 + last_digit);
    }
    let total: u32 = numbers.iter().sum();
    total as i128
}

fn solve_part_2(input: Vec<String>) -> i128 {
    0
}

pub fn solve01(input: Vec<String>) -> (i128, i128) {
    (solve_part_1(input.clone()), solve_part_2(input))
}

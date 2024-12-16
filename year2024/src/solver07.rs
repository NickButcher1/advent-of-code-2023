extern crate regex;
use regex::Regex;

pub fn solve(input: &[String], is_part_two: bool) -> i64 {
    let mut solution = 0;

    let re = Regex::new(r"(\d+):\s*((?:\d+\s*)+)").unwrap();
    for input_line in input {
        if let Some(captures) = re.captures(input_line) {
            let first_number: i64 = captures[1].parse().unwrap();

            let numbers: Vec<i64> = captures[2]
                .split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect();

            let mut output_vec = vec![numbers[0]];
            for number in numbers.iter().skip(1) {
                let mut new_vec = vec![];
                for item in output_vec {
                    new_vec.push(item + number);
                    let m = item * number;
                    if m <= first_number {
                        new_vec.push(m);
                    }
                    if is_part_two {
                        let mut digits = 1;
                        while digits <= *number {
                            digits *= 10;
                        }

                        let concat_number = item * digits + number;
                        if concat_number <= first_number {
                            new_vec.push(concat_number);
                        }
                    }
                }
                output_vec = new_vec;
            }

            for item in output_vec {
                if item == first_number {
                    solution += first_number;
                    break;
                }
            }
        }
    }

    solution
}

pub fn solve07(input: &[String]) -> (i128, i128) {
    (solve(input, false) as i128, solve(input, true) as i128)
}

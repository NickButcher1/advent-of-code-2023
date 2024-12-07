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
            for i in 1..numbers.len() {
                let mut new_vec = vec![];
                for j in 0..output_vec.len() {
                    new_vec.push(output_vec[j] + numbers[i]);
                    let m = output_vec[j] * numbers[i];
                    if m <= first_number {
                        new_vec.push(m);
                    }
                    if is_part_two {
                        let concat_number = [output_vec[j].to_string(), numbers[i].to_string()]
                            .concat()
                            .parse()
                            .unwrap();
                        if concat_number <= first_number {
                            new_vec.push(concat_number);
                        }
                    }
                }
                output_vec = new_vec;
            }

            for k in 0..output_vec.len() {
                if output_vec[k] == first_number {
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

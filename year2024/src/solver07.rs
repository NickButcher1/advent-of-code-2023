extern crate regex;
use regex::Regex;

pub fn solve07(input: &[String]) -> (i128, i128) {
    let mut part_one = 0;

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
                }
                output_vec = new_vec;
            }

            for k in 0..output_vec.len() {
                if output_vec[k] == first_number {
                    part_one += first_number;
                    break;
                }
            }
        }
    }

    (part_one as i128, 0 as i128)
}

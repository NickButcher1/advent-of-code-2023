use aoc::solution::{Solution, Solutions};

pub fn solve10(input: &[String]) -> Solutions {
    (
        Solution::I128(solve(&input[0], 40)),
        Solution::I128(solve(&input[0], 50)),
    )
}

fn solve(input: &str, num_iterations: usize) -> i128 {
    let mut output = input.to_string();
    for _ in 0..num_iterations {
        output = look_and_say(output);
    }

    output.chars().collect::<Vec<char>>().len() as i128
}

fn look_and_say(input: String) -> String {
    let mut output = String::new();
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        let mut count = 1;

        while chars.peek() == Some(&c) {
            chars.next();
            count += 1;
        }

        output.push_str(&count.to_string());
        output.push(c);
    }

    output
}

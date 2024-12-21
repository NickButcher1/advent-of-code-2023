use aoc::solution::{Solution, Solutions};
use itertools::iproduct;
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug)]
struct Code {
    number: i32,
    digits: [i32; 4], // Use ten for A.
}

fn parse_input(input: &[String]) -> Vec<Code> {
    let mut codes = vec![];
    let re = Regex::new(r"^(\d+)A$").unwrap();
    for line in input {
        let capture = re.captures(line).unwrap();
        let number = capture.get(1).unwrap().as_str().parse::<i32>().unwrap();
        codes.push(Code {
            number,
            digits: [number / 100, (number / 10) % 10, number % 10, 10],
        });
    }
    codes
}

fn add_direction_keypad(
    direction_keypad_to_direction_keypad: &HashMap<(char, char), Vec<char>>,
    input_direction_keypad_steps: &[char],
    mut output_direction_keypad_steps: Vec<char>,
) -> Vec<char> {
    for i in 0..(input_direction_keypad_steps.len() - 1) {
        let from = input_direction_keypad_steps[i];
        let to = input_direction_keypad_steps[i + 1];
        let new_seq = direction_keypad_to_direction_keypad
            .get(&(from, to))
            .unwrap();
        output_direction_keypad_steps.extend(new_seq);
    }

    output_direction_keypad_steps
}

fn add_intermediate_direction_keypad(
    direction_keypad_to_direction_keypad: &HashMap<(char, char), Vec<char>>,
    input_direction_keypad_steps: &[char],
) -> Vec<char> {
    add_direction_keypad(
        direction_keypad_to_direction_keypad,
        input_direction_keypad_steps,
        vec!['A'],
    )
}

fn add_final_direction_keypad(
    direction_keypad_to_direction_keypad: &HashMap<(char, char), Vec<char>>,
    input_direction_keypad_steps: &[char],
) -> Vec<char> {
    add_direction_keypad(
        direction_keypad_to_direction_keypad,
        input_direction_keypad_steps,
        vec![],
    )
}

pub fn solve(input: &[String], num_robot_keypads: i32) -> i64 {
    let codes = parse_input(input);

    // Key is (from, to) on numeric keypad. Value is a list of all the ways to make that move, where
    // each entry is the list of steps.
    let direction_keypad_to_numeric_keypad: HashMap<(i32, i32), Vec<Vec<char>>> = HashMap::from([
        ((0, 2), vec![vec!['^', 'A']]),
        ((0, 10), vec![vec!['>', 'A']]),
        (
            (1, 6),
            vec![
                vec!['>', '>', '^', 'A'],
                vec!['>', '^', '>', 'A'],
                vec!['^', '>', '>', 'A'],
            ],
        ),
        ((1, 7), vec![vec!['^', '^', 'A']]),
        (
            (2, 9),
            vec![
                vec!['^', '^', '>', 'A'],
                vec!['^', '>', '^', 'A'],
                vec!['>', '^', '^', 'A'],
            ],
        ),
        (
            (3, 7),
            vec![
                vec!['^', '^', '<', '<', 'A'],
                vec!['^', '<', '^', '<', 'A'],
                vec!['^', '<', '<', '^', 'A'],
                vec!['<', '^', '^', '<', 'A'],
                vec!['<', '^', '<', '^', 'A'],
                vec!['<', '<', '^', '^', 'A'],
            ],
        ),
        (
            (3, 8),
            vec![
                vec!['^', '^', '<', 'A'],
                vec!['^', '<', '^', 'A'],
                vec!['<', '^', '^', 'A'],
            ],
        ),
        ((3, 9), vec![vec!['^', '^', 'A']]),
        ((4, 5), vec![vec!['>', 'A']]),
        ((5, 6), vec![vec!['>', 'A']]),
        ((5, 7), vec![vec!['<', '^', 'A'], vec!['^', '<', 'A']]),
        ((6, 3), vec![vec!['v', 'A']]),
        (
            (6, 7),
            vec![
                vec!['^', '<', '<', 'A'],
                vec!['<', '^', '<', 'A'],
                vec!['<', '<', '^', 'A'],
            ],
        ),
        ((6, 9), vec![vec!['^', 'A']]),
        ((6, 10), vec![vec!['v', 'v', 'A']]),
        (
            (7, 0),
            vec![
                vec!['>', 'v', 'v', 'v', 'A'],
                vec!['v', '>', 'v', 'v', 'A'],
                vec!['v', 'v', '>', 'v', 'A'],
            ],
        ),
        ((7, 9), vec![vec!['>', '>', 'A']]),
        ((8, 0), vec![vec!['v', 'v', 'v', 'A']]),
        (
            (8, 3),
            vec![
                vec!['v', 'v', '>', 'A'],
                vec!['v', '>', 'v', 'A'],
                vec!['>', 'v', 'v', 'A'],
            ],
        ),
        (
            (8, 10),
            vec![
                vec!['v', 'v', 'v', '>', 'A'],
                vec!['v', 'v', '>', 'v', 'A'],
                vec!['v', '>', 'v', 'v', 'A'],
                vec!['>', 'v', 'v', 'v', 'A'],
            ],
        ),
        ((9, 8), vec![vec!['<', 'A']]),
        ((9, 10), vec![vec!['v', 'v', 'v', 'A']]),
        ((10, 0), vec![vec!['<', 'A']]),
        ((10, 1), vec![vec!['^', '<', '<', 'A']]),
        ((10, 3), vec![vec!['^', 'A']]),
        (
            (10, 4),
            vec![
                vec!['^', '^', '<', '<', 'A'],
                vec!['^', '<', '<', '^', 'A'],
                vec!['^', '<', '^', '<', 'A'],
                vec!['^', '<', '<', '^', 'A'],
                vec!['<', '^', '<', '^', 'A'],
                vec!['<', '^', '^', '<', 'A'],
            ],
        ),
        (
            (10, 5),
            vec![
                vec!['^', '^', '<', 'A'],
                vec!['^', '<', '^', 'A'],
                vec!['<', '^', '^', 'A'],
            ],
        ),
        ((10, 6), vec![vec!['^', '^', 'A']]),
        (
            (10, 8),
            vec![
                vec!['^', '^', '^', '<', 'A'],
                vec!['^', '^', '<', '^', 'A'],
                vec!['^', '<', '^', '^', 'A'],
                vec!['<', '^', '^', '^', 'A'],
            ],
        ),
        ((10, 9), vec![vec!['^', '^', '^', 'A']]),
    ]);

    // Key is (from, to) on arrow keypad. Value is a list of steps to make that move.
    let direction_keypad_to_direction_keypad: HashMap<(char, char), Vec<char>> = HashMap::from([
        (('A', '^'), vec!['<', 'A']),
        (('A', '>'), vec!['v', 'A']),
        (('A', 'v'), vec!['v', '<', 'A']),
        (('A', '<'), vec!['v', '<', '<', 'A']),
        (('A', 'A'), vec!['A']),
        (('^', 'A'), vec!['>', 'A']),
        (('^', 'v'), vec!['v', 'A']),
        (('^', '<'), vec!['v', '<', 'A']),
        (('^', '>'), vec!['v', '>', 'A']),
        (('^', '^'), vec!['A']),
        (('>', 'A'), vec!['^', 'A']),
        (('>', 'v'), vec!['<', 'A']),
        (('>', '<'), vec!['<', '<', 'A']),
        (('>', '^'), vec!['<', '^', 'A']),
        (('>', '>'), vec!['A']),
        (('v', 'A'), vec!['^', '>', 'A']),
        (('v', '^'), vec!['^', 'A']),
        (('v', '<'), vec!['<', 'A']),
        (('v', '>'), vec!['>', 'A']),
        (('v', 'v'), vec!['A']),
        (('<', 'A'), vec!['>', '>', '^', 'A']),
        (('<', 'v'), vec!['>', 'A']),
        (('<', '>'), vec!['>', '>', 'A']),
        (('<', '^'), vec!['>', '^', 'A']),
        (('<', '<'), vec!['A']),
    ]);

    let mut total_complexity = 0;
    for code in codes {
        let mut lowest_final_keypad_steps_for_this_code = usize::MAX;
        println!("\nCODE: {code:?}");

        // Iterate over the four moves on the numeric keypad to enter a code.
        for (move_1, move_2, move_3, move_4) in iproduct!(
            direction_keypad_to_numeric_keypad
                .get(&(code.digits[3], code.digits[0]))
                .unwrap(),
            direction_keypad_to_numeric_keypad
                .get(&(code.digits[0], code.digits[1]))
                .unwrap(),
            direction_keypad_to_numeric_keypad
                .get(&(code.digits[1], code.digits[2]))
                .unwrap(),
            direction_keypad_to_numeric_keypad
                .get(&(code.digits[2], code.digits[3]))
                .unwrap(),
        ) {
            // This gives the following sequence, starting from 'A', which isn't actually part of the sequence.
            let mut direction_keypad_steps = vec!['A'];
            direction_keypad_steps.extend(move_1);
            direction_keypad_steps.extend(move_2);
            direction_keypad_steps.extend(move_3);
            direction_keypad_steps.extend(move_4);

            for _ in 1..num_robot_keypads {
                direction_keypad_steps = add_intermediate_direction_keypad(
                    &direction_keypad_to_direction_keypad,
                    &direction_keypad_steps,
                );
            }
            direction_keypad_steps = add_final_direction_keypad(
                &direction_keypad_to_direction_keypad,
                &direction_keypad_steps,
            );

            if direction_keypad_steps.len() < lowest_final_keypad_steps_for_this_code {
                lowest_final_keypad_steps_for_this_code = direction_keypad_steps.len();
            }
        }

        println!(
            "BEST direction_keypad_steps_final: len {lowest_final_keypad_steps_for_this_code}"
        );

        total_complexity += code.number as i64 * lowest_final_keypad_steps_for_this_code as i64;
    }

    total_complexity
}

pub fn solve21(input: &[String]) -> Solutions {
    (
        Solution::I64(solve(input, 2)),
        Solution::I64(solve(input, 25)),
    )
}

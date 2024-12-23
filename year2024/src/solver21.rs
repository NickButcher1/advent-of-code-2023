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

fn add_direction_keypad_v2(
    direction_keypad_to_direction_keypad: &HashMap<(char, char), Vec<char>>,
    input_sequence_counts: &HashMap<(char, char), u32>,
    first_char_of_sequence: char,
    is_final_step: bool,
) -> (HashMap<(char, char), u32>, char) {
    let mut output_sequence_counts: HashMap<(char, char), u32> = HashMap::new();
    let mut new_first_char_of_sequence: char = ' ';

    if !is_final_step {
        // Add A at start
        let key = ('A', first_char_of_sequence);
        let new_seq = direction_keypad_to_direction_keypad.get(&key).unwrap();
        println!(
            "\n   Process {key:?} first_character {first_char_of_sequence}    new_seq: {new_seq:?}"
        );
        new_first_char_of_sequence = new_seq[0];

        // println!("       OSC: {output_sequence_counts:?}");
        // println!("       INC: (A,{}) by 1", new_seq[0]);
        *output_sequence_counts.entry(('A', new_seq[0])).or_insert(0) += 1;

        for i in 0..(new_seq.len() - 1) {
            // println!("     INDEX {i}");
            // println!("       OSC: {output_sequence_counts:?}");
            // println!("       INC: ({},{}) by 1", new_seq[i], new_seq[i+1]);
            *output_sequence_counts
                .entry((new_seq[i], new_seq[i + 1]))
                .or_insert(0) += 1;
            println!(
                "       Added ({},{}) new count {:?}",
                new_seq[i],
                new_seq[i + 1],
                output_sequence_counts
                    .get(&(new_seq[i], new_seq[i + 1]))
                    .unwrap()
            );
        }
    }

    for (key, value) in input_sequence_counts {
        let new_seq = direction_keypad_to_direction_keypad.get(key).unwrap();
        println!("\n   Process {key:?} -> {value}    new_seq: {new_seq:?}");

        let prev_count = *value;
        // println!("       OSC: {output_sequence_counts:?}");
        // println!("       INC: (A,{}) by {}", new_seq[0], prev_count);
        *output_sequence_counts.entry(('A', new_seq[0])).or_insert(0) += prev_count;

        for i in 0..(new_seq.len() - 1) {
            let prev_count = *value;
            // println!("     INDEX {i}");
            // println!("       OSC: {output_sequence_counts:?}");
            // println!("       INC: ({},{}) by {}", new_seq[i], new_seq[i+1], prev_count);
            *output_sequence_counts
                .entry((new_seq[i], new_seq[i + 1]))
                .or_insert(0) += prev_count;
            println!(
                "       Add ({},{}) prev_count {}  new count {:?}",
                new_seq[i],
                new_seq[i + 1],
                prev_count,
                output_sequence_counts
                    .get(&(new_seq[i], new_seq[i + 1]))
                    .unwrap()
            );
        }
    }

    (output_sequence_counts, new_first_char_of_sequence)
}

fn add_intermediate_direction_keypad_v2(
    direction_keypad_to_direction_keypad: &HashMap<(char, char), Vec<char>>,
    input_sequence_counts: &HashMap<(char, char), u32>,
    first_char_of_sequence: char,
) -> (HashMap<(char, char), u32>, char) {
    add_direction_keypad_v2(
        direction_keypad_to_direction_keypad,
        input_sequence_counts,
        first_char_of_sequence,
        false,
    )
}

fn add_final_direction_keypad_v2(
    direction_keypad_to_direction_keypad: &HashMap<(char, char), Vec<char>>,
    input_sequence_counts: &HashMap<(char, char), u32>,
    first_char_of_sequence: char,
) -> (HashMap<(char, char), u32>, char) {
    add_direction_keypad_v2(
        direction_keypad_to_direction_keypad,
        input_sequence_counts,
        first_char_of_sequence,
        true,
    )
}

pub fn solve(input: &[String], num_robot_keypads: i32) -> i64 {
    let codes = parse_input(input);

    // Key is (from, to) on numeric keypad. Value is a list of all the ways to make that move, where
    // each entry is the list of steps.
    // Omit sequences that have excessive direction changes (you never need to change direction
    // more than once).
    let direction_keypad_to_numeric_keypad: HashMap<(i32, i32), Vec<Vec<char>>> = HashMap::from([
        ((0, 2), vec![vec!['^', 'A']]),
        ((0, 10), vec![vec!['>', 'A']]),
        (
            (1, 6),
            vec![vec!['>', '>', '^', 'A'], vec!['^', '>', '>', 'A']],
        ),
        ((1, 7), vec![vec!['^', '^', 'A']]),
        (
            (2, 9),
            vec![vec!['^', '^', '>', 'A'], vec!['>', '^', '^', 'A']],
        ),
        (
            (3, 7),
            vec![vec!['^', '^', '<', '<', 'A'], vec!['<', '<', '^', '^', 'A']],
        ),
        (
            (3, 8),
            vec![vec!['^', '^', '<', 'A'], vec!['<', '^', '^', 'A']],
        ),
        ((3, 9), vec![vec!['^', '^', 'A']]),
        ((4, 5), vec![vec!['>', 'A']]),
        ((5, 6), vec![vec!['>', 'A']]),
        ((5, 7), vec![vec!['<', '^', 'A'], vec!['^', '<', 'A']]),
        ((6, 3), vec![vec!['v', 'A']]),
        (
            (6, 7),
            vec![vec!['^', '<', '<', 'A'], vec!['<', '<', '^', 'A']],
        ),
        ((6, 9), vec![vec!['^', 'A']]),
        ((6, 10), vec![vec!['v', 'v', 'A']]),
        ((7, 0), vec![vec!['>', 'v', 'v', 'v', 'A']]),
        ((7, 9), vec![vec!['>', '>', 'A']]),
        ((8, 0), vec![vec!['v', 'v', 'v', 'A']]),
        (
            (8, 3),
            vec![vec!['v', 'v', '>', 'A'], vec!['>', 'v', 'v', 'A']],
        ),
        (
            (8, 10),
            vec![vec!['v', 'v', 'v', '>', 'A'], vec!['>', 'v', 'v', 'v', 'A']],
        ),
        ((9, 8), vec![vec!['<', 'A']]),
        ((9, 10), vec![vec!['v', 'v', 'v', 'A']]),
        ((10, 0), vec![vec!['<', 'A']]),
        ((10, 1), vec![vec!['^', '<', '<', 'A']]),
        ((10, 3), vec![vec!['^', 'A']]),
        ((10, 4), vec![vec!['^', '^', '<', '<', 'A']]),
        (
            (10, 5),
            vec![vec!['^', '^', '<', 'A'], vec!['<', '^', '^', 'A']],
        ),
        ((10, 6), vec![vec!['^', '^', 'A']]),
        (
            (10, 8),
            vec![vec!['^', '^', '^', '<', 'A'], vec!['<', '^', '^', '^', 'A']],
        ),
        ((10, 9), vec![vec!['^', '^', '^', 'A']]),
    ]);

    // Key is (from, to) on arrow keypad. Value is a list of steps to make that move.
    let direction_keypad_to_direction_keypad: HashMap<(char, char), Vec<char>> = HashMap::from([
        (('A', '^'), vec!['<', 'A']),
        (('A', '>'), vec!['v', 'A']),
        (('A', 'v'), vec!['<', 'v', 'A']),
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
        (('v', 'A'), vec!['>', '^', 'A']),
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
        let mut idx = 0;
        let mut lowest_idx = 0;
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
            let mut direction_keypad_steps = vec![];
            direction_keypad_steps.extend(move_1);
            direction_keypad_steps.extend(move_2);
            direction_keypad_steps.extend(move_3);
            direction_keypad_steps.extend(move_4);
            idx += 1;
            let mut first_char_of_sequence: char = direction_keypad_steps[0];
            println!("\n    MOVE {idx}: first_char_of_sequence: {first_char_of_sequence}  direction_keypad_steps: {direction_keypad_steps:?}");
            println!("\n        move_1 {move_1:?}");
            println!("\n        move_2 {move_2:?}");
            println!("\n        move_3 {move_3:?}");
            println!("\n        move_4 {move_4:?}");

            let mut sequence_counts: HashMap<(char, char), u32> = HashMap::new();
            for i in 0..(direction_keypad_steps.len() - 1) {
                *sequence_counts
                    .entry((direction_keypad_steps[i], direction_keypad_steps[i + 1]))
                    .or_insert(0) += 1;
            }
            println!(
                "        FIRST SUM VALUES = {}, HASHMAP: {sequence_counts:?}",
                sequence_counts.values().sum::<u32>()
            );

            for i in 1..num_robot_keypads {
                (sequence_counts, first_char_of_sequence) = add_intermediate_direction_keypad_v2(
                    &direction_keypad_to_direction_keypad,
                    &sequence_counts,
                    first_char_of_sequence,
                );
                println!(
                    "        Depth {i} SUM VALUES = {}, HASHMAP: {sequence_counts:?}",
                    sequence_counts.values().sum::<u32>()
                );
                for (sk, sv) in &sequence_counts {
                    println!("            {:?} -> {}", sk, sv);
                }
            }
            (sequence_counts, _) = add_final_direction_keypad_v2(
                &direction_keypad_to_direction_keypad,
                &sequence_counts,
                first_char_of_sequence,
            );
            println!(
                "        Final SUM VALUES = {}, HASHMAP: {sequence_counts:?}",
                sequence_counts.values().sum::<u32>()
            );

            let total_steps: u32 = sequence_counts.values().sum();
            if total_steps < lowest_final_keypad_steps_for_this_code as u32 {
                lowest_idx = idx;
                lowest_final_keypad_steps_for_this_code = total_steps as usize;
            }
        }

        println!(
            "BEST MOVE {lowest_idx} direction_keypad_steps_final: {lowest_final_keypad_steps_for_this_code}"
        );

        total_complexity += code.number as i64 * lowest_final_keypad_steps_for_this_code as i64;
    }

    total_complexity
}

pub fn solve21(input: &[String]) -> Solutions {
    (
        Solution::I64(solve(input, 2)),
        // Solution::I64(0),
        // TOO LOW: 1485007033780
        Solution::I64(solve(input, 25)),
    )
}

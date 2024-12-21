use std::collections::HashMap;
use itertools::iproduct;
use regex::Regex;
use aoc::solution::{Solution, Solutions};

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
        codes.push(
            Code {
                number,
                digits: [
                    number / 100,
                    (number / 10) % 10,
                    number % 10,
                    10,
                ]
            }
        );
    }
    codes
}

pub fn solve21(input: &[String]) -> Solutions {
    let codes = parse_input(input);

    // Key is (from, to) on numeric keypad. Value is a list of all possible steps to make that move.
    let direction_keypad_to_numeric_keypad: HashMap<(i32, i32), Vec<Vec<char>>> = HashMap::from([
        ((0, 2), vec![vec!['^', 'A']]),
        ((0, 10), vec![vec!['>', 'A']]),
        ((1, 6), vec![vec!['>', '>', '^', 'A'], vec!['>', '^', '>', 'A'], vec!['^', '>', '>', 'A']]),
        ((1, 7), vec![vec!['^', '^', 'A']]),
        ((2, 9), vec![vec!['^', '^', '>', 'A'], vec!['^', '>', '^', 'A'], vec!['>', '^', '^', 'A']]),
        ((3, 7), vec![vec!['^', '^', '<', '<', 'A'], vec!['^', '<', '^', '<', 'A'], vec!['^', '<', '<', '^', 'A'], vec!['<', '^', '^', '<', 'A'], vec!['<', '^', '<', '^', 'A'], vec!['<', '<', '^', '^', 'A']]),
        ((3, 8), vec![vec!['^', '^', '<', 'A'], vec!['^', '<', '^', 'A'], vec!['<', '^', '^', 'A']]),
        ((3, 9), vec![vec!['^', '^', 'A']]),
        ((4, 5), vec![vec!['>', 'A']]),
        ((5, 6), vec![vec!['>', 'A']]),
        ((5, 7), vec![vec!['<', '^', 'A'], vec!['^', '<', 'A']]),
        ((6, 3), vec![vec!['v', 'A']]),
        ((6, 7), vec![vec!['^', '<', '<', 'A'], vec!['<', '^', '<', 'A'], vec!['<', '<', '^', 'A']]),
        ((6, 9), vec![vec!['^', 'A']]),
        ((6, 10), vec![vec!['v', 'v', 'A']]),
        ((7, 0), vec![vec!['>', 'v', 'v', 'v', 'A'], vec!['v', '>', 'v', 'v', 'A'], vec!['v', 'v', '>', 'v', 'A']]),
        ((7, 9), vec![vec!['>', '>', 'A']]),
        ((8, 0), vec![vec!['v', 'v', 'v', 'A']]),
        ((8, 3), vec![vec!['v', 'v', '>', 'A'], vec!['v', '>', 'v', 'A'], vec!['>', 'v', 'v', 'A']]),
        ((8, 10), vec![vec!['v', 'v', 'v', '>', 'A'], vec!['v', 'v', '>', 'v', 'A'], vec!['v', '>', 'v', 'v', 'A'], vec!['>', 'v', 'v', 'v', 'A']]),
        ((9, 8), vec![vec!['<', 'A']]),
        ((9, 10), vec![vec!['v', 'v', 'v', 'A']]),
        ((10, 0), vec![vec!['<', 'A']]),
        ((10, 1), vec![vec!['^', '<', '<', 'A']]),
        ((10, 3), vec![vec!['^', 'A']]),
        ((10, 4), vec![vec!['^', '^', '<', '<', 'A'], vec!['^', '<', '<', '^', 'A'], vec!['^', '<', '^', '<', 'A'], vec!['^', '<', '<', '^', 'A'], vec!['<', '^', '<', '^', 'A'], vec!['<', '^', '^', '<', 'A']]),
        ((10, 5), vec![vec!['^', '^', '<', 'A'], vec!['^', '<', '^', 'A'], vec!['<', '^', '^', 'A']]),
        ((10, 6), vec![vec!['^', '^', 'A']]),
        ((10, 8), vec![vec!['^', '^', '^', '<', 'A'], vec!['^', '^', '<', '^', 'A'], vec!['^', '<', '^', '^', 'A'], vec!['<', '^', '^', '^', 'A']]),
        ((10, 9), vec![vec!['^', '^', '^', 'A']]),
    ]);

    // Key is (from, to) on arrow keypad. Value is a list of all possible steps to make that move.
    let direction_keypad_to_direction_keypad: HashMap<(char, char), Vec<Vec<char>>> = HashMap::from([
        (('A', '^'), vec![vec!['<', 'A']]),
        (('A', '>'), vec![vec!['v', 'A']]),
        (('A', 'v'), vec![vec!['v', '<', 'A'], vec!['<', 'v']]),
        (('A', '<'), vec![vec!['v', '<', '<', 'A'], vec!['<', 'v', '<', 'A']]),
        (('A', 'A'), vec![vec!['A']]),
        (('^', 'A'), vec![vec!['>', 'A']]),
        (('^', 'v'), vec![vec!['v', 'A']]),
        (('^', '<'), vec![vec!['v', '<', 'A']]),
        (('^', '>'), vec![vec!['v', '>', 'A'], vec!['>', 'v', 'A']]),
        (('^', '^'), vec![vec!['A']]),
        (('>', 'A'), vec![vec!['^', 'A']]),
        (('>', 'v'), vec![vec!['<', 'A']]),
        (('>', '<'), vec![vec!['<', '<', 'A']]),
        (('>', '^'), vec![vec!['<', '^', 'A'], vec!['^', '<', 'A']]),
        (('>', '>'), vec![vec!['A']]),
        (('v', 'A'), vec![vec!['^', '>', 'A'], vec!['>', '^', 'A']]),
        (('v', '^'), vec![vec!['^', 'A']]),
        (('v', '<'), vec![vec!['<', 'A']]),
        (('v', '>'), vec![vec!['>', 'A']]),
        (('v', 'v'), vec![vec!['A']]),
        (('<', 'A'), vec![vec!['>', '>', '^', 'A'], vec!['>', '^', '>', 'A']]),
        (('<', 'v'), vec![vec!['>', 'A']]),
        (('<', '>'), vec![vec!['>', '>', 'A']]),
        (('<', '^'), vec![vec!['>', '^', 'A']]),
        (('<', '<'), vec![vec!['A']]),
    ]);

    let mut total_complexity_solution_one = 0;
    for code in codes {
        let mut lowest_final_keypad_steps_for_this_code = usize::MAX;
        let mut best_direction_keypad_steps_3: Vec<char> = vec![];
        println!("\nCODE: {code:?}");

        // Iterate over the four moves on the numeric keypad to enter a code.
        for (move_1, move_2, move_3, move_4) in iproduct!(
            direction_keypad_to_numeric_keypad.get(&(code.digits[3], code.digits[0])).unwrap(),
            direction_keypad_to_numeric_keypad.get(&(code.digits[0], code.digits[1])).unwrap(),
            direction_keypad_to_numeric_keypad.get(&(code.digits[1], code.digits[2])).unwrap(),
            direction_keypad_to_numeric_keypad.get(&(code.digits[2], code.digits[3])).unwrap(),
        ) {
            // This gives the following sequence, starting from 'A', which isn't actually part of the sequence.
            let mut direction_keypad_steps_1 = vec!['A'];
            direction_keypad_steps_1.extend(move_1);
            direction_keypad_steps_1.extend(move_2);
            direction_keypad_steps_1.extend(move_3);
            direction_keypad_steps_1.extend(move_4);

            // println!("direction_keypad_steps_1: len {}  {direction_keypad_steps_1:?}", direction_keypad_steps_1.len() - 1);
            let mut direction_keypad_steps_2 = vec!['A'];
            for i in 0..(direction_keypad_steps_1.len() - 1) {
                let from = direction_keypad_steps_1[i];
                let to = direction_keypad_steps_1[i+1];
                let new_seq = direction_keypad_to_direction_keypad.get(&(from, to)).unwrap();
                // println!("    {from} -> {to}   new seq {new_seq:?}");
                direction_keypad_steps_2.extend(&new_seq[0]);
            }

            // println!("direction_keypad_steps_2: len {}  {direction_keypad_steps_2:?}", direction_keypad_steps_2.len() - 1);

            let mut direction_keypad_steps_3: Vec<char> = vec![];
            for i in 0..(direction_keypad_steps_2.len() - 1) {
                let from = direction_keypad_steps_2[i];
                let to = direction_keypad_steps_2[i+1];
                // println!("    {from} -> {to}   new seq {new_seq:?}");
                let new_seq = direction_keypad_to_direction_keypad.get(&(from, to)).unwrap();
                direction_keypad_steps_3.extend(&new_seq[0]);
            }
            // println!("MOVEIT3: len {}  {moveit3:?}", moveit3.len());
            if direction_keypad_steps_3.len() < lowest_final_keypad_steps_for_this_code {
                lowest_final_keypad_steps_for_this_code = direction_keypad_steps_3.len();
                best_direction_keypad_steps_3 = direction_keypad_steps_3;
            }
        }

        println!("BEST direction_keypad_steps_3: len {}  {lowest_final_keypad_steps_for_this_code:?}", best_direction_keypad_steps_3.len());

        total_complexity_solution_one += code.number * lowest_final_keypad_steps_for_this_code as i32;
    }

    (Solution::I32(total_complexity_solution_one), Solution::I32(0))
}

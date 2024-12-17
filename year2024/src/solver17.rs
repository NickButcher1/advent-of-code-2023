use aoc::solution::{Solution, Solutions};
use itertools::Itertools;

#[derive(Clone, Debug)]
struct Computer {
    a: u64,
    program: Vec<u64>,
    is_sample: bool,
    output: Vec<u64>,
}

#[allow(dead_code)]
impl Computer {
    // This is a vast simplification of the computer architecture defined in the puzzle.
    // It simply maps from some value of a to some output value, and returns true if the program
    // should continue.
    //
    // Inspection of the sample and actual programs shows that both follow this pattern:
    // - Manipulate the registers over multiple steps.
    // - Output a value.
    // - Jump to instruction pointer zero if a is non-zero, otherwise halt.
    // Registers b and c are never used as input - they are always derived from some combination of
    // register a and the program instructions.
    fn execute_program_once(&mut self) -> bool {
        if !self.is_sample {
            let partial = (self.a % 8) ^ 7;
            let c = self.a / 2u64.pow(partial as u32); // 75
            let output = ((partial ^ c) ^ 7) % 8; // 40 17
            self.a /= 8; // 03
            self.output.push(output); // 55
        } else {
            self.a /= 8; // 03  For the part two sample, change the 8 to a 2.
            self.output.push(self.a % 8); // 54
        }
        self.a != 0
    }
}

fn initialize_computer() -> Computer {
    // TODO: I have hardcoded the input here, to save time in parsing the input file.
    // Part one sample
    // Computer { a: 729, b: 0, c: 0, program: vec![0,1,5,4,3,0], is_sample: true, output: vec![]}
    // Part two sample
    // Computer { a: 2024, program: vec![0,3,5,4,3,0], is_sample: true, output: vec![]}
    Computer {
        a: 62769524,
        program: vec![2, 4, 1, 7, 7, 5, 0, 3, 4, 0, 1, 7, 5, 5, 3, 0],
        is_sample: false,
        output: vec![],
    }
}

fn solve_part_one() -> String {
    let mut computer = initialize_computer();
    while computer.execute_program_once() {}
    computer
        .output
        .iter()
        .map(|digit| digit.to_string())
        .join(",")
}

// Recurse backwards, to find the a value(s) that can lead to the required output.
//
// Given that every program execution divides a by 8, and the program stops when a reaches zero,
// we know that the value of a which produces the final output value must be in range 1-7.
fn solve_part_two() -> u64 {
    let mut computer = initialize_computer();
    let mut target_index = computer.program.len();
    let mut candidates = vec![(1, 0)];

    while target_index != 0 {
        let mut new_candidates = vec![];
        for (min_a, prev_a) in &candidates {
            // I think this upper bound is right, it certainly works.
            for a in *min_a..=(*min_a + 8) {
                computer.a = a;
                computer.output.clear();
                let _not_halted = computer.execute_program_once();
                if computer.output[0] == computer.program[target_index - 1] && *prev_a == computer.a
                {
                    // Lower bound for next value of a must be 8 times this one, because a is divided by 8
                    // on each program execution.
                    new_candidates.push((a * 8, a));
                }
            }
        }

        target_index -= 1;
        candidates = new_candidates;
    }

    // There might be multiple values of a that generate the program, but we only want the lowest
    // one.
    candidates[0].1
}

pub fn solve17(_input: &[String]) -> Solutions {
    (
        Solution::STR(solve_part_one()),
        Solution::U64(solve_part_two()),
    )
}

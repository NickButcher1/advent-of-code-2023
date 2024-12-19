use aoc::solution::{Solution, Solutions};
use regex::Regex;
use std::collections::HashSet;

#[derive(Debug)]
struct Computer {
    accumulator: i32,
    ip: i32,
    modified_instrucion: i32,
}

struct Program<'a> {
    instructions: &'a Vec<&'a str>,
    arguments: Vec<i32>,
}

impl Computer {
    fn execute_current_instruction(&mut self, program: &Program) {
        match program.instructions[self.ip as usize] {
            "acc" => {
                self.accumulator += program.arguments[self.ip as usize];
                self.ip += 1;
            }
            "jmp" => {
                if self.modified_instrucion != self.ip {
                    self.ip += program.arguments[self.ip as usize];
                } else {
                    self.ip += 1;
                }
            }
            "nop" => {
                if self.modified_instrucion != self.ip {
                    self.ip += 1;
                } else {
                    self.ip += program.arguments[self.ip as usize];
                }
            }
            _ => {
                unreachable!()
            }
        }
    }
}

pub fn solve08(input: &[String]) -> Solutions {
    let re = Regex::new(r"^(acc|jmp|nop) ([+-]\d+)$").unwrap();
    let mut computer = Computer {
        accumulator: 0,
        ip: 0,
        modified_instrucion: -1,
    };

    let mut instructions: Vec<&str> = vec![];
    let mut arguments: Vec<i32> = vec![];
    let mut instructions_executed: HashSet<i32> = HashSet::new();

    for line in input {
        let captures = re.captures(line).unwrap();
        instructions.push(captures.get(1).unwrap().as_str());
        arguments.push(captures.get(2).unwrap().as_str().parse::<i32>().unwrap());
    }
    let program = Program {
        instructions: &instructions,
        arguments,
    };

    let solution_one;
    loop {
        if instructions_executed.contains(&computer.ip) {
            solution_one = computer.accumulator;
            break;
        }
        instructions_executed.insert(computer.ip);
        computer.execute_current_instruction(&program);
    }

    let mut computers: Vec<Computer> = vec![];
    for i in 0..instructions.len() {
        if program.instructions[i] == "jmp" || program.instructions[i] == "nop" {
            computers.push(Computer {
                accumulator: 0,
                ip: 0,
                modified_instrucion: i as i32,
            });
        }
    }

    loop {
        for computer in &mut computers {
            computer.execute_current_instruction(&program);
            if computer.ip >= program.instructions.len() as i32 {
                return (
                    Solution::I32(solution_one),
                    Solution::I32(computer.accumulator),
                );
            }
        }
    }
}

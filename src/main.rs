mod board;
mod common;
mod solver01;
mod solver02;
mod solver03;
mod solver04;
mod solver05;
mod solver06;
mod solver07;
mod solver08;
mod solver09;
mod solver10;
mod solver11;
mod solver12;
mod solver13;
mod solver14;
mod solver15;
mod solver16;
mod solver17;
mod solver18;
mod solver19;
mod solver20;
mod solver21;
mod solver22;
mod solver23;
mod solver24;
mod solver25;

use crate::solver01::solve01;
use crate::solver02::solve02;
use crate::solver03::solve03;
use crate::solver04::solve04;
use crate::solver05::solve05;
use crate::solver06::solve06;
use crate::solver07::solve07;
use crate::solver08::solve08;
use crate::solver09::solve09;
use crate::solver10::solve10;
use crate::solver11::solve11;
use crate::solver12::solve12;
use crate::solver13::solve13;
use crate::solver14::solve14;
use crate::solver15::solve15;
use crate::solver16::solve16;
use crate::solver17::solve17;
use crate::solver18::solve18;
use crate::solver19::solve19;
use crate::solver20::solve20;
use crate::solver21::solve21;
use crate::solver22::solve22;
use crate::solver23::solve23;
use crate::solver24::solve24;
use crate::solver25::solve25;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

type SolverFunction = fn(Vec<String>) -> (i128, i128);

const SOLVER_FUNCTIONS: [SolverFunction; 25] = [
    // Display row 1 (top edge).
    solve01, solve02, solve03, solve04, solve05, solve06, solve07, solve08, solve09, solve10,
    solve11, solve12, solve13, solve14, solve15, solve16, solve17, solve18, solve19, solve20,
    solve21, solve22, solve23, solve24, solve25,
];

fn read_input_file(filename: &str) -> Vec<String> {
    let file = match File::open(filename) {
        Ok(file) => file,
        Err(e) => {
            panic!("Missing input file: {}", e);
        }
    };

    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line| match line {
            Ok(line) => line,
            Err(e) => {
                panic!("Error reading line: {}", e);
            }
        })
        .collect()
}

fn run_one_day(day: i32, is_sample_mode: bool, expected_outputs: Vec<String>) -> f64 {
    let time = Instant::now();

    let filename = format!(
        "input/input{:02}{}",
        day,
        if is_sample_mode { "-sample" } else { "" }
    );
    let input_file = read_input_file(&filename);
    let (result1, result2) = if input_file.is_empty() {
        (-1, -1)
    } else {
        SOLVER_FUNCTIONS[day as usize - 1](input_file)
    };

    let elapsed_ms = time.elapsed().as_nanos() as f64 / 1_000_000.0;

    println!(
        "{:02}    {:12}  {:16}  {:12}ms",
        day, result1, result2, elapsed_ms
    );

    if !is_sample_mode {
        let expected_result1 = &expected_outputs[day as usize * 2 - 2];
        let expected_result2 = &expected_outputs[day as usize * 2 - 1];

        if expected_result1 != "-1" && result1.to_string() != *expected_result1 {
            panic!(
                "Day {}, incorrect result1: expected {}, actual {}",
                day, expected_result1, result1
            );
        }

        if expected_result2 != "-1" && result2.to_string() != *expected_result2 {
            panic!(
                "Day {}, incorrect result2: expected {}, actual {}",
                day, expected_result2, result2
            );
        }
    }

    elapsed_ms
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let day = if args.len() >= 2 {
        match args[1].parse::<i32>() {
            Ok(number) => number,
            Err(_) => {
                // Parsing failed
                panic!("Failed to parse the argument as an integer: {}", args[1]);
            }
        }
    } else {
        0
    };

    let is_sample_mode = args.len() >= 3;
    let expected_outputs: Vec<String> = BufReader::new(File::open("expected_outputs.txt").unwrap())
        .lines()
        .collect::<Result<_, _>>()
        .unwrap();

    if day != 0 {
        run_one_day(day, is_sample_mode, expected_outputs);
    } else {
        let mut total_ms: f64 = 0.0;
        for day in 1..=25 {
            total_ms += run_one_day(day, is_sample_mode, expected_outputs.clone());
        }
        println!(
            "TOTAL                                      {:.2}ms",
            total_ms
        );
    }
}

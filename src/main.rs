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
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

const SOLVER_FUNCTIONS: [fn(Vec<String>) -> (i128, i128); 10] = [
    // Display row 1 (top edge).
    solve01, solve02, solve03, solve04, solve05, solve06, solve07, solve08, solve09, solve10,
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

fn run_one_day(day: i32, is_sample_mode: bool) {
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

    println!("{}  {}  {}ms", result1, result2, elapsed_ms);
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

    if day != 0 {
        run_one_day(day, is_sample_mode);
    } else {
        for day in 1..25 {
            run_one_day(day, is_sample_mode);
        }
    }
}

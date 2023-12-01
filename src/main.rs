mod solver01;

use crate::solver01::solve01;
use std::env;
use std::fs::File;
use std::io::Read;
use std::time::Instant;

fn read_input_file(filename: &str) -> Vec<u8> {
    let mut file = match File::open(filename) {
        Ok(file) => file,
        Err(_) => {
            return Vec::new();
        }
    };

    let mut buffer = Vec::new();
    match file.read_to_end(&mut buffer) {
        Ok(_) => {}
        Err(e) => {
            panic!("Failed to read file: {}", e);
        }
    }

    buffer
}

fn run_one_day(day: i32, is_sample_mode: bool) {
    let filename = format!(
        "input/input{:02}{}",
        day,
        if is_sample_mode { "-sample" } else { "" }
    );
    let input_file = read_input_file(&filename);
    let (result1, result2) = if input_file.is_empty() {
        (-1, -1)
    } else {
        solve01(input_file)
    };
    println!("{}  {}", result1, result2);
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

    let time = Instant::now();
    if day != 0 {
        run_one_day(day, is_sample_mode);
    } else {
        for day in 1..25 {
            run_one_day(day, is_sample_mode);
        }
    }
    let elapsed_ms = time.elapsed().as_nanos() as f64 / 1_000_000.0;
    println!(
        "Solve day {}, sample mode: {}, took {}ms",
        day, is_sample_mode, elapsed_ms
    );
}

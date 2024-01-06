use clap::Parser;
use std::collections::HashMap;
extern crate aoc;
extern crate year2023;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;
use year2023::SolverFunction;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = (&"2024").to_string())]
    year: String,

    #[arg(short, long, default_value_t = 0)]
    day: usize,

    #[arg(short, long, default_value_t = false)]
    sample: bool,
}

fn read_input_file(filename: &str) -> Vec<String> {
    let file = match File::open(filename) {
        Ok(file) => file,
        Err(e) => {
            println!("Missing input file: {e}");
            return vec![];
        }
    };

    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line| match line {
            Ok(line) => line,
            Err(e) => {
                panic!("Error reading line: {e}");
            }
        })
        .collect()
}

fn run_one_day(
    solver_fn: SolverFunction,
    year: &str,
    day: usize,
    is_sample_mode: bool,
    expected_outputs: &[String],
) -> u128 {
    let time = Instant::now();

    let filename = format!(
        "input/{year}/input{day:02}{}",
        if is_sample_mode { "-sample" } else { "" }
    );
    let input_file = read_input_file(&filename);
    let (result1, result2) = if input_file.is_empty() {
        (-1, -1)
    } else {
        solver_fn(&input_file)
    };

    let elapsed_ms = time.elapsed().as_nanos() / 1_000_000;

    if result1 != 0 && result2 != 0 {
        println!("{day:02}    {result1:12}  {result2:16}  {elapsed_ms:12}ms",);
    } else {
        println!("{day:02}            todo              todo");
    }

    if !is_sample_mode {
        let expected_result1 = &expected_outputs[day * 2 - 2];
        let expected_result2 = &expected_outputs[day * 2 - 1];

        assert!(
            expected_result1 == "-1" || result1.to_string() == *expected_result1,
            "Day {day}, incorrect part 1: expected {expected_result1}, actual {result1}"
        );

        assert!(
            expected_result2 == "-1" || result2.to_string() == *expected_result2,
            "Day {day}, incorrect part 2: expected {expected_result2}, actual {result2}"
        );
    }

    elapsed_ms
}

fn main() {
    let solver_fns: HashMap<String, &[SolverFunction; 25]> = HashMap::from([
        ("2024".to_string(), &year2024::SOLVER_FUNCTIONS),
        ("2023".to_string(), &year2023::SOLVER_FUNCTIONS),
        ("2022".to_string(), &year2022::SOLVER_FUNCTIONS),
        ("2021".to_string(), &year2021::SOLVER_FUNCTIONS),
        ("2020".to_string(), &year2020::SOLVER_FUNCTIONS),
        ("2019".to_string(), &year2019::SOLVER_FUNCTIONS),
        ("2018".to_string(), &year2018::SOLVER_FUNCTIONS),
        ("2017".to_string(), &year2017::SOLVER_FUNCTIONS),
        ("2016".to_string(), &year2016::SOLVER_FUNCTIONS),
        ("2015".to_string(), &year2015::SOLVER_FUNCTIONS),
    ]);

    let args = Args::parse();

    let expected_outputs: Vec<String> =
        BufReader::new(File::open(format!("output/{}/expected_outputs.txt", args.year)).unwrap())
            .lines()
            .collect::<Result<_, _>>()
            .unwrap();

    if args.day != 0 {
        run_one_day(
            solver_fns.get(&args.year).unwrap()[args.day - 1],
            &args.year,
            args.day,
            args.sample,
            &expected_outputs,
        );
    } else {
        let mut total_ms: u128 = 0;
        for day in 1..=25 {
            total_ms += run_one_day(
                solver_fns.get(&args.year).unwrap()[day - 1],
                &args.year,
                day,
                args.sample,
                &expected_outputs,
            );
        }
        println!("TOTAL                                         {total_ms:.2}ms");
    }
}

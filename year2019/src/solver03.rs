use aoc::solution::{Solution, Solutions};
use regex::Regex;
use std::collections::{HashMap, HashSet};

fn parse_step(step: &str) -> (i32, i32, i32) {
    let (direction, number) = step.split_at(1);
    let (dx, dy) = match direction {
        "U" => (-1, 0),
        "D" => (1, 0),
        "L" => (0, -1),
        "R" => (0, 1),
        _ => {
            unreachable!()
        }
    };
    let num_moves = number.parse().unwrap();

    (dx, dy, num_moves)
}

pub fn solve03(input: &[String]) -> Solutions {
    let re = Regex::new(r"[A-Z]\d+").unwrap();

    let mut is_first_line = true;
    let mut first_line_points: HashSet<(i32, i32)> = HashSet::new();
    let mut first_line_step_counts: HashMap<(i32, i32), i32> = HashMap::new();
    let mut lowest_manhattan_distance = i32::MAX;
    let mut lowest_combined_step_count = i32::MAX;

    for line in input {
        let steps: Vec<String> = re
            .find_iter(line)
            .map(|mat| mat.as_str().to_string())
            .collect();

        let mut x = 0;
        let mut y = 0;
        let mut step_count = 0;
        for step in steps {
            let (dx, dy, num_moves) = parse_step(&step);
            for _ in 0..num_moves {
                step_count += 1;
                x += dx;
                y += dy;
                if is_first_line {
                    first_line_points.insert((x, y));
                    first_line_step_counts.entry((x, y)).or_insert(step_count);
                } else if first_line_points.contains(&(x, y)) {
                    let manhattan_distance = x.abs() + y.abs();
                    if manhattan_distance < lowest_manhattan_distance {
                        lowest_manhattan_distance = manhattan_distance;
                    }

                    let combined_step_count =
                        step_count + first_line_step_counts.get(&(x, y)).unwrap();
                    if combined_step_count < lowest_combined_step_count {
                        lowest_combined_step_count = combined_step_count;
                    }
                }
            }
        }

        is_first_line = false;
    }

    (
        Solution::I32(lowest_manhattan_distance),
        Solution::I32(lowest_combined_step_count),
    )
}

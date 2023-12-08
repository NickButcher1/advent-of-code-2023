use num_integer::lcm;
use regex::Regex;
use std::collections::HashMap;

// There are N parallel paths that all need to terminate at the same time.
// - In part 1, N=1 (terminate on location 'ZZZ'.
// - In part 2, N is calculated (terminate on any location that ends with 'Z').
//
// We start with the first location for each path.
// - In part 1, the starting location is 'AAA'.
// - In part 2, any location beginning with 'A' is a starting location.
//
// The main loop then:
// - Loops over the directions (repeating if necessary) and for each direction updates the location
//   for each path.
// - The fist time a path reaches a terminating location, records the number of steps.
// - When all paths have reached a terminating location at least once, stops.
// - For part 1, the solution is simply the number of steps to reach the terminating location.
// - For part 2, the solution is too big to brute force. The solution is the least common multiple
//   (LCM) of the number of steps to first terminate each path. The puzzle has been designed such
//   that each path follows a repeating loop, with exactly one terminating location in its path.

fn is_a_terminating_location(locations: &Vec<&str>, id: usize) -> bool {
    if locations.len() == 1 {
        // Part 1
        locations[id] == "ZZZ"
    } else {
        // Par 2
        locations[id].chars().collect::<Vec<char>>()[2] == 'Z'
    }
}

fn solve<'a>(
    directions: Vec<char>,
    from_x_to_left: HashMap<&str, &'a str>,
    from_x_to_right: HashMap<&str, &'a str>,
    mut locations: Vec<&'a str>,
) -> i128 {
    let num_paths = locations.len();
    let mut steps = 0;
    let mut first_z_for_path: Vec<u64> = vec![0; num_paths];
    let mut first_z_found_count = 0;

    loop {
        for direction in &directions {
            steps += 1;

            for i in 0..num_paths {
                match direction {
                    'L' => locations[i] = from_x_to_left[locations[i]],
                    'R' => locations[i] = from_x_to_right[locations[i]],
                    _ => unreachable!(),
                }
            }

            for (i, first_z) in first_z_for_path.iter_mut().enumerate() {
                if *first_z == 0 && is_a_terminating_location(&locations, i) {
                    *first_z = steps;
                    first_z_found_count += 1;
                }
            }

            if first_z_found_count == num_paths {
                let mut answer = first_z_for_path[0];
                for first_z in first_z_for_path.iter() {
                    answer = lcm(answer, *first_z);
                }
                return answer as i128;
            }
        }
    }
}

pub fn solve08(input: Vec<String>) -> (i128, i128) {
    let directions: Vec<char> = input[0].chars().collect();
    let mut from_x_to_left: HashMap<&str, &str> = HashMap::new();
    let mut from_x_to_right: HashMap<&str, &str> = HashMap::new();
    let mut locations_part_2: Vec<&str> = Vec::new();

    let re = Regex::new(r"\b[A-Z0-9]+\b").unwrap();

    for input_line in input.iter().skip(2) {
        let matches: Vec<_> = re.find_iter(input_line).map(|m| m.as_str()).collect();
        from_x_to_left.insert(matches[0], matches[1]);
        from_x_to_right.insert(matches[0], matches[2]);
        if matches[0].chars().collect::<Vec<char>>()[2] == 'A' {
            locations_part_2.push(matches[0]);
        }
    }

    (
        solve(
            directions.clone(),
            from_x_to_left.clone(),
            from_x_to_right.clone(),
            vec!["AAA"],
        ),
        solve(
            directions,
            from_x_to_left,
            from_x_to_right,
            locations_part_2,
        ),
    )
}

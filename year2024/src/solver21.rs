use aoc::solution::{Solution, Solutions};
use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;

type Keypad = [[char; 3]; 4];
const NUMERIC_KEYPAD: Keypad = [
    ['7', '8', '9'],
    ['4', '5', '6'],
    ['1', '2', '3'],
    [' ', '0', 'A'],
];

const ARROW_KEYPAD: Keypad = [
    [' ', '^', 'A'],
    ['<', 'v', '>'],
    [' ', ' ', ' '],
    [' ', ' ', ' '],
];

struct Robot<'a> {
    button: char,
    is_human: bool,
    moves_from_to: &'a HashMap<(char, char), Vec<String>>,
    known_cost_from_to: HashMap<(char, char), i64>,
}

#[derive(Debug)]
struct Code {
    number: u32,
    digits: [char; 4],
}

fn parse_input(input: &[String]) -> Vec<Code> {
    let re = Regex::new(r"^(\d+)A$").unwrap();

    input
        .iter()
        .map(|line| {
            let capture = re.captures(line).unwrap();
            let number = capture.get(1).unwrap().as_str().parse::<u32>().unwrap();
            Code {
                number,
                digits: [
                    char::from_digit(number / 100, 10).unwrap(),
                    char::from_digit((number / 10) % 10, 10).unwrap(),
                    char::from_digit(number % 10, 10).unwrap(),
                    'A',
                ],
            }
        })
        .collect()
}

fn path_to_valid_permutations(
    keypad: &Keypad,
    from_r: i32,
    from_c: i32,
    base_path: &str,
) -> Vec<String> {
    let mut valid_paths: Vec<String> = vec![];

    for path in base_path.chars().permutations(base_path.len()) {
        let path = path.iter().collect::<String>() + "A";
        if is_path_valid(keypad, from_r, from_c, &path) {
            valid_paths.push(path);
        }
    }

    valid_paths
}

fn is_path_valid(keypad: &Keypad, mut r: i32, mut c: i32, path: &str) -> bool {
    for button in path.chars() {
        if keypad[r as usize][c as usize] == ' ' {
            return false;
        } else {
            match button {
                'A' => break,
                '<' => c -= 1,
                '>' => c += 1,
                '^' => r -= 1,
                'v' => r += 1,
                _ => {}
            }
        }
    }
    true
}

fn move_keypad_to(robots: &mut Vec<Robot>, index: usize, to_button: char) -> i64 {
    let robot = &mut robots[index];

    if robot.is_human {
        return 1;
    }

    let from_button = robot.button;

    let valid_paths = robot.moves_from_to.get(&(from_button, to_button)).unwrap();

    robot.button = to_button;

    if let Some(cost) = robot.known_cost_from_to.get(&(from_button, to_button)) {
        return *cost;
    }

    let lowest_cost = valid_paths
        .iter()
        .map(|path| {
            path.chars()
                .map(|c| move_keypad_to(robots, index + 1, c))
                .sum()
        })
        .min()
        .unwrap_or(i64::MAX);

    // Satisfy tne borrow checker - can't just use `current_robot`.
    let robot2 = &mut robots[index];
    robot2
        .known_cost_from_to
        .insert((from_button, to_button), lowest_cost);
    lowest_cost
}

// For a keypad, build the map:
// - from current button and target button
// - to all valid paths to get there.
fn build_moves_from_to(keypad: &Keypad) -> HashMap<(char, char), Vec<String>> {
    let mut moves_from_to: HashMap<(char, char), Vec<String>> = HashMap::new();
    for (from_r, row) in keypad.iter().enumerate() {
        for (from_c, from_button) in row.iter().enumerate() {
            for (to_r, row) in keypad.iter().enumerate() {
                for (to_c, to_button) in row.iter().enumerate() {
                    if *from_button != ' ' && *to_button != ' ' {
                        let path = match from_c.cmp(&to_c) {
                            std::cmp::Ordering::Greater => "<".repeat(from_c - to_c),
                            std::cmp::Ordering::Less => ">".repeat(to_c - from_c),
                            std::cmp::Ordering::Equal => String::new(),
                        } + &match from_r.cmp(&to_r) {
                            std::cmp::Ordering::Greater => "^".repeat(from_r - to_r),
                            std::cmp::Ordering::Less => "v".repeat(to_r - from_r),
                            std::cmp::Ordering::Equal => String::new(),
                        };

                        let valid_paths =
                            path_to_valid_permutations(keypad, from_r as i32, from_c as i32, &path);
                        moves_from_to.insert((keypad[from_r][from_c], *to_button), valid_paths);
                    }
                }
            }
        }
    }
    moves_from_to
}

fn solve_for_code(code: &Code, robots: &mut Vec<Robot>) -> i64 {
    code.number as i64
        * code
            .digits
            .iter()
            .map(|&digit| move_keypad_to(robots, 0, digit))
            .sum::<i64>()
}

fn solve(codes: &[Code], num_robot_keypads: i32) -> i64 {
    let numeric_moves_from_to = build_moves_from_to(&NUMERIC_KEYPAD);
    let arrow_moves_from_to = build_moves_from_to(&ARROW_KEYPAD);

    let mut robots: Vec<Robot> = vec![];
    robots.push(Robot {
        button: 'A',
        is_human: false,
        moves_from_to: &numeric_moves_from_to,
        known_cost_from_to: HashMap::new(),
    });

    for _ in 1..=num_robot_keypads {
        robots.push(Robot {
            button: 'A',
            is_human: false,
            moves_from_to: &arrow_moves_from_to,
            known_cost_from_to: HashMap::new(),
        });
    }

    // The human controller at the end of the chain.
    robots.push(Robot {
        button: 'A',
        is_human: true,
        moves_from_to: &arrow_moves_from_to,
        known_cost_from_to: HashMap::new(),
    });

    codes
        .iter()
        .map(|code| solve_for_code(code, &mut robots))
        .sum()
}

pub fn solve21(input: &[String]) -> Solutions {
    let codes = parse_input(input);

    (
        Solution::I64(solve(&codes, 2)),
        Solution::I64(solve(&codes, 25)),
    )
}

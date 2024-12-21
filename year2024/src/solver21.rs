use std::collections::HashMap;
use regex::Regex;
use aoc::solution::{Solution, Solutions};

#[derive(Debug)]
struct Code {
    number: i32,
    digits: [i32; 4], // Use ten for A.
}
pub fn solve21(input: &[String]) -> Solutions {
    // Parse input.
    // TODO: Simplify input reading.
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
    println!("CODES: {codes:?}");

    // Index is from-to on numeric. Order U R D L A.
    // For key move on numeric do steps on right on radition.
    let mut radiation_to_numeric: HashMap<(i32, i32), Vec<char>> = HashMap::new();
    radiation_to_numeric.insert((0, 2),vec!['^', 'A']);
    radiation_to_numeric.insert((0, 10),vec!['>', 'A']);
    radiation_to_numeric.insert((1, 7),vec!['^', '^', 'A']);
    radiation_to_numeric.insert((2, 9),vec!['^', '^', '>', 'A']);
    radiation_to_numeric.insert((3, 7),vec!['^', '^', '<', '<', 'A']);
    radiation_to_numeric.insert((4, 5),vec!['>', 'A']);
    radiation_to_numeric.insert((5, 6),vec!['>', 'A']);
    radiation_to_numeric.insert((6, 10),vec!['v', 'v', 'A']);
    radiation_to_numeric.insert((7, 9),vec!['>', '>', 'A']);
    radiation_to_numeric.insert((8, 0),vec!['v', 'v', 'v', 'A']);
    radiation_to_numeric.insert((9, 8),vec!['<', 'A']);
    radiation_to_numeric.insert((9, 10),vec!['v', 'v', 'v', 'A']);
    radiation_to_numeric.insert((10, 0),vec!['<', 'A']);
    radiation_to_numeric.insert((10, 1),vec!['^', '<', '<', 'A']);
    radiation_to_numeric.insert((10, 3),vec!['^', 'A']);
    radiation_to_numeric.insert((10, 4),vec!['^', '^', '<', '<', 'A']);
    radiation_to_numeric.insert((10, 9),vec!['^', '^', '^', 'A']);
    // TODO add the rest

    // For key move on radiation do steps on right on cold.
    let mut cold_to_radiation: HashMap<(char, char), Vec<char>> = HashMap::new();
    cold_to_radiation.insert(('A', '^'),vec!['<', 'A']);
    cold_to_radiation.insert(('A', '>'),vec!['v', 'A']);
    cold_to_radiation.insert(('A', 'v'),vec!['v', '<', 'A']);
    cold_to_radiation.insert(('A', '<'),vec!['v', '<', '<', 'A']);
    cold_to_radiation.insert(('A', 'A'),vec!['A']);
    cold_to_radiation.insert(('^', 'A'),vec!['>', 'A']);
    cold_to_radiation.insert(('^', 'v'),vec!['v', 'A']);
    cold_to_radiation.insert(('^', '<'),vec!['v', '<', 'A']);
    cold_to_radiation.insert(('^', '>'),vec!['v', '>', 'A']);
    cold_to_radiation.insert(('^', '^'),vec!['A']);
    cold_to_radiation.insert(('>', 'A'),vec!['^', 'A']);
    cold_to_radiation.insert(('>', 'v'),vec!['<', 'A']);
    cold_to_radiation.insert(('>', '<'),vec!['<', '<', 'A']);
    cold_to_radiation.insert(('>', '^'),vec!['<', '^', 'A']);
    cold_to_radiation.insert(('>', '>'),vec!['A']);
    cold_to_radiation.insert(('v', 'A'),vec!['^', '>', 'A']);
    cold_to_radiation.insert(('v', '^'),vec!['^', 'A']);
    cold_to_radiation.insert(('v', '<'),vec!['<', 'A']);
    cold_to_radiation.insert(('v', '>'),vec!['>', 'A']);
    cold_to_radiation.insert(('v', 'v'),vec!['A']);
    cold_to_radiation.insert(('<', 'A'),vec!['>', '>', '^', 'A']);
    cold_to_radiation.insert(('<', 'v'),vec!['>', 'A']);
    cold_to_radiation.insert(('<', '>'),vec!['>', '>', 'A']);
    cold_to_radiation.insert(('<', '^'),vec!['>', '^', 'A']);
    cold_to_radiation.insert(('<', '<'),vec!['A']);

    let mut total_complexity_solution_one = 0;
    for code in codes {
        println!("\nCODE: {code:?}");
        // Calculate Robot 1 -> Numeric.
        // Digit 0 -> 1
        let move_1 = radiation_to_numeric.get(&(code.digits[3], code.digits[0])).unwrap();
        // Digit 1 -> 2
        let move_2 = radiation_to_numeric.get(&(code.digits[0], code.digits[1])).unwrap();
        // Digit 2 -> 3
        let move_3 = radiation_to_numeric.get(&(code.digits[1], code.digits[2])).unwrap();
        // Digit 3 -> A
        let move_4 = radiation_to_numeric.get(&(code.digits[2], code.digits[3])).unwrap();
        let mut moveit = vec!['A'];
        moveit.extend(move_1);
        moveit.extend(move_2);
        moveit.extend(move_3);
        moveit.extend(move_4);

        println!("MOVEIT: len {}  {moveit:?}", moveit.len() - 1);
        let mut moveit2: Vec<char> = vec!['A'];
        for i in 0..(moveit.len() - 1) {
            let from = moveit[i];
            let to = moveit[i+1];
            let new_seq = cold_to_radiation.get(&(from, to)).unwrap();
            // println!("    {from} -> {to}   new seq {new_seq:?}");
            moveit2.extend(new_seq);
        }
        println!("MOVEIT2: len {}  {moveit2:?}", moveit2.len() - 1);

        let mut moveit3: Vec<char> = vec![];
        for i in 0..(moveit2.len() - 1) {
            let from = moveit2[i];
            let to = moveit2[i+1];
            // println!("    {from} -> {to}   new seq {new_seq:?}");
            let new_seq = cold_to_radiation.get(&(from, to)).unwrap();
            moveit3.extend(new_seq);
        }
        println!("MOVEIT3: len {}  {moveit3:?}", moveit3.len());

        total_complexity_solution_one += code.number * moveit3.len() as i32;
    }

    (Solution::I32(total_complexity_solution_one), Solution::I32(0))
}

use std::collections::HashMap;
use regex::Regex;
use aoc::board::Board;
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
    // Index is desired key on cold. Order U R D L.
    let mut you_to_cold: [[i32; 5]; 4] = [
        [0, 1, 0, 1, 2], // A -> U -> A
        [1, 0, 1, 0, 2], // A -> R -> A
        [1, 1, 1, 1, 4], // A -> D -> A
        [1, 2, 1, 2, 6], // A -> L -> A
    ];

    // Index is desired key on cold. Order U R D L.
    let mut cold_to_radiation: [[i32; 5]; 4] = [
        [0, 1, 0, 1, 2], // A -> U -> A
        [1, 0, 1, 0, 2], // A -> R -> A
        [1, 1, 1, 1, 4], // A -> D -> A
        [1, 2, 1, 2, 6], // A -> L -> A
    ];

    // Index is from-to on numeric. Order U R D L A.
    let mut radiation_to_numeric: HashMap<(i32, i32), [i32; 6]> = HashMap::new();
    radiation_to_numeric.insert((0, 2), [1, 0, 0, 0, 1, 1]);
    radiation_to_numeric.insert((0, 10), [0, 1, 0, 0, 1, 1]);
    radiation_to_numeric.insert((1, 6), [1, 2, 0, 0, 1, 3]);
    radiation_to_numeric.insert((1, 7), [2, 0, 0, 0, 1, 2]);
    radiation_to_numeric.insert((2, 9), [2, 1, 0, 0, 1, 3]);
    radiation_to_numeric.insert((3, 7), [2, 0, 0, 2, 1, 4]);
    radiation_to_numeric.insert((3, 8), [1, 0, 0, 2, 1, 3]);
    radiation_to_numeric.insert((3, 9), [2, 0, 0, 0, 1, 2]);
    radiation_to_numeric.insert((4, 5), [0, 1, 0, 0, 1, 1]);
    radiation_to_numeric.insert((5, 6), [0, 1, 0, 0, 1, 1]);
    radiation_to_numeric.insert((5, 7), [1, 0, 0, 1, 1, 2]);
    radiation_to_numeric.insert((6, 3), [0, 0, 1, 0, 1, 1]);
    radiation_to_numeric.insert((6, 7), [1, 0, 0, 2, 1, 3]);
    radiation_to_numeric.insert((6, 9), [1, 0, 0, 0, 1, 1]);
    radiation_to_numeric.insert((6, 10), [0, 0, 2, 0, 1, 2]);
    radiation_to_numeric.insert((7, 0), [0, 1, 3, 0, 1, 4]);
    radiation_to_numeric.insert((7, 9), [0, 2, 0, 0, 1, 2]);
    radiation_to_numeric.insert((8, 0), [0, 0, 3, 0, 1, 3]);
    radiation_to_numeric.insert((8, 3), [0, 1, 2, 0, 1, 3]);
    radiation_to_numeric.insert((8, 10), [0, 1, 3, 0, 1, 4]);
    radiation_to_numeric.insert((9, 8), [0, 0, 0, 1, 1, 1]);
    radiation_to_numeric.insert((9, 10), [0, 0, 3, 0, 1, 3]);
    radiation_to_numeric.insert((10, 0), [0, 0, 0, 1, 1, 1]);
    radiation_to_numeric.insert((10, 1), [1, 0, 0, 1, 1, 2]);
    radiation_to_numeric.insert((10, 3), [1, 0, 0, 0, 1, 1]);
    radiation_to_numeric.insert((10, 4), [2, 0, 0, 2, 1, 4]);
    radiation_to_numeric.insert((10, 5), [2, 0, 0, 1, 1, 3]);
    radiation_to_numeric.insert((10, 6), [2, 0, 0, 0, 1, 2]);
    radiation_to_numeric.insert((10, 8), [3, 0, 0, 1, 1, 4]);
    radiation_to_numeric.insert((10, 9), [3, 0, 0, 0, 1, 3]);
    // TODO finish this.

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

        let all_moves = vec![move_1, move_2, move_3, move_4].iter()
            .fold([0, 0, 0, 0, 0, 0], |acc, &[x, y, z, z2, z3, z4]|
                [acc[0] + x, acc[1] + y, acc[2] + z, acc[3] + z2, acc[4] + z3, acc[5] + z4]
            );
        // all_moves is now the number of  UDLRA required on the numeric keypad.
        println!("{move_1:?}  {move_2:?}  {move_3:?}  {move_4:?}    {all_moves:?}");

        let move_cr_u = cold_to_radiation[0].map(|x| x * all_moves[0]);
        let move_cr_r = cold_to_radiation[1].map(|x| x * all_moves[1]);
        let move_cr_d = cold_to_radiation[2].map(|x| x * all_moves[2]);
        let move_cr_l = cold_to_radiation[3].map(|x| x * all_moves[3]);

        let all_moves_cr = vec![move_cr_u, move_cr_r, move_cr_d, move_cr_l].iter()
            .fold([0, 0, 0, 0, 0], |acc, &[x, y, z, z2, z3]|
                [acc[0] + x, acc[1] + y, acc[2] + z, acc[3] + z2, acc[4] + z3]
            );
        println!("{move_cr_u:?}  {move_cr_r:?}  {move_cr_d:?}  {move_cr_l:?}    {all_moves_cr:?}");

        // let complexity = length_of_shortest_sequence * code.number;
        // total_complexity_solution_one += complexity;
    }

    (Solution::U32(total_complexity_solution_one), Solution::U32(0))
}

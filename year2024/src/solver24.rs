use aoc::solution::{Solution, Solutions};
use regex::Regex;
use std::collections::HashMap;

const OP_AND: &str = "AND";
const OP_OR: &str = "OR";
const OP_XOR: &str = "XOR";

const MAX_BITS: usize = 48;

#[derive(Clone, Debug)]
struct Output<'a> {
    name: &'a str,
    input1: &'a str,
    input2: &'a str,
    op: &'a str,
}

fn bit_array_to_int(bit_array: &[u8; MAX_BITS]) -> u64 {
    bit_array
        .iter()
        .fold(0, |acc, &bit| (acc << 1) | (bit as u64))
}

// fn int_to_bin_array(input: u64) -> [u8; MAX_BITS] {
//     let binary_vec: Vec<u8> = (0..MAX_BITS)
//         .rev()
//         .map(|i| ((input >> i) & 1) as u8)
//         .collect();
//     <[u8; MAX_BITS]>::try_from(&binary_vec[..]).unwrap()
// }

pub fn solve24(input: &[String]) -> Solutions {
    // let mut solved_wires: HashMap<&str, Wire> = HashMap::new();
    let mut solved_wires: HashMap<&str, u8> = HashMap::new();
    let mut unsolved_outputs: Vec<Output> = vec![];
    let mut x_inputs: [u8; MAX_BITS] = [0; MAX_BITS];
    let mut y_inputs: [u8; MAX_BITS] = [0; MAX_BITS];
    let mut solved_outputs: [u8; MAX_BITS] = [0; MAX_BITS];
    let mut reading_wires = true;

    let re_wire = Regex::new(r"([a-z0-9]+): ([0-9])$").unwrap();
    let re_gate = Regex::new(r"([a-z0-9]+) ([A-Z0-9]+) ([a-z0-9]+) -> ([a-z0-9]+)$").unwrap();

    for line in input {
        if reading_wires {
            if line.is_empty() {
                reading_wires = false;
            } else {
                let captures = re_wire.captures(line).unwrap();
                let name = captures.get(1).unwrap().as_str();
                let value = captures.get(2).unwrap().as_str().parse::<u8>().unwrap();
                solved_wires.insert(name, value);

                if name.starts_with('x') {
                    let i = name[1..3].parse::<usize>().unwrap();
                    x_inputs[MAX_BITS - 1 - i] = value;
                }
                if name.starts_with('y') {
                    let i = name[1..3].parse::<usize>().unwrap();
                    y_inputs[MAX_BITS - 1 - i] = value;
                }
            }
        } else {
            let captures = re_gate.captures(line).unwrap();
            let input1 = captures.get(1).unwrap().as_str();
            let op = captures.get(2).unwrap().as_str();
            let input2 = captures.get(3).unwrap().as_str();
            let name = captures.get(4).unwrap().as_str();
            unsolved_outputs.push(Output {
                name,
                input1,
                input2,
                op,
            });
        }
    }

    while !unsolved_outputs.is_empty() {
        let mut new_unsolved_outputs: Vec<Output> = vec![];

        for output in &unsolved_outputs {
            if solved_wires.contains_key(output.input1) && solved_wires.contains_key(output.input2)
            {
                let input1 = solved_wires.get(output.input1).unwrap();
                let input2 = solved_wires.get(output.input2).unwrap();
                let value = match output.op {
                    OP_AND => *input1 & *input2,
                    OP_OR => *input1 | *input2,
                    OP_XOR => *input1 ^ *input2,
                    _ => {
                        unreachable!()
                    }
                };

                solved_wires.insert(output.name, value);
                if output.name.starts_with('z') {
                    let i = output.name[1..3].parse::<usize>().unwrap();
                    solved_outputs[MAX_BITS - 1 - i] = value;
                }
            } else {
                new_unsolved_outputs.push(output.clone());
            }
        }

        unsolved_outputs = new_unsolved_outputs;
    }

    // let x_input = bit_array_to_int(&x_inputs);
    // let y_input = bit_array_to_int(&y_inputs);
    // let expected_output = x_input + y_input;
    let solution_one = bit_array_to_int(&solved_outputs);
    // println!("X INPUTS:     {x_inputs:?}");
    // println!("Y INPUTS:     {y_inputs:?}");
    // println!("\nSOLUTION ONE =     {solved_outputs:?}");
    // println!("EXPECTED OUTPUT =  {:?}", int_to_bin_array(expected_output));
    // println!("\nX =               {x_input}");
    // println!("Y =               {y_input}");
    // println!("EXPECTED OUTPUT = {expected_output}");
    // println!("SOLUTION ONE =    {solution_one}");

    // Hardcoded output because I found the solution by visual inspection in graphviz.
    // Four iterations of:
    // - Compare the binary expected and actual output to find the least significant incorrect
    //   binary digit.
    // - Visual inspection in of that unit in graphviz to spot the swapped outputs.
    // - Fix that in the input file.
    // Then manually sort the four pairs of output names into alphabetical order.
    //
    // Each "unit" consists of (except for slight variation in first and last bits.
    //
    // (Xn XOR Yn) XOR O(n-1) -> Zn
    // ( (Xn XOR Yn) AND O(n-1) ) OR (Xn AND Yn) -> On.
    //
    // It should be straightforward to convert this logic into code, removing the need for visual
    // inspection and manually determining each pair of swapped outputs.
    let solution_two = "gst,khg,nhn,tvb,vdc,z12,z21,z33";

    (
        Solution::U64(solution_one),
        Solution::STR(solution_two.to_string()),
    )
}

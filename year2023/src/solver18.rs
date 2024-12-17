use aoc::dir::Dir;
use aoc::solution::{Solution, Solutions};

#[derive(Debug)]
struct Instruction {
    dir: Dir,
    distance: i64,
}

pub fn solve18(input: &[String]) -> Solutions {
    (
        Solution::I64(solve(input, true)),
        Solution::I64(solve(input, false)),
    )
}
pub fn solve(input: &[String], is_part_one: bool) -> i64 {
    let mut instructions: Vec<Instruction> = vec![];

    if is_part_one {
        for line in input {
            let split_1: Vec<&str> = line.split(' ').collect();
            let dir = Dir::from_letter_str(split_1[0]);
            let distance = split_1[1].parse().unwrap();
            instructions.push(Instruction { dir, distance });
        }
    } else {
        for line in input {
            // TODO: Use regex.
            let split_1: Vec<&str> = line.split(' ').collect();
            let split_2: &str = split_1[2];
            let dir = Dir::from_int_str(&split_2[7..8]);
            let distance = i64::from_str_radix(&split_2[2..7], 16).unwrap();
            instructions.push(Instruction { dir, distance });
        }
    }

    area(&vertices(&instructions), path_len(&instructions))
}

fn vertices(instructions: &Vec<Instruction>) -> Vec<(i64, i64)> {
    let mut vertices: Vec<(i64, i64)> = vec![];

    let mut r: i64 = 0;
    let mut c: i64 = 0;
    for instruction in instructions {
        match instruction.dir {
            Dir::Up => r -= instruction.distance,
            Dir::Down => r += instruction.distance,
            Dir::Left => c -= instruction.distance,
            Dir::Right => c += instruction.distance,
        };
        vertices.push((r, c));
    }
    vertices
}

fn path_len(instructions: &Vec<Instruction>) -> i64 {
    let mut path_len = 0;

    for instruction in instructions {
        path_len += instruction.distance;
    }
    path_len
}

fn calculate_interior_points(area: i64, path_len: i64) -> i64 {
    (area as f64 - 0.5 * path_len as f64 + 1.0) as i64
}

// Combination of Pick's theorem and shoelace cobbled together from examples.
fn area(vertices: &[(i64, i64)], path_len: i64) -> i64 {
    let mut area = 0;
    for i in 1..vertices.len() {
        let x: i64 = vertices[i].0 * vertices[i - 1].1 - vertices[i - 1].0 * vertices[i].1;

        area += x;
    }
    area = i64::abs(area);
    area /= 2;
    let interior_area = calculate_interior_points(area, path_len);

    path_len + interior_area
}

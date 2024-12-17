const MIN_XY: f64 = 200_000_000_000_000.0;
const MAX_XY: f64 = 400_000_000_000_000.0;

use aoc::input::string_to_vec_i64;
use aoc::solution::{Solution, Solutions};
use std::cmp::Ordering;

#[derive(Clone, Debug)]
struct Hailstone {
    p: (i64, i64, i64),
    v: (i64, i64, i64),
    m: f64,
    c: f64,
}

fn read_input(input: &[String]) -> Vec<Hailstone> {
    let mut hailstones: Vec<Hailstone> = vec![];

    for line in input {
        let split_1: Vec<&str> = line.split(" @ ").collect();
        let p_vec = string_to_vec_i64(split_1[0], ',');
        let v_vec = string_to_vec_i64(split_1[1], ',');
        let p = (p_vec[0], p_vec[1], p_vec[2]);
        let v = (v_vec[0], v_vec[1], v_vec[2]);
        let m = v.1 as f64 / v.0 as f64;
        let hailstone = Hailstone {
            p,
            v,
            m,
            c: p.1 as f64 - m * p.0 as f64,
        };
        hailstones.push(hailstone);
    }
    hailstones
}

fn calculate_xy_intersects_in_future(hailstones: &mut [Hailstone]) -> Vec<(f64, f64)> {
    let mut intersects: Vec<(f64, f64)> = vec![];
    for id_1 in 0..hailstones.len() {
        for id_2 in 0..hailstones.len() {
            if id_1 < id_2 {
                let h_1 = &hailstones[id_1];
                let h_2 = &hailstones[id_2];

                let intersect_x = (h_2.c - h_1.c) / (h_1.m - h_2.m);
                let intersect_y = h_1.m * intersect_x + h_1.c;
                // Check (subject to float rounding).
                // let intersect_y2 = h_2.m * intersect_x + h_2.c;
                // One of the real inputs just fails with: 337194890889741.9   337194890889742.5
                // assert_eq!(intersect_y as i64, intersect_y2 as i64);
                let intersect_h1_in_future = match h_1.v.0.cmp(&0) {
                    Ordering::Greater => intersect_x as i64 > h_1.p.0,
                    Ordering::Less => (intersect_x as i64) < h_1.p.0,
                    Ordering::Equal => unreachable!(),
                };

                let intersect_h2_in_future = match h_2.v.0.cmp(&0) {
                    Ordering::Greater => intersect_x as i64 > h_2.p.0,
                    Ordering::Less => (intersect_x as i64) < h_2.p.0,
                    Ordering::Equal => unreachable!(),
                };

                if intersect_h1_in_future && intersect_h2_in_future {
                    intersects.push((intersect_x, intersect_y));
                }
            }
        }
    }
    intersects
}

fn count_xy_intersects_inside(min_xy: f64, max_xy: f64, intersects: &Vec<(f64, f64)>) -> u64 {
    let mut num_inside = 0;
    for intersect in intersects {
        if intersect.0 > min_xy
            && intersect.0 < max_xy
            && intersect.1 > min_xy
            && intersect.1 < max_xy
        {
            num_inside += 1;
        }
    }

    num_inside
}

fn solve_part_2(_hailstones: &[Hailstone]) -> i64 {
    // TODO
    0
}

pub fn solve24(input: &[String]) -> Solutions {
    let mut hailstones: Vec<Hailstone> = read_input(input);
    let intersects = calculate_xy_intersects_in_future(&mut hailstones);
    let part_1_solution = count_xy_intersects_inside(MIN_XY, MAX_XY, &intersects);

    let part_2_solution = solve_part_2(&hailstones);

    (
        Solution::U64(part_1_solution),
        Solution::I64(part_2_solution),
    )
}

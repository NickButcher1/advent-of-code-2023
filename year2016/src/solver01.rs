use aoc::infinite_grid::{START_CELL, START_GRID};
use aoc::solution::{Solution, Solutions};

pub fn solve_part_one(input: &[String]) -> u64 {
    let mut grid = START_GRID.clone();
    grid.make_moves(input[0].split(", ").collect::<Vec<&str>>());

    grid.taxicab_distance(&START_CELL)
}

pub fn solve_part_two(input: &[String]) -> u64 {
    let first_visited_twice_cell = START_GRID
        .clone()
        .make_moves_stop_when_any_cell_visited_twice(input[0].split(", ").collect::<Vec<&str>>());
    START_GRID.taxicab_distance(&first_visited_twice_cell)
}

pub fn solve01(input: &[String]) -> Solutions {
    (
        Solution::U64(solve_part_one(input)),
        Solution::U64(solve_part_two(input)),
    )
}

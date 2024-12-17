use aoc::dir::Dir;
use aoc::infinite_grid::InfiniteGrid;
use aoc::infinite_grid::{Cell, START_CELL, START_GRID};
use aoc::solution::{Solution, Solutions};
use std::collections::HashSet;

pub fn solve(input: &[String], num_santas: usize) -> usize {
    let mut visited_cells: HashSet<Cell> = HashSet::new();
    let mut grids: Vec<InfiniteGrid> = vec![START_GRID.clone(), START_GRID.clone()];
    visited_cells.insert(START_CELL);

    let mut i = 0;
    for c in input[0].chars() {
        grids[i].move_in_dir(Dir::from_arrow_char(c));
        visited_cells.insert(grids[i].pos);
        i += 1;
        i %= num_santas;
    }
    visited_cells.len()
}

pub fn solve03(input: &[String]) -> Solutions {
    (
        Solution::USIZE(solve(input, 1)),
        Solution::USIZE(solve(input, 2)),
    )
}

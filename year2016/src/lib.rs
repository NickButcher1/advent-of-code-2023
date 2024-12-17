pub mod solver01;
pub mod solver02;
pub mod solver03;

use crate::solver01::solve01;
use crate::solver02::solve02;
use crate::solver03::solve03;
use aoc::solution::{solve_blank, SolverFunction};

pub const SOLVER_FUNCTIONS: [SolverFunction; 25] = [
    solve01,
    solve02,
    solve03,
    solve_blank,
    solve_blank,
    solve_blank,
    solve_blank,
    solve_blank,
    solve_blank,
    solve_blank,
    solve_blank,
    solve_blank,
    solve_blank,
    solve_blank,
    solve_blank,
    solve_blank,
    solve_blank,
    solve_blank,
    solve_blank,
    solve_blank,
    solve_blank,
    solve_blank,
    solve_blank,
    solve_blank,
    solve_blank,
];

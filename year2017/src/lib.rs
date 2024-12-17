pub mod solver01;
pub mod solver02;
pub mod solver04;
pub mod solver05;
pub mod solver20;

use crate::solver01::solve01;
use crate::solver02::solve02;
use crate::solver04::solve04;
use crate::solver05::solve05;
use crate::solver20::solve20;
use aoc::solution::{solve_blank, SolverFunction};

pub const SOLVER_FUNCTIONS: [SolverFunction; 25] = [
    solve01,
    solve02,
    solve_blank,
    solve04,
    solve05,
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
    solve20,
    solve_blank,
    solve_blank,
    solve_blank,
    solve_blank,
    solve_blank,
];

pub mod solver01;
pub mod solver02;
pub mod solver20;

use crate::solver01::solve01;
use crate::solver02::solve02;
use crate::solver20::solve20;

type SolverFunction = fn(&[String]) -> (i128, i128);

pub const fn solve_blank(_input: &[String]) -> (i128, i128) {
    (0, 0)
}

pub const SOLVER_FUNCTIONS: [SolverFunction; 25] = [
    solve01,
    solve02,
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
    solve20,
    solve_blank,
    solve_blank,
    solve_blank,
    solve_blank,
    solve_blank,
];

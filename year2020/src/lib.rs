pub mod solver01;
pub mod solver02;
pub mod solver03;
pub mod solver11;

use crate::solver01::solve01;
use crate::solver02::solve02;
use crate::solver03::solve03;
use crate::solver11::solve11;

type SolverFunction = fn(&[String]) -> (i128, i128);

pub const fn solve_blank(_input: &[String]) -> (i128, i128) {
    (0, 0)
}

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
    solve11,
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

pub mod solver01;
pub mod solver02;
pub mod solver03;
pub mod solver05;
pub mod solver06;
pub mod solver10;
pub mod solver17;
pub mod solver18;

use crate::solver01::solve01;
use crate::solver02::solve02;
use crate::solver03::solve03;
use crate::solver05::solve05;
use crate::solver06::solve06;
use crate::solver10::solve10;
use crate::solver17::solve17;
use crate::solver18::solve18;
use aoc::solution::{solve_blank, SolverFunction};

pub const SOLVER_FUNCTIONS: [SolverFunction; 25] = [
    solve01,
    solve02,
    solve03,
    solve_blank,
    solve05,
    solve06,
    solve_blank,
    solve_blank,
    solve_blank,
    solve10,
    solve_blank,
    solve_blank,
    solve_blank,
    solve_blank,
    solve_blank,
    solve_blank,
    solve17,
    solve18,
    solve_blank,
    solve_blank,
    solve_blank,
    solve_blank,
    solve_blank,
    solve_blank,
    solve_blank,
];

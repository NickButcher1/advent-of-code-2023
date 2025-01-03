pub mod solver01;
pub mod solver02;
pub mod solver03;
pub mod solver04;
pub mod solver05;
pub mod solver06;
pub mod solver10;
pub mod solver13;
pub mod solver14;
pub mod solver15;
pub mod solver18;
pub mod solver20;
pub mod solver23;

use crate::solver01::solve01;
use crate::solver02::solve02;
use crate::solver03::solve03;
use crate::solver04::solve04;
use crate::solver05::solve05;
use crate::solver06::solve06;
use crate::solver10::solve10;
use crate::solver13::solve13;
use crate::solver14::solve14;
use crate::solver15::solve15;
use crate::solver18::solve18;
use crate::solver20::solve20;
use crate::solver23::solve23;
use aoc::solution::{solve_blank, SolverFunction};

pub const SOLVER_FUNCTIONS: [SolverFunction; 25] = [
    solve01,
    solve02,
    solve03,
    solve04,
    solve05,
    solve06,
    solve_blank,
    solve_blank,
    solve_blank,
    solve10,
    solve_blank,
    solve_blank,
    solve13,
    solve14,
    solve15,
    solve_blank,
    solve_blank,
    solve18,
    solve_blank,
    solve20,
    solve_blank,
    solve_blank,
    solve23,
    solve_blank,
    solve_blank,
];

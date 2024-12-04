pub mod solver01;
pub mod solver03;
pub mod solver04;
pub mod solver05;
pub mod solver09;
pub mod solver10;
pub mod solver15;
pub mod solver21;
pub mod solver25;

use crate::solver01::solve01;
use crate::solver03::solve03;
use crate::solver04::solve04;
use crate::solver05::solve05;
use crate::solver09::solve09;
use crate::solver10::solve10;
use crate::solver15::solve15;
use crate::solver21::solve21;
use crate::solver25::solve25;

type SolverFunction = fn(&[String]) -> (i128, i128);

pub const fn solve_blank(_input: &[String]) -> (i128, i128) {
    (0, 0)
}

pub const SOLVER_FUNCTIONS: [SolverFunction; 25] = [
    solve01,
    solve_blank,
    solve03,
    solve04,
    solve05,
    solve_blank,
    solve_blank,
    solve_blank,
    solve09,
    solve10,
    solve_blank,
    solve_blank,
    solve_blank,
    solve_blank,
    solve15,
    solve_blank,
    solve_blank,
    solve_blank,
    solve_blank,
    solve_blank,
    solve21,
    solve_blank,
    solve_blank,
    solve_blank,
    solve25,
];

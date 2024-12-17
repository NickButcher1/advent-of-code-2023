pub mod solver01;
pub mod solver02;
pub mod solver03;
pub mod solver04;
pub mod solver05;
pub mod solver06;
pub mod solver07;
pub mod solver08;
pub mod solver09;
pub mod solver10;
pub mod solver11;
pub mod solver12;
pub mod solver13;
pub mod solver14;
pub mod solver15;
pub mod solver16;
pub mod solver17;
pub mod solver18;
pub mod solver19;
pub mod solver20;
pub mod solver21;
pub mod solver22;
pub mod solver23;
pub mod solver24;
pub mod solver25;

use crate::solver01::solve01;
use crate::solver02::solve02;
use crate::solver03::solve03;
use crate::solver04::solve04;
use crate::solver05::solve05;
use crate::solver06::solve06;
use crate::solver07::solve07;
use crate::solver08::solve08;
use crate::solver09::solve09;
use crate::solver10::solve10;
use crate::solver11::solve11;
use crate::solver12::solve12;
use crate::solver13::solve13;
use crate::solver14::solve14;
use crate::solver15::solve15;
use crate::solver16::solve16;
use crate::solver17::solve17;
use crate::solver18::solve18;
use crate::solver19::solve19;
use crate::solver20::solve20;
use crate::solver21::solve21;
use crate::solver22::solve22;
use crate::solver23::solve23;
use crate::solver24::solve24;
use crate::solver25::solve25;
use aoc::solution::SolverFunction;

pub const SOLVER_FUNCTIONS: [SolverFunction; 25] = [
    solve01, solve02, solve03, solve04, solve05, solve06, solve07, solve08, solve09, solve10,
    solve11, solve12, solve13, solve14, solve15, solve16, solve17, solve18, solve19, solve20,
    solve21, solve22, solve23, solve24, solve25,
];

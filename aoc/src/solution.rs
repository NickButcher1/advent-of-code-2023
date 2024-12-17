use std::fmt;
use std::fmt::{Display, Formatter};

pub type Solutions = (Solution, Solution);

#[derive(Debug)]
pub enum Solution {
    U32(u32),
    I32(i32),
    U64(u64),
    I64(i64),
    I128(i128),
    USIZE(usize),
    ISIZE(isize),
    STR(String),
}

impl Display for Solution {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            // TODO Collapse into one.
            Solution::U32(val) => write!(f, "{}", *val),
            Solution::I32(val) => write!(f, "{}", *val),
            Solution::U64(val) => write!(f, "{}", *val),
            Solution::I64(val) => write!(f, "{}", *val),
            Solution::I128(val) => write!(f, "{}", *val),
            Solution::USIZE(val) => write!(f, "{}", *val),
            Solution::ISIZE(val) => write!(f, "{}", *val),
            Solution::STR(val) => write!(f, "{}", *val),
        }
    }
}

pub type SolverFunction = fn(&[String]) -> Solutions;

pub const fn solve_blank(_input: &[String]) -> Solutions {
    (Solution::U64(0), Solution::U64(0))
}

use aoc::solution::{Solution, Solutions};
use md5;

pub fn solve04(input: &[String]) -> Solutions {
    let key = &input[0];
    let mut test_val = 0;
    let mut part_1 = 0;

    loop {
        test_val += 1;
        let hash = md5::compute(format!("{key}{test_val}"));
        if hash[0] == 0 && hash[1] == 0 {
            if part_1 == 0 && hash[2] < 16 {
                part_1 = test_val
            }
            if hash[2] == 0 {
                break;
            }
        }
    }
    (Solution::I32(part_1), Solution::I32(test_val))
}

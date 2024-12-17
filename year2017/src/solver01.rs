use aoc::solution::{Solution, Solutions};

pub fn solve01(input: &[String]) -> Solutions {
    let chars: Vec<char> = input[0].chars().collect();
    (
        Solution::U32(solve(&chars, 1)),
        Solution::U32(solve(&chars, chars.len() / 2)),
    )
}

fn solve(chars: &[char], offset: usize) -> u32 {
    (0..chars.len()).fold(0, |total, i| {
        if chars[i] == chars[(i + offset) % chars.len()] {
            total + chars[i].to_digit(10).unwrap()
        } else {
            total
        }
    })
}

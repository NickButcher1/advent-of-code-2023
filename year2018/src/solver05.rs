use aoc::solution::{Solution, Solutions};

// a-z is 97-122
// A-Z is 65-90
fn is_match(c1: u8, c2: u8) -> bool {
    if c1 <= 90 && c2 >= 97 {
        (c2 - c1) == 32
    } else if c2 <= 90 && c1 >= 97 {
        (c1 - c2) == 32
    } else {
        false
    }
}

fn reduce_input(mut chars: Vec<u8>) -> usize {
    loop {
        let mut new_chars = vec![];
        let mut i = 0;

        while i < chars.len() {
            if i == chars.len() - 1 {
                new_chars.push(chars[i]);
                i += 1;
            } else if is_match(chars[i], chars[i + 1]) {
                i += 2;
            } else {
                new_chars.push(chars[i]);
                i += 1;
            }
        }

        if chars.len() == new_chars.len() {
            break;
        }
        chars = new_chars;
    }

    chars.len()
}

pub fn solve05(input: &[String]) -> Solutions {
    let chars = input[0].chars().map(|c| c as u8).collect::<Vec<u8>>();

    let solution_one = reduce_input(chars.clone());

    let solution_two = (65..=90)
        .map(|i| {
            let mut new_chars = chars.clone();
            new_chars.retain(|&c| c != i && c != (i + 32));
            reduce_input(new_chars)
        })
        .min()
        .unwrap_or(usize::MAX);

    (Solution::USIZE(solution_one), Solution::USIZE(solution_two))
}

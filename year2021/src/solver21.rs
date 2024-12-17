use aoc::solution::{Solution, Solutions};
use itertools::iproduct;
use regex::Regex;
use std::cmp::max;
use std::collections::HashMap;

pub fn solve_part_one(p1_start_pos: i32, p2_start_pos: i32) -> i128 {
    let mut p1_pos = p1_start_pos;
    let mut p2_pos = p2_start_pos;
    let mut p1_score = 0;
    let mut p2_score = 0;
    let mut next_dice = 1;
    let mut is_p1_turn = true;
    let mut dice_rolls = 0;

    loop {
        dice_rolls += 3;
        let mut dice_total = next_dice;
        next_dice += 1;
        if next_dice == 101 {
            next_dice = 1;
        }
        dice_total += next_dice;
        next_dice += 1;
        if next_dice == 101 {
            next_dice = 1;
        }
        dice_total += next_dice;
        next_dice += 1;
        if next_dice == 101 {
            next_dice = 1;
        }
        let reduced_dice_total = dice_total % 10;

        if is_p1_turn {
            p1_pos += reduced_dice_total;
            p1_pos = ((p1_pos - 1) % 10) + 1;
            p1_score += p1_pos;
        } else {
            p2_pos += reduced_dice_total;
            p2_pos = ((p2_pos - 1) % 10) + 1;
            p2_score += p2_pos;
        }

        if p1_score >= 1000 || p2_score >= 1000 {
            break;
        }

        is_p1_turn = !is_p1_turn;
    }

    let solution_one = if is_p1_turn {
        dice_rolls * p2_score
    } else {
        dice_rolls * p1_score
    };

    solution_one as i128
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct State {
    pub p1_pos: i32,
    pub p2_pos: i32,
    pub p1_score: i32,
    pub p2_score: i32,
}

pub fn solve_part_two(p1_start_pos: i32, p2_start_pos: i32) -> i128 {
    let mut states: HashMap<State, u64> = HashMap::new();
    let x = State {
        p1_pos: p1_start_pos,
        p2_pos: p2_start_pos,
        p1_score: 0,
        p2_score: 0,
    };
    states.insert(x, 1);

    let mut p1_wins: u64 = 0;
    let mut p2_wins: u64 = 0;

    loop {
        if states.is_empty() {
            break;
        }
        let old_state_orig = states.keys().next().unwrap();
        let (old_state, old_count) = states.remove_entry(&old_state_orig.clone()).unwrap();

        for (dice_roll_p1_1, dice_roll_p1_2, dice_roll_p1_3) in iproduct!(1..=3, 1..=3, 1..=3) {
            let dice_roll_p1 = dice_roll_p1_1 + dice_roll_p1_2 + dice_roll_p1_3;
            let p1_new_pos = ((old_state.p1_pos + dice_roll_p1 - 1) % 10) + 1;
            let intermediate_state = State {
                p1_pos: p1_new_pos,
                p2_pos: old_state.p2_pos,
                p1_score: old_state.p1_score + p1_new_pos,
                p2_score: old_state.p2_score,
            };
            if intermediate_state.p1_score >= 21 {
                p1_wins += old_count;
            } else {
                for (dice_roll_p2_1, dice_roll_p2_2, dice_roll_p2_3) in
                    iproduct!(1..=3, 1..=3, 1..=3)
                {
                    let dice_roll_p2 = dice_roll_p2_1 + dice_roll_p2_2 + dice_roll_p2_3;
                    let p2_new_pos = ((intermediate_state.p2_pos + dice_roll_p2 - 1) % 10) + 1;
                    let new_state = State {
                        p1_pos: intermediate_state.p1_pos,
                        p2_pos: p2_new_pos,
                        p1_score: intermediate_state.p1_score,
                        p2_score: intermediate_state.p2_score + p2_new_pos,
                    };
                    if new_state.p2_score >= 21 {
                        p2_wins += old_count;
                    } else {
                        match states.get(&new_state) {
                            Some(value) => {
                                let new_count = value + old_count;
                                states.insert(new_state, new_count);
                            }
                            None => {
                                let new_count = old_count;
                                states.insert(new_state, new_count);
                            }
                        }
                    }
                }
            }
        }
    }

    max(p1_wins, p2_wins) as i128
}

pub fn solve21(input: &[String]) -> Solutions {
    let re = Regex::new(r"^Player (\d+) starting position: (\d+)").unwrap();
    let captures = re.captures(&input[0]).unwrap();
    let p1_pos = captures.get(2).unwrap().as_str().parse::<i32>().unwrap();
    let captures = re.captures(&input[1]).unwrap();
    let p2_pos = captures.get(2).unwrap().as_str().parse::<i32>().unwrap();

    (
        Solution::I128(solve_part_one(p1_pos, p2_pos)),
        Solution::I128(solve_part_two(p1_pos, p2_pos)),
    )
}

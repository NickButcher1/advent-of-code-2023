use aoc::input::string_to_vec_usize;

pub fn solve02(input: &[String]) -> (i128, i128) {
    let codes = string_to_vec_usize(&input[0], ',');
    (
        solve(&mut codes.clone(), 12, 2) as i128,
        solve_part_2(&codes) as i128,
    )
}

pub fn solve(codes: &mut Vec<usize>, noun: usize, verb: usize) -> usize {
    codes[1] = noun;
    codes[2] = verb;

    let mut pos = 0;
    loop {
        match codes[pos] {
            1 => {
                let new_value = codes[codes[pos + 1]] + codes[codes[pos + 2]];
                let new_idx = codes[pos + 3];
                codes[new_idx] = new_value;
            }
            2 => {
                let new_value = codes[codes[pos + 1]] * codes[codes[pos + 2]];
                let new_idx = codes[pos + 3];
                codes[new_idx] = new_value;
            }
            99 => return codes[0],
            _ => unreachable!(),
        }
        pos = (pos + 4) % codes.len();
    }
}

pub fn solve_part_2(codes: &[usize]) -> usize {
    for noun in 0..=99 {
        for verb in 0..=99 {
            if solve(&mut codes.to_owned(), noun, verb) == 19690720 {
                return 100 * noun + verb;
            }
        }
    }
    unreachable!();
}

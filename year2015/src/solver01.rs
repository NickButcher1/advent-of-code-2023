pub fn solve01(input: &[String]) -> (i128, i128) {
    let (up, down): (Vec<char>, Vec<char>) = input[0].chars().partition(|c| *c == '(');
    let part_1_floor = up.len() - down.len();

    // TODO: Terminate the fold() early.
    let part_2_steps = input[0]
        .chars()
        .fold((0, 0, false), |(steps, floor, mut solved), c| {
            if solved {
                (steps, floor, solved)
            } else if c == '(' {
                (steps + 1, floor + 1, solved)
            } else if floor == 0 {
                solved = true;
                (steps + 1, floor - 1, solved)
            } else {
                (steps + 1, floor - 1, solved)
            }
        })
        .0;

    (part_1_floor as i128, part_2_steps as i128)
}

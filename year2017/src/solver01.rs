pub fn solve01(input: &[String]) -> (i128, i128) {
    let chars: Vec<char> = input[0].chars().collect();
    (
        i128::from(solve(&chars, 1)),
        i128::from(solve(&chars, chars.len() / 2)),
    )
}

fn solve(chars: &Vec<char>, offset: usize) -> u32 {
    (0..chars.len()).fold(0, |total, i| {
        if chars[i] == chars[(i + offset) % chars.len()] {
            total + chars[i].to_digit(10).unwrap()
        } else {
            total
        }
    })
}

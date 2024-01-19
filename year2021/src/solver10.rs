use aoc::input::to_vec_char;

pub fn solve10(input: &[String]) -> (i128, i128) {
    let mut score = 0;
    let lines = to_vec_char(input);
    lines.iter().for_each(|line| {
        let mut stack: Vec<char> = vec![];
        for c in line {
            match c {
                '(' => stack.push(*c),
                '[' => stack.push(*c),
                '{' => stack.push(*c),
                '<' => stack.push(*c),
                ')' => {
                    let old_c = stack.pop().unwrap();
                    if old_c != '(' {
                        score += 3;
                        break;
                    }
                }
                ']' => {
                    let old_c = stack.pop().unwrap();
                    if old_c != '[' {
                        score += 57;
                        break;
                    }
                }
                '}' => {
                    let old_c = stack.pop().unwrap();
                    if old_c != '{' {
                        score += 1_197;
                        break;
                    }
                }
                '>' => {
                    let old_c = stack.pop().unwrap();
                    if old_c != '<' {
                        score += 25_137;
                        break;
                    }
                }
                _ => unreachable!(),
            }
        }
    });
    (score as i128, 0)
}

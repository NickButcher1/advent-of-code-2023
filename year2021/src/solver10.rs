use aoc::input::to_vec_char;

pub fn solve10(input: &[String]) -> (i128, i128) {
    let mut part_one_score = 0;
    let lines = to_vec_char(input);
    let mut incomplete_lines: Vec<Vec<char>> = vec![];

    lines.iter().for_each(|line| {
        let mut stack: Vec<char> = vec![];
        let mut line_is_corrupt = false;
        for c in line {
            match c {
                '(' => stack.push(*c),
                '[' => stack.push(*c),
                '{' => stack.push(*c),
                '<' => stack.push(*c),
                ')' => {
                    let old_c = stack.pop().unwrap();
                    if old_c != '(' {
                        part_one_score += 3;
                        line_is_corrupt = true;
                        break;
                    }
                }
                ']' => {
                    let old_c = stack.pop().unwrap();
                    if old_c != '[' {
                        part_one_score += 57;
                        line_is_corrupt = true;
                        break;
                    }
                }
                '}' => {
                    let old_c = stack.pop().unwrap();
                    if old_c != '{' {
                        part_one_score += 1_197;
                        line_is_corrupt = true;
                        break;
                    }
                }
                '>' => {
                    let old_c = stack.pop().unwrap();
                    if old_c != '<' {
                        part_one_score += 25_137;
                        line_is_corrupt = true;
                        break;
                    }
                }
                _ => unreachable!(),
            }
        }
        if !line_is_corrupt {
            incomplete_lines.push(stack);
        }
    });

    let mut scores: Vec<u64> = vec![];

    incomplete_lines.iter().for_each(|line| {
        let mut score: u64 = 0;
        for c in line.iter().rev() {
            score *= 5;
            score += match c {
                '(' => 1,
                '[' => 2,
                '{' => 3,
                '<' => 4,
                _ => unreachable!(),
            }
        }
        scores.push(score);
    });
    scores.sort();
    let part_two_score = scores[(scores.len() - 1) / 2];

    (part_one_score as i128, part_two_score as i128)
}

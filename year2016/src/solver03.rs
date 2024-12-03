use regex::Regex;

fn capture_line(re: &Regex, line: &String) -> (usize, usize, usize) {
    let captures = re.captures(line).unwrap();
    (
        captures.get(2).unwrap().as_str().parse::<usize>().unwrap(),
        captures.get(4).unwrap().as_str().parse::<usize>().unwrap(),
        captures.get(6).unwrap().as_str().parse::<usize>().unwrap(),
    )
}
fn is_valid_triangle(l1: usize, l2: usize, l3: usize) -> bool {
    (l1 + l2) > l3 && (l2 + l3) > l1 && (l1 + l3) > l2
}
fn solve_part_one(input_values: &Vec<(usize, usize, usize)>) -> i128 {
    let num_valid_triangles = input_values
        .iter()
        .filter(|&&(l1, l2, l3)| is_valid_triangle(l1, l2, l3))
        .count();

    num_valid_triangles as i128
}

fn solve_part_two(input_values: &[(usize, usize, usize)]) -> i128 {
    (0..input_values.len())
        .step_by(3)
        .map(|i| {
            let l1 = input_values[i];
            let l2 = input_values[i + 1];
            let l3 = input_values[i + 2];

            [(l1.0, l2.0, l3.0), (l1.1, l2.1, l3.1), (l1.2, l2.2, l3.2)]
                .iter()
                .filter(|&&(a, b, c)| is_valid_triangle(a, b, c))
                .count() as i128
        })
        .sum()
}

pub fn solve03(input: &[String]) -> (i128, i128) {
    let re = Regex::new(r"^(\s+)(\d+)(\s+)(\d+)(\s+)(\d+)$").unwrap();

    let input_values: Vec<(usize, usize, usize)> =
        input.iter().map(|line| capture_line(&re, line)).collect();

    (solve_part_one(&input_values), solve_part_two(&input_values))
}

use aoc::input::to_vec_char;

fn count_at_index(input: &[Vec<char>], index: usize) -> (i32, i32) {
    input.iter().fold((0, 0), |(zeroes, ones), line| {
        if line[index] == '0' {
            (zeroes + 1, ones)
        } else {
            (zeroes, ones + 1)
        }
    })
}

fn solve_part_1(input: &[Vec<char>]) -> u64 {
    let mut gamma = 0;
    let mut epsilon = 0;
    for i in 0..input[0].len() {
        let (zeroes, ones) = count_at_index(input, i);
        gamma *= 2;
        epsilon *= 2;
        if ones >= zeroes {
            gamma += 1;
        } else {
            epsilon += 1;
        }
    }
    gamma * epsilon
}

fn to_integer(input: &[char]) -> u64 {
    input
        .iter()
        .fold(0, |acc, c| if *c == '1' { acc * 2 + 1 } else { acc * 2 })
}

fn solve_oxygen_or_co2(input: &Vec<Vec<char>>, is_oxygen: bool) -> u64 {
    let mut numbers = input.to_owned();

    for i in 0..numbers[0].len() {
        let (zeroes, ones) = count_at_index(&numbers, i);
        numbers = if (is_oxygen && ones >= zeroes) || (!is_oxygen && zeroes > ones) {
            numbers
                .into_iter()
                .filter(|number| number[i] == '1')
                .collect()
        } else {
            numbers
                .into_iter()
                .filter(|number| number[i] == '0')
                .collect()
        };
        if numbers.len() < 2 {
            break;
        }
    }
    to_integer(&numbers[0])
}

fn solve_part_2(input: &Vec<Vec<char>>) -> u64 {
    solve_oxygen_or_co2(input, true) * solve_oxygen_or_co2(input, false)
}

pub fn solve03(input: &[String]) -> (i128, i128) {
    (
        solve_part_1(&to_vec_char(input)) as i128,
        solve_part_2(&to_vec_char(input)) as i128,
    )
}

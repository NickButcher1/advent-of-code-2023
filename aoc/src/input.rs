pub fn to_vec_u64(input: &[String]) -> Vec<u64> {
    input.iter().map(|x| x.parse::<u64>().unwrap()).collect()
}

pub fn to_vec_i64(input: &[String]) -> Vec<i64> {
    input.iter().map(|x| x.parse::<i64>().unwrap()).collect()
}

pub fn to_vec_char(input: &[String]) -> Vec<Vec<char>> {
    input
        .iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect()
}

#[allow(dead_code)]
pub fn string_to_vec_u64_ignore_prefix(prefix: &str, input: &str) -> Vec<u64> {
    let split_1: Vec<&str> = input.split(prefix).collect();
    string_to_vec_u64(split_1[1], ' ')
}

#[allow(dead_code)]
pub fn split_string_to_u64(input: &str, split_on: char, index: usize) -> u64 {
    input.split(split_on).collect::<Vec<&str>>()[index]
        .parse::<u64>()
        .unwrap()
}

pub fn string_to_vec_u64(input: &str, split_on: char) -> Vec<u64> {
    input
        .split(split_on)
        .filter(|s| !s.is_empty())
        .map(|x| x.parse::<u64>().unwrap())
        .collect()
}

#[allow(dead_code)]
pub fn string_to_vec_i64(input: &str, split_on: char) -> Vec<i64> {
    input
        .split(split_on)
        .map(|x| x.trim().parse::<i64>().unwrap())
        .collect()
}

#[allow(dead_code)]
pub fn string_to_vec_usize(input: &str, split_on: char) -> Vec<usize> {
    input
        .split(split_on)
        .map(|s| s.parse::<usize>().unwrap())
        .collect()
}

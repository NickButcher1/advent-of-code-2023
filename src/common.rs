pub fn string_to_vec_u64_ignore_prefix(prefix: &str, input: &str) -> Vec<u64> {
    let split_1: Vec<&str> = input.split(prefix).collect();
    string_to_vec_u64(split_1[1])
}

pub fn string_to_vec_u64(input: &str) -> Vec<u64> {
    input
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(|x| x.parse::<u64>().unwrap())
        .collect()
}

pub fn string_to_vec_i64(input: &str) -> Vec<i64> {
    input
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(|x| x.parse::<i64>().unwrap())
        .collect()
}

pub fn split_string_to_u64(input: &str, split_on: char, index: usize) -> u64 {
    input.split(split_on).collect::<Vec<&str>>()[index]
        .parse::<u64>()
        .unwrap()
}

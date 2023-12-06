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

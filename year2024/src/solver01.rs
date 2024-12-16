use regex::Regex;

pub fn solve01(input: &[String]) -> (i128, i128) {
    let mut list1: Vec<u64> = Vec::new();
    let mut list2: Vec<u64> = Vec::new();

    let re = Regex::new(r"\s+").unwrap();

    for line in input {
        let numbers: Vec<u64> = re
            .split(line)
            .filter_map(|s| s.parse::<u64>().ok())
            .collect();
        list1.push(numbers[0]);
        list2.push(numbers[1]);
    }

    list1.sort();
    list2.sort();

    let total_distance: u64 = list1
        .iter()
        .zip(list2.iter())
        .map(|(&item1, &item2)| (item1 as i64 - item2 as i64).unsigned_abs())
        .sum();

    let total_similarity: u64 = list1
        .iter()
        .map(|&item| item * list2.iter().filter(|&&x| x == item).count() as u64)
        .sum();

    (total_distance as i128, total_similarity as i128)
}

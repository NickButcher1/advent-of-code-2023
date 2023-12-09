use crate::common::string_to_vec_i64;

pub fn solve09(input: Vec<String>) -> (i128, i128) {
    let mut total_part_1: i64 = 0;
    let mut total_part_2: i64 = 0;

    for line in input {
        let mut ints = string_to_vec_i64(&line);
        let mut heads_of_lists: Vec<i64> = vec![ints[0]];
        let mut tails_of_lists: Vec<i64> = vec![*ints.last().unwrap()];

        while !ints.iter().all(|&x| x == 0) {
            ints = (0..ints.len() - 1).map(|i| ints[i + 1] - ints[i]).collect();
            heads_of_lists.push(ints[0]);
            tails_of_lists.push(*ints.last().unwrap());
        }

        for i in (0..(heads_of_lists.len() - 1)).rev() {
            heads_of_lists[i] -= heads_of_lists[i + 1];
            tails_of_lists[i] += tails_of_lists[i + 1];
        }

        total_part_1 += tails_of_lists[0];
        total_part_2 += heads_of_lists[0];
    }

    (total_part_1 as i128, total_part_2 as i128)
}

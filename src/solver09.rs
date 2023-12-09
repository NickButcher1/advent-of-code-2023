use crate::common::string_to_vec_i64;

pub fn solve09(input: Vec<String>) -> (i128, i128) {
    let mut total_part_1: i64 = 0;
    let mut total_part_2: i64 = 0;

    for line in input {
        let mut ints = string_to_vec_i64(&line);
        let mut list_of_lists = vec![ints.clone()];

        while !ints.iter().all(|&x| x == 0) {
            let mut new_ints: Vec<i64> = Vec::new();
            for i in 0..(ints.len() - 1) {
                new_ints.push(ints[i + 1] - ints[i]);
            }
            ints = new_ints.clone();
            list_of_lists.push(new_ints.clone());
        }

        for i in (0..(list_of_lists.len() - 1)).rev() {
            let push_value =
                list_of_lists[i].last().unwrap() + list_of_lists[i + 1].last().unwrap();
            let insert_value = list_of_lists[i][0] - list_of_lists[i + 1][0];
            list_of_lists[i].push(push_value);
            list_of_lists[i].insert(0, insert_value);
        }

        total_part_1 += list_of_lists[0].last().unwrap();
        total_part_2 += list_of_lists[0][0];
    }

    (total_part_1 as i128, total_part_2 as i128)
}

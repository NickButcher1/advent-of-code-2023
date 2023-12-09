use crate::common::string_to_vec_i64;

// Build up a list of integers. Start from the input, then sum adjecent pairs to make a new list,
// one shorter in length than the previous list. Repeat until the new list is all zeroes.
//
// We then need to add a new value to the end of each list, but there is an optimisation here -
// simply sum the last value in each list, which gives the new last value of the first list. That's
// the score we need for this input.
//
// Part 2 can be solved the same way as part 1 - the caller must simply reverse the input.
pub fn score_line(mut ints: Vec<i64>) -> i64 {
    let mut tails_of_lists: Vec<i64> = Vec::new();

    while !ints.iter().all(|&x| x == 0) {
        tails_of_lists.push(*ints.last().unwrap());
        ints = (0..ints.len() - 1).map(|i| ints[i + 1] - ints[i]).collect();
    }

    tails_of_lists.iter().sum::<i64>()
}

pub fn solve09(input: Vec<String>) -> (i128, i128) {
    (
        input
            .iter()
            .map(|line| score_line(string_to_vec_i64(line)) as i128)
            .sum(),
        input
            .iter()
            .map(|line| {
                score_line(
                    string_to_vec_i64(line)
                        .into_iter()
                        .rev()
                        .collect::<Vec<i64>>(),
                ) as i128
            })
            .sum(),
    )
}

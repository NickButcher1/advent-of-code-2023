use crate::common::string_to_vec_u64_ignore_prefix;

fn solve(times: Vec<u64>, distances: &[u64]) -> i128 {
    let mut total = 1;
    for i in 0..times.len() {
        let race_time = times[i];
        let record_distance = distances[i];
        let mut ways = 0;

        for my_time in 0..race_time {
            let my_speed = my_time;
            let my_distance = my_speed * (race_time - my_time);
            if my_distance > record_distance {
                ways += 1;
            }
        }
        total *= ways;
    }

    total
}

pub fn solve06(input: Vec<String>) -> (i128, i128) {
    let times_part_1 = string_to_vec_u64_ignore_prefix("Time:", &input[0]);
    let times_line: Vec<&str> = input[0].split("Time:").collect();
    let times_part_2: Vec<u64> = times_line[1]
        .replace(' ', "")
        .split(' ')
        .map(|x| x.parse::<u64>().unwrap())
        .collect();

    let distances_part_1 = string_to_vec_u64_ignore_prefix("Distance:", &input[1]);
    let distances_line: Vec<&str> = input[1].split("Distance:").collect();
    let distances_part_2: Vec<u64> = distances_line[1]
        .replace(' ', "")
        .split(' ')
        .map(|x| x.parse::<u64>().unwrap())
        .collect();

    (
        solve(times_part_1, &distances_part_1),
        solve(times_part_2, &distances_part_2),
    )
}

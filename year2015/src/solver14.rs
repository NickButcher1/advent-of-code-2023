use regex::Regex;
use std::cmp::min;

const PART_1_TIME: usize = 2503;

#[allow(clippy::needless_range_loop)]
pub fn solve14(input: &[String]) -> (i128, i128) {
    // Distance travelled by each reindeer after each second.
    let mut reindeer_distance: Vec<Vec<usize>> = vec![];

    let mut furthest_distance = 0;
    let re = Regex::new(
        r"^(\w+) can fly (\d+) km/s for (\d+) seconds, but then must rest for (\d+) seconds.$",
    )
    .unwrap();

    for line in input {
        let captures = re.captures(line).unwrap();
        let speed = captures.get(2).unwrap().as_str().parse::<usize>().unwrap();
        let fly_time = captures.get(3).unwrap().as_str().parse::<usize>().unwrap();
        let rest_time = captures.get(4).unwrap().as_str().parse::<usize>().unwrap();
        let cycle_time = fly_time + rest_time;
        let num_cycles = PART_1_TIME / cycle_time;
        let partial_cycle_time = PART_1_TIME % cycle_time;

        let this_distance =
            num_cycles * fly_time * speed + min(partial_cycle_time, fly_time) * speed;

        if this_distance > furthest_distance {
            furthest_distance = this_distance;
        }

        let mut this_reindeer_distance: Vec<usize> = vec![0];
        let mut distance = 0;
        for _ in 0..num_cycles {
            for _ in 0..fly_time {
                distance += speed;
                this_reindeer_distance.push(distance);
            }
            for _ in 0..rest_time {
                this_reindeer_distance.push(distance);
            }
        }
        for _ in 0..min(partial_cycle_time, fly_time) {
            distance += speed;
            this_reindeer_distance.push(distance);
        }
        while this_reindeer_distance.len() <= PART_1_TIME {
            this_reindeer_distance.push(distance);
        }

        reindeer_distance.push(this_reindeer_distance);
    }

    let mut reindeer_points: Vec<usize> = vec![0; reindeer_distance.len()];
    for second in 1..=PART_1_TIME {
        let mut max_i = 0;
        let mut max_distance = 0;
        for j in 0..reindeer_points.len() {
            if reindeer_distance[j][second] > max_distance {
                max_distance = reindeer_distance[j][second];
                max_i = j;
            }
        }

        reindeer_points[max_i] += 1;
    }

    (
        furthest_distance as i128,
        *reindeer_points.iter().max().unwrap() as i128,
    )
}

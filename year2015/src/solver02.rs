use aoc::input::string_to_vec_u64;
use aoc::solution::{Solution, Solutions};

pub fn solve02(input: &[String]) -> Solutions {
    let answer = input
        .iter()
        .fold((0, 0), |(part_1_area, part_2_len), line| {
            let mut xyz = string_to_vec_u64(line, 'x');
            xyz.sort_unstable();
            (
                // One extra for the smallest of the six sides.
                part_1_area + 3 * xyz[0] * xyz[1] + 2 * xyz[0] * xyz[2] + 2 * xyz[1] * xyz[2],
                // Volume + perimeter of smallest side.
                part_2_len + xyz[0] * xyz[1] * xyz[2] + 2 * xyz[0] + 2 * xyz[1],
            )
        });
    (Solution::U64(answer.0), Solution::U64(answer.1))
}

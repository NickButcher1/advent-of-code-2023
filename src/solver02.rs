const RED_MAX: u32 = 12;
const GREEN_MAX: u32 = 13;
const BLUE_MAX: u32 = 14;

pub fn solve02(input: &[String]) -> (i128, i128) {
    let mut total_part_1 = 0;
    let mut total_part_2 = 0;

    for game_id in 1..=input.len() {
        let split_1: Vec<&str> = input[game_id - 1].split(':').collect();
        let turns: Vec<&str> = split_1[1].split(';').collect();

        let mut game_possible = true;
        let mut red_max = 0;
        let mut green_max = 0;
        let mut blue_max = 0;

        for turn_str in turns {
            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;
            let turn: Vec<&str> = turn_str.split(',').collect();

            for colour in turn {
                let items: Vec<&str> = colour.split(' ').collect();
                let num_colour = items[1].parse::<u32>().unwrap();
                match items[2] {
                    "red" => red = num_colour,
                    "green" => green = num_colour,
                    "blue" => blue = num_colour,
                    _ => unreachable!(),
                }
            }

            if red > RED_MAX || green > GREEN_MAX || blue > BLUE_MAX {
                game_possible = false;
            }

            if red > red_max {
                red_max = red;
            }
            if green > green_max {
                green_max = green;
            }
            if blue > blue_max {
                blue_max = blue;
            }
        }

        if game_possible {
            total_part_1 += game_id;
        }

        total_part_2 += red_max * blue_max * green_max;
    }

    (total_part_1 as i128, total_part_2 as i128)
}

use regex::Regex;

const COST_A: i64 = 3;
const COST_B: i64 = 1;
const PRIZE_OFFSET_PART_TWO: i64 = 10_000_000_000_000;

#[derive(Debug)]
struct Machine {
    a_x: i64,
    a_y: i64,
    b_x: i64,
    b_y: i64,
    prize_x: i64,
    prize_y: i64,
}

fn parse_input(input: &[String], offset: i64) -> Vec<Machine> {
    let re_a = Regex::new(r"^Button A: X\+(\d+), Y\+(\d+)$").unwrap();
    let re_b = Regex::new(r"^Button B: X\+(\d+), Y\+(\d+)$").unwrap();
    let re_prize = Regex::new(r"^Prize: X=(\d+), Y=(\d+)$").unwrap();

    let mut machines = vec![];

    for i in (0..input.len()).step_by(4) {
        let captures_a = re_a.captures(&input[i]).unwrap();
        let captures_b = re_b.captures(&input[i + 1]).unwrap();
        let captures_prize = re_prize.captures(&input[i + 2]).unwrap();
        machines.push(Machine {
            a_x: captures_a.get(1).unwrap().as_str().parse::<i64>().unwrap(),
            a_y: captures_a.get(2).unwrap().as_str().parse::<i64>().unwrap(),
            b_x: captures_b.get(1).unwrap().as_str().parse::<i64>().unwrap(),
            b_y: captures_b.get(2).unwrap().as_str().parse::<i64>().unwrap(),
            prize_x: captures_prize
                .get(1)
                .unwrap()
                .as_str()
                .parse::<i64>()
                .unwrap()
                + offset,
            prize_y: captures_prize
                .get(2)
                .unwrap()
                .as_str()
                .parse::<i64>()
                .unwrap()
                + offset,
        });
    }
    machines
}

// Rearrange and solve these two equations to find a and b.
// PX = a * AX + b * BX
// PY = a * AY + b * BY
// Then plug the values of a and b in, and check if it fits.
pub fn solve(input: &[String], offset: i64) -> i64 {
    parse_input(input, offset)
        .iter()
        .filter_map(|m| {
            let a =
                ((m.prize_x * m.b_y) - (m.prize_y * m.b_x)) / ((m.a_x * m.b_y) - (m.a_y * m.b_x));
            let b =
                ((m.prize_x * m.a_y) - (m.prize_y * m.a_x)) / ((m.b_x * m.a_y) - (m.b_y * m.a_x));

            if ((a * m.a_x) + (b * m.b_x)) == m.prize_x && ((a * m.a_y) + (b * m.b_y)) == m.prize_y
            {
                Some(a * COST_A + b * COST_B)
            } else {
                None
            }
        })
        .sum::<i64>()
}

pub fn solve13(input: &[String]) -> (i128, i128) {
    (
        solve(input, 0) as i128,
        solve(input, PRIZE_OFFSET_PART_TWO) as i128,
    )
}

use regex::Regex;

const COST_A: i64 = 3;
const COST_B: i64 = 1;
const PRIZE_OFFSET_PART_TWO: i64 = 10000000000000;

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

pub fn solve(input: &[String], offset: i64) -> i64 {
    let machines = parse_input(input, offset);

    let mut solution = 0;

    for machine in machines {
        println!("{machine:?}");
        let mut cheapest_cost = i64::MAX;

        for a in 0..=100 {
            for b in 0..=100 {
                let x = machine.a_x * a + machine.b_x * b;
                let y = machine.a_y * a + machine.b_y * b;
                if x == machine.prize_x && y == machine.prize_y {
                    let cost = a * COST_A + b * COST_B;
                    println!("    a {a}    b {b}    cost {cost}");
                    if cost < cheapest_cost {
                        cheapest_cost = cost;
                    }
                }
            }
        }

        if cheapest_cost != i64::MAX {
            solution += cheapest_cost;
        }
    }

    solution
}

pub fn solve13(input: &[String]) -> (i128, i128) {
    (
        solve(input, 0) as i128,
        solve(input, PRIZE_OFFSET_PART_TWO) as i128,
    )
}

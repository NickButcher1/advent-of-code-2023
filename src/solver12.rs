const OPERATIONAL_CHAR: char = '.';
const DAMAGED_CHAR: char = '#';
const UNKNOWN_CHAR: char = '?';

const OPERATIONAL: u8 = 0;
const DAMAGED: u8 = 1;
const UNKNOWN: u8 = 2;
const UNWANTED: u8 = 3;

pub fn solve12(input: Vec<String>) -> (i128, i128) {
    let mut sum_arrangements = 0;

    for line in &input {
        let line_ints: Vec<u8> = line.chars().map(|c| {
            match c {
                OPERATIONAL_CHAR => OPERATIONAL,
                DAMAGED_CHAR => DAMAGED,
                UNKNOWN_CHAR => UNKNOWN,
                _ => UNWANTED,
            }
        })
            .filter(|i| *i != UNWANTED)
            .collect();

        let split_1: Vec<&str> = line.split(' ').collect();
        let counts: Vec<u8> = split_1[1].split(',')
            .map(|x| x.parse::<u8>().unwrap())
            .collect();

        let mut unknown_positions: Vec<u8> = vec![];

        for i in 0..line_ints.len() {
            if line_ints[i] == UNKNOWN {
                unknown_positions.push(i as u8);
            }
        }

        let mut arrangements = 0;
        let num_ways_to_try = u32::pow(2, unknown_positions.len() as u32);
        println!("{:?}  {:?}  {:?}  {}    {}", line_ints, counts, unknown_positions, num_ways_to_try, arrangements);
        for way_int in 0..num_ways_to_try {
            let mut copy_ints = line_ints.clone().to_owned();

            // Flip the different bits.
            for bit in 0..unknown_positions.len() {
                if is_bit_set(way_int, bit) {
                    copy_ints[unknown_positions[bit] as usize] = DAMAGED;
                } else {
                    copy_ints[unknown_positions[bit] as usize] = OPERATIONAL;
                }
            }

            let mut copy_counts: Vec<u8> = vec![];
            let mut damaged_len = 0;
            for i in 0..copy_ints.len() {
                if copy_ints[i] == DAMAGED {
                    damaged_len += 1;
                    if i == (copy_ints.len() - 1) || copy_ints[i+1] != DAMAGED {
                        copy_counts.push(damaged_len);
                        damaged_len = 0;
                    }
                }
            }

            if copy_counts == counts {
                arrangements += 1;
            }
        }
        println!("{:?}  {:?}  {:?}  {}    {}", line_ints, counts, unknown_positions, num_ways_to_try, arrangements);
        sum_arrangements += arrangements;
    }

    (sum_arrangements, 0)
}

fn is_bit_set(input: u32, bit_position: usize) -> bool {
    // Create a mask with only the bit at the specified position set to 1
    let mask = 1 << bit_position;

    // Use bitwise AND to check if the bit is set
    (input & mask) != 0
}
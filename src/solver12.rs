use std::collections::HashMap;

const OPERATIONAL_CHAR: char = '.';
const DAMAGED_CHAR: char = '#';
const UNKNOWN_CHAR: char = '?';

const OPERATIONAL: u8 = 0;
const DAMAGED: u8 = 1;
const UNKNOWN: u8 = 2;
const UNWANTED: u8 = 3;

pub fn solve12(input: Vec<String>) -> (i128, i128) {
    (solve_part(input.clone(), true), solve_part(input, false))
}
pub fn solve_part(input: Vec<String>, is_part_one: bool) -> i128 {
    let mut sum_arrangements: i128 = 0;

    for line in &input {
        let mut line_ints: Vec<u8> = line
            .chars()
            .map(|c| match c {
                OPERATIONAL_CHAR => OPERATIONAL,
                DAMAGED_CHAR => DAMAGED,
                UNKNOWN_CHAR => UNKNOWN,
                _ => UNWANTED,
            })
            .filter(|i| *i != UNWANTED)
            .collect();

        if !is_part_one {
            let copy_line_ints = line_ints.clone().to_owned();
            for _i in 0..4 {
                line_ints.append(&mut vec![UNKNOWN]);
                line_ints.append(&mut copy_line_ints.clone().to_owned());
            }
        }

        let split_1: Vec<&str> = line.split(' ').collect();
        let mut counts: Vec<u8> = split_1[1]
            .split(',')
            .map(|x| x.parse::<u8>().unwrap())
            .collect();

        if !is_part_one {
            let copy_counts = counts.clone().to_owned();
            for _i in 0..4 {
                counts.append(&mut copy_counts.clone().to_owned());
            }
        }

        let mut cache: HashMap<(Vec<u8>, Vec<u8>), u64> = HashMap::new();
        let arrangements = solve_subset(&line_ints, &counts, &mut cache);

        sum_arrangements += arrangements as i128;
    }
    sum_arrangements
}

fn solve_subset(
    line_ints: &Vec<u8>,
    counts: &Vec<u8>,
    cache: &mut HashMap<(Vec<u8>, Vec<u8>), u64>,
) -> u64 {
    return if line_ints.is_empty() {
        0
    } else if line_ints[line_ints.len() - 1] == OPERATIONAL {
        // Chop trailing '.'
        solve_subset(
            line_ints[..line_ints.len() - 1].to_vec().as_ref(),
            counts,
            cache,
        )
    } else if line_ints[0] == OPERATIONAL {
        // Chop leading '.'
        solve_subset(line_ints[1..].to_vec().as_ref(), counts, cache)
    } else {
        if cache.contains_key(&(line_ints.clone(), counts.clone())) {
            return *cache.get(&(line_ints.clone(), counts.clone())).unwrap();
        }

        // Must start with either '?' or '#'. Find all possible positions for first block.
        let mut min_for_block: Vec<u8> = vec![];
        if line_ints[0] == DAMAGED {
            min_for_block.push(0);
        } else {
            let mut pos = 0;
            for count in counts {
                while !can_block_fit_in_pos_forward(line_ints, pos, *count) {
                    pos += 1;
                }
                min_for_block.push(pos);
                pos += *count + 1;
            }
        }

        let mut max_for_block: Vec<u8> = vec![];
        if line_ints[0] == DAMAGED {
            max_for_block.push(counts[0] - 1);
        } else {
            let mut pos = line_ints.len() as u8 - 1;
            for i in (0..counts.len()).rev() {
                while !can_block_fit_in_pos_backward(line_ints, pos, counts[i]) {
                    pos -= 1;
                }
                max_for_block.insert(0, pos);
                pos -= counts[i] + 1;
            }
        }

        // Now place all possible positions of first block.
        let ways = max_for_block[0] - min_for_block[0] - counts[0] + 2;

        let mut sum: u64 = 0;
        for i in 0..ways {
            let offset = min_for_block[0] as usize + i as usize + counts[0] as usize + 1;

            // Check if block can actually fit here.
            let start_pos = min_for_block[0] as usize + i as usize;

            let mut bad_ops = false;
            if start_pos != 0 {
                for i in 0..start_pos {
                    if line_ints[i] == DAMAGED {
                        bad_ops = true;
                        break;
                    }
                }
            }
            if bad_ops {
                continue;
            }

            if start_pos != 0 && line_ints[start_pos - 1] == DAMAGED {
                continue;
            }

            let end_pos = min_for_block[0] as usize + i as usize + counts[0] as usize - 1;
            if end_pos != (line_ints.len() - 1) && line_ints[end_pos + 1] == DAMAGED {
                continue;
            }
            for x in start_pos..=end_pos {
                if line_ints[x] == OPERATIONAL {
                    bad_ops = true;
                    break;
                }
            }
            if bad_ops {
                continue;
            }

            if counts.len() > 1 {
                let x = solve_subset(
                    line_ints[offset..].to_vec().as_ref(),
                    counts[1..].to_vec().as_ref(),
                    cache,
                );
                sum += x;
            } else {
                let x = 1;
                let temp_vec: Vec<u8> = line_ints[(offset - 1)..].to_vec();
                let num_damaged_leftover = temp_vec.into_iter().filter(|i| *i == DAMAGED).count();
                if num_damaged_leftover == 0 {
                    sum += x;
                }
            }
        }

        cache.insert((line_ints.clone(), counts.clone()), sum);
        sum
    };
}

fn can_block_fit_in_pos_forward(line_ints: &[u8], pos: u8, block_len: u8) -> bool {
    for i in 0..block_len {
        if line_ints[pos as usize + i as usize] == OPERATIONAL {
            return false;
        }
    }
    let next_id = pos as usize + block_len as usize;
    next_id == line_ints.len() || line_ints[next_id] != DAMAGED
}

fn can_block_fit_in_pos_backward(line_ints: &[u8], pos: u8, block_len: u8) -> bool {
    for i in 0..block_len {
        if line_ints[pos as usize - i as usize] == OPERATIONAL {
            return false;
        }
    }
    let next_id = pos as i32 - block_len as i32;
    next_id == -1 || line_ints[next_id as usize] != DAMAGED
}

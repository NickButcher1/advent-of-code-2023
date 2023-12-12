use std::collections::HashMap;

const OPERATIONAL: usize = 0;
const DAMAGED: usize = 1;
const UNKNOWN: usize = 2;
const UNWANTED: usize = 3;

pub fn solve12(input: Vec<String>) -> (i128, i128) {
    (solve_part(input.clone(), false), solve_part(input, true))
}
pub fn solve_part(input: Vec<String>, is_part_two: bool) -> i128 {
    let mut sum_arrangements: i128 = 0;

    for line in &input {
        let mut line_ints: Vec<usize> = line
            .chars()
            .map(|c| match c {
                '.' => OPERATIONAL,
                '#' => DAMAGED,
                '?' => UNKNOWN,
                _ => UNWANTED,
            })
            .filter(|i| *i != UNWANTED)
            .collect();

        let split_1: Vec<&str> = line.split(' ').collect();
        let mut counts: Vec<usize> = split_1[1]
            .split(',')
            .map(|x| x.parse::<usize>().unwrap())
            .collect();

        if is_part_two {
            let copy_line_ints = line_ints.clone();
            for _ in 0..4 {
                line_ints.push(UNKNOWN);
                line_ints.append(&mut copy_line_ints.clone());
            }

            let copy_counts = counts.clone();
            for _ in 0..4 {
                counts.append(&mut copy_counts.clone());
            }
        }

        let mut cache: HashMap<(Vec<usize>, Vec<usize>), u64> = HashMap::new();
        let arrangements = solve_subset(&line_ints, &counts, &mut cache);

        sum_arrangements += arrangements as i128;
    }
    sum_arrangements
}

fn solve_subset(
    line_ints: &Vec<usize>,
    counts: &Vec<usize>,
    cache: &mut HashMap<(Vec<usize>, Vec<usize>), u64>,
) -> u64 {
    return if line_ints.is_empty() {
        0
    } else if line_ints[0] == OPERATIONAL {
        // Chop leading '.'
        solve_subset(line_ints[1..].to_vec().as_ref(), counts, cache)
    } else {
        if cache.contains_key(&(line_ints.clone(), counts.clone())) {
            return *cache.get(&(line_ints.clone(), counts.clone())).unwrap();
        }

        // Find all possible positions for the first block.
        let mut min_for_block: usize = 0;
        if line_ints[0] != DAMAGED {
            let mut pos: usize = 0;
            while !can_block_fit_in_pos_forward(line_ints, pos, counts[0]) {
                pos += 1;
            }
            min_for_block = pos;
        }

        let mut max_for_block: usize = 0;
        if line_ints[0] == DAMAGED {
            max_for_block = counts[0] - 1;
        } else {
            let mut pos = line_ints.len() - 1;
            for i in (0..counts.len()).rev() {
                while !can_block_fit_in_pos_backward(line_ints, pos, counts[i]) {
                    pos -= 1;
                }
                max_for_block = pos;
                pos -= counts[i] + 1;
            }
        }

        if max_for_block < min_for_block {
            return 0;
        }

        // Try all possible positions of the first block.
        let mut sum: u64 = 0;
        // The code from here onwards is terrible and needs a complete rewrite to simplify it!
        for i in 0..(max_for_block - min_for_block - counts[0] + 2) {
            let offset = min_for_block + i + counts[0] + 1;

            // Check if block can actually fit here.
            let start_pos = min_for_block + i;

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

            let end_pos = min_for_block + i + counts[0] - 1;
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
                let temp_vec: Vec<usize> = line_ints[(offset - 1)..].to_vec();
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

fn can_block_fit_in_pos_forward(line_ints: &[usize], pos: usize, block_len: usize) -> bool {
    for i in 0..block_len {
        if line_ints[pos + i] == OPERATIONAL {
            return false;
        }
    }
    let next_id = pos + block_len;
    next_id == line_ints.len() || line_ints[next_id] != DAMAGED
}

fn can_block_fit_in_pos_backward(line_ints: &[usize], pos: usize, block_len: usize) -> bool {
    for i in 0..block_len {
        if line_ints[pos - i] == OPERATIONAL {
            return false;
        }
    }
    let next_id = pos as i32 - block_len as i32;
    next_id == -1 || line_ints[next_id as usize] != DAMAGED
}

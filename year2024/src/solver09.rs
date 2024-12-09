const EMPTY_BLOCK: usize = usize::MAX;

fn add_file(output: &mut Vec<usize>, num_bytes: usize, id: usize) {
    for _ in 0..num_bytes {
        output.push(id);
    }
}

fn add_free_space(output: &mut Vec<usize>, num_bytes: usize) {
    for _ in 0..num_bytes {
        output.push(EMPTY_BLOCK);
    }
}

fn parse_input(input: &[String]) -> (Vec<(usize, usize, usize)>, Vec<usize>) {
    let disk_map = input[0]
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect::<Vec<usize>>();

    // Files in a list of (ID, len, start_index) tuples.
    disk_map.chunks(2).enumerate().fold(
        (vec![], vec![]),
        |(mut files, mut disk_layout), (id, chunk)| {
            match chunk {
                [a, b] => {
                    files.push((id, *a, disk_layout.len()));
                    add_file(&mut disk_layout, *a, id);
                    add_free_space(&mut disk_layout, *b);
                }
                [a] => {
                    files.push((id, *a, disk_layout.len()));
                    add_file(&mut disk_layout, *a, id);
                }
                _ => unreachable!(),
            }
            (files, disk_layout)
        },
    )
}

fn calculate_checksum(disk_layout: &Vec<usize>) -> i64 {
    disk_layout
        .iter()
        .enumerate()
        .filter(|&(_, &value)| value != EMPTY_BLOCK)
        .map(|(index, &value)| index as i64 * value as i64)
        .sum()
}

fn solve_part_one(disk_layout: &mut Vec<usize>) -> i64 {
    let mut l_index = 0;
    let mut r_index = disk_layout.len() - 1;
    while r_index > l_index {
        if disk_layout[l_index] != EMPTY_BLOCK {
            l_index += 1;
        } else if disk_layout[r_index] == EMPTY_BLOCK {
            r_index -= 1;
        } else {
            // Move from right to left.
            disk_layout[l_index] = disk_layout[r_index];
            disk_layout[r_index] = EMPTY_BLOCK;
            r_index -= 1;
        }
    }

    calculate_checksum(&disk_layout)
}

fn solve_part_two(disk_layout: &mut Vec<usize>, files: &Vec<(usize, usize, usize)>) -> i64 {
    for (id, len, start_index) in files.into_iter().rev() {
        let mut first_free_slot_index = 0;
        let mut is_gap = false;

        'outer_loop: while first_free_slot_index < *start_index {
            let mut gap_len = 0;
            for i in 0..*len {
                if disk_layout[first_free_slot_index + i] == EMPTY_BLOCK {
                    gap_len += 1;
                    if gap_len == *len {
                        is_gap = true;
                        break 'outer_loop;
                    }
                } else {
                    gap_len = 0;
                }
            }
            first_free_slot_index += 1;
        }

        // Move file.
        if is_gap {
            for i in 0..*len {
                disk_layout[(first_free_slot_index + i) as usize] = *id;
                disk_layout[(start_index + i) as usize] = EMPTY_BLOCK;
            }
        }
    }

    calculate_checksum(&disk_layout)
}

pub fn solve09(input: &[String]) -> (i128, i128) {
    let (files, mut disk_layout) = parse_input(input);

    (
        solve_part_one(&mut disk_layout.clone()) as i128,
        solve_part_two(&mut disk_layout, &files) as i128,
    )
}

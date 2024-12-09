fn add_file(output: &mut Vec<i32>, num_bytes: i32, id: i32) {
    for _ in 0..num_bytes {
        output.push(id);
    }
}

fn add_free_space(output: &mut Vec<i32>, num_bytes: i32) {
    for _ in 0..num_bytes {
        output.push(i32::MAX);
    }
}

pub fn solve09(input: &[String]) -> (i128, i128) {
    // Alternatives file / free space.  Each has ID number from zero in original order.
    let disk_map = input[0]
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i32)
        .collect::<Vec<i32>>();

    let mut files: Vec<(i32, i32, i32)> = vec![];

    let mut disk_layout = vec![];
    let mut id = 0;
    // TODO just flip a bool.
    for chunk in disk_map.chunks(2) {
        match chunk {
            [a, b] => {
                // ID, len, start_index;
                files.push((id, *a, disk_layout.len() as i32));
                add_file(&mut disk_layout, *a, id);
                add_free_space(&mut disk_layout, *b);
            }
            [a] => {
                files.push((id, *a, disk_layout.len() as i32));
                add_file(&mut disk_layout, *a, id);
            }
            _ => unreachable!(),
        }
        id += 1;
    }
    let original_disk_layout = disk_layout.clone();
    let mut l_index = 0;
    let mut r_index = disk_layout.len() - 1;
    while r_index > l_index {
        if disk_layout[l_index] != i32::MAX {
            l_index += 1;
        } else if disk_layout[r_index] == i32::MAX {
            r_index -= 1;
        } else {
            // Move from right to left.
            disk_layout[l_index] = disk_layout[r_index];
            disk_layout[r_index] = i32::MAX;
            r_index -= 1;
            l_index += 1;
        }
    }

    let mut checksum: i64 = 0;
    l_index = 0;
    loop {
        let this_value = disk_layout[l_index];
        if this_value == i32::MAX {
            break;
        }
        checksum += l_index as i64 * this_value as i64;
        l_index += 1;
    }

    disk_layout = original_disk_layout;
    for (id, len, start_index) in files.into_iter().rev() {
        // Find first free slot with at least len bytes.
        let mut first_free_slot_index = 0;
        let mut is_gap = false;
        'outer_loop: while first_free_slot_index < start_index {
            let mut gap_len = 0;
            for i in 0..len {
                if disk_layout[(first_free_slot_index + i) as usize] == i32::MAX {
                    gap_len += 1;
                    if gap_len == len {
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
            for i in 0..len {
                disk_layout[(first_free_slot_index + i) as usize] = id;
                disk_layout[(start_index + i) as usize] = i32::MAX;
            }
        }
    }

    let mut checksum2: i64 = 0;
    l_index = 0;
    while l_index < disk_layout.len() {
        let this_value = disk_layout[l_index];
        if this_value != i32::MAX {
            checksum2 += l_index as i64 * this_value as i64;
        }
        l_index += 1;
    }

    (checksum as i128, checksum2 as i128)
}

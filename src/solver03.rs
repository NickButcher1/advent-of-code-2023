use std::collections::HashSet;

pub fn solve03(input: Vec<String>) -> (i128, i128) {
    let mut total_part_1 = 0;
    let mut total_part_2 = 0;

    let num_rows = input.len();
    let num_cols = input[0].len();

    // Create a border around the board to avoid having to check edge cases.
    let mut is_symbol = vec![vec![false; num_cols + 2]; num_rows + 2];
    let mut digit = vec![vec![' '; num_cols + 2]; num_rows + 2];
    let mut star_number = vec![vec![0; num_cols + 2]; num_rows + 2];
    let mut next_star = 1;

    for r in 0..num_rows {
        for (c, char) in input[r].chars().enumerate() {
            is_symbol[r + 1][c + 1] = char != '.' && !char.is_ascii_digit();

            if char.is_ascii_digit() {
                digit[r + 1][c + 1] = char;
            } else {
                digit[r + 1][c + 1] = ' ';
            }

            if char == '*' {
                star_number[r + 1][c + 1] = next_star;
                next_star += 1;
            }
        }
    }

    let mut is_adjacent_symbol = vec![vec![false; num_cols + 2]; num_rows + 2];
    let mut is_adjacent_star_numbers = vec![vec![Vec::<u32>::new(); num_cols + 2]; num_rows + 2];

    for r in 1..=num_rows {
        for c in 1..=num_cols {
            for (row, col) in [
                (r, c),
                (r, c - 1),
                (r, c + 1),
                (r - 1, c),
                (r - 1, c - 1),
                (r - 1, c + 1),
                (r + 1, c),
                (r + 1, c - 1),
                (r + 1, c + 1),
            ] {
                if is_symbol[row][col] {
                    is_adjacent_symbol[r][c] = true;
                }
                if star_number[row][col] != 0 {
                    is_adjacent_star_numbers[r][c].push(star_number[row][col]);
                }
            }
        }
    }

    let mut star_list = vec![Vec::<u32>::new(); next_star as usize + 1];

    for r in 0..=num_rows {
        for c in 0..=num_cols {
            let mut matching_star_numbers = Vec::<u32>::new();
            let mut is_allowed = false;

            if digit[r][c] != ' ' && digit[r][c - 1] == ' ' {
                let mut this_number = 0;
                let mut cx = c;

                while digit[r][cx] != ' ' {
                    this_number = (10 * this_number) + digit[r][cx].to_digit(10).unwrap();
                    is_allowed = is_allowed || is_adjacent_symbol[r][cx];
                    matching_star_numbers.append(&mut is_adjacent_star_numbers[r][cx]);
                    cx += 1;
                }

                matching_star_numbers = matching_star_numbers
                    .iter()
                    .cloned()
                    .collect::<HashSet<_>>()
                    .into_iter()
                    .collect();
                for star_number in matching_star_numbers {
                    star_list[star_number as usize].push(this_number);
                }
                if is_allowed {
                    total_part_1 += this_number;
                }
            }
        }
    }

    for star_line in star_list {
        if star_line.len() == 2 {
            let this_number = star_line[0] * star_line[1];
            total_part_2 += this_number;
        }
    }

    (total_part_1 as i128, total_part_2 as i128)
}

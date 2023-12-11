use std::cmp;
use std::collections::HashMap;

const EXPANSION_FACTOR_PART_1: i64 = 2;
const EXPANSION_FACTOR_PART_2: i64 = 1000000;
type Board = Vec<Vec<i32>>;
type Cell = (usize, usize);

pub fn solve11(input: Vec<String>) -> (i128, i128) {
    // Expand rows and replace '#' with numbers in one pass.
    let num_rows_unexpanded = input.len();
    let num_cols_unexpanded = input[0].len();
    let mut next_galaxy_id = 0;
    let mut is_row_id_empty: HashMap<usize, bool> = HashMap::new();
    let mut is_col_id_empty: HashMap<usize, bool> = HashMap::new();

    let mut board_a: Board = vec![];
    for r in 0..num_rows_unexpanded {
        let row = &input[r];
        let mut galaxy_this_row = false;
        let mut this_row: Vec<i32> = vec![];

        for c in row.chars() {
            let galaxy_id = if c == '#' {
                galaxy_this_row = true;
                next_galaxy_id += 1;
                next_galaxy_id
            } else {
                0
            };
            this_row.push(galaxy_id);
        }
        if !galaxy_this_row {
            // col because will flip.
            is_col_id_empty.insert(r, true);
        } else {
            is_col_id_empty.insert(r, false);
        }
        board_a.push(this_row);
    }

    let num_rows = board_a.len();

    // Flip the board and expand the columns.
    let mut board_b: Board = vec![];
    for c in 0..num_cols_unexpanded {
        let mut galaxy_this_row = false;
        let mut this_row: Vec<i32> = vec![];
        for r in 0..num_rows {
            this_row.push(board_a[r][c]);
            if board_a[r][c] != 0 {
                galaxy_this_row = true;
            }
        }
        if !galaxy_this_row {
            is_row_id_empty.insert(c, true);
        } else {
            is_row_id_empty.insert(c, false);
        }
        board_b.push(this_row);
    }

    let num_rows = board_b.len();
    let num_cols = board_b[0].len();

    let mut galaxy_coordinates: HashMap<i32, Cell> = HashMap::new();
    for r in 0..num_rows {
        for c in 0..num_cols {
            let galaxy_id = board_b[r][c];
            if galaxy_id != 0 {
                galaxy_coordinates.insert(galaxy_id, (r, c));
            }
        }
    }
    let num_galaxies = next_galaxy_id;
    let mut part_1_solution = 0;
    let mut part_2_solution = 0;
    for g1_id in 1..(num_galaxies + 1) {
        for g2_id in (g1_id + 1)..(num_galaxies + 1) {
            let max_row = cmp::max(
                galaxy_coordinates[&g2_id].0 as i32,
                galaxy_coordinates[&g1_id].0 as i32,
            );
            let min_row = cmp::min(
                galaxy_coordinates[&g2_id].0 as i32,
                galaxy_coordinates[&g1_id].0 as i32,
            );
            let max_col = cmp::max(
                galaxy_coordinates[&g2_id].1 as i32,
                galaxy_coordinates[&g1_id].1 as i32,
            );
            let min_col = cmp::min(
                galaxy_coordinates[&g2_id].1 as i32,
                galaxy_coordinates[&g1_id].1 as i32,
            );

            let row_diff = max_row - min_row;
            let col_diff = max_col - min_col;
            let path_len: i64 = (row_diff + col_diff) as i64;
            part_1_solution += path_len;
            part_2_solution += path_len;

            // Add empty rows/cols.
            if row_diff > 1 {
                for r in (min_row + 1)..max_row {
                    if is_row_id_empty[&(r as usize)] {
                        part_1_solution += EXPANSION_FACTOR_PART_1 - 1;
                        part_2_solution += EXPANSION_FACTOR_PART_2 - 1;
                    }
                }
            }
            if col_diff > 1 {
                for c in (min_col + 1)..max_col {
                    if is_col_id_empty[&(c as usize)] {
                        part_1_solution += EXPANSION_FACTOR_PART_1 - 1;
                        part_2_solution += EXPANSION_FACTOR_PART_2 - 1;
                    }
                }
            }
        }
    }

    (part_1_solution as i128, part_2_solution as i128)
}

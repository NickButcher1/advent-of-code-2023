use std::cmp;
use std::collections::HashMap;

// The only difference between part 1 and part 2 is how many times empty rows and columns are expanded.
const EXPANSION_FACTOR_PART_1: i64 = 2;
const EXPANSION_FACTOR_PART_2: i64 = 1_000_000;

type Board = Vec<Vec<i32>>;
type Cell = (i32, i32);

pub fn solve11(input: &[String]) -> (i128, i128) {
    // Track which rows and columns are empty. Don't actually expand the board.
    let mut is_row_id_empty: HashMap<usize, bool> = HashMap::new();
    let mut is_col_id_empty: HashMap<usize, bool> = HashMap::new();

    // First pass:
    // - Replace '.' with zero to indicate no galaxy.
    // - Replace '#' with unique galaxy IDs.
    // - Calculate is_col_id_empty (yes, column not row because we're going to flip the board in part 2.
    let mut num_galaxies = 0;
    let mut board_a: Board = vec![];

    for (r, row) in input.iter().enumerate() {
        let mut is_empty_row = true;
        let mut this_row: Vec<i32> = vec![];

        for c in row.chars() {
            let galaxy_id = if c == '#' {
                is_empty_row = false;
                num_galaxies += 1;
                num_galaxies
            } else {
                0
            };
            this_row.push(galaxy_id);
        }
        is_col_id_empty.insert(r, is_empty_row);
        board_a.push(this_row);
    }

    // Second pass:
    // - Flip the board on the diagonal axis.
    // - Calculate is_row_id_empty.
    // - Build a lookup of galaxy ID to its coordinates.
    let mut galaxy_coordinates: HashMap<i32, Cell> = HashMap::new();

    for c in 0..input[0].len() {
        let mut is_empty_row = true;

        for (r, _) in board_a.iter().enumerate() {
            if board_a[r][c] != 0 {
                galaxy_coordinates.insert(board_a[r][c], (c as i32, r as i32));
                is_empty_row = false;
            }
        }
        is_row_id_empty.insert(c, is_empty_row);
    }

    // Calculate the shortest path from every galaxy to every other galaxy with a greater ID.
    // The sum of these is the solution.
    let mut part_1_solution = 0;
    let mut part_2_solution = 0;

    for g1_id in 1..=num_galaxies {
        let g1_coordinates = galaxy_coordinates[&g1_id];
        for g2_id in (g1_id + 1)..=num_galaxies {
            let g2_coordinates = galaxy_coordinates[&g2_id];

            let max_row = cmp::max(g2_coordinates.0, g1_coordinates.0);
            let min_row = cmp::min(g2_coordinates.0, g1_coordinates.0);
            let max_col = cmp::max(g2_coordinates.1, g1_coordinates.1);
            let min_col = cmp::min(g2_coordinates.1, g1_coordinates.1);

            let path_len = i64::from(max_row - min_row + max_col - min_col);
            part_1_solution += path_len;
            part_2_solution += path_len;

            // So far we've got the path lengths based on the unexpanded board.
            // Every path length increases by one for every empty row or column that it crosses.
            for r in (min_row + 1)..max_row {
                if is_row_id_empty[&(r as usize)] {
                    part_1_solution += EXPANSION_FACTOR_PART_1 - 1;
                    part_2_solution += EXPANSION_FACTOR_PART_2 - 1;
                }
            }

            for c in (min_col + 1)..max_col {
                if is_col_id_empty[&(c as usize)] {
                    part_1_solution += EXPANSION_FACTOR_PART_1 - 1;
                    part_2_solution += EXPANSION_FACTOR_PART_2 - 1;
                }
            }
        }
    }

    (i128::from(part_1_solution), i128::from(part_2_solution))
}

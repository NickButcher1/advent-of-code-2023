use std::collections::HashMap;
use string_builder::Builder;

type Board = Vec<Vec<i32>>;
type Cell = (usize, usize);

fn print_board(board: &Board, num_rows: usize, num_cols: usize) {
    let mut builder = Builder::default();

    for r in 0..num_rows {
        builder.append('\n');
        for c in 0..num_cols {
            let ch = if board[r][c] == 0 {
                '.'
            } else {
                // char::from_digit(board[r][c] as u32, 10).unwrap()
                '#'
            };
            builder.append(ch);
        }
    }

    println!("\n\nR {}, C {}{}", num_rows, num_cols, builder.string().unwrap());
}

pub fn solve11(input: Vec<String>) -> (i128, i128) {
    // Expand rows and replace '#' with numbers in one pass.
    let num_rows_unexpanded = input.len();
    let num_cols_unexpanded = input[0].len();
    let mut next_galaxy_id = 0;

    let mut board_a: Board = vec![];
    for r in 0..num_rows_unexpanded {
        let row = &input[r];
        // println!("{}", row);
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
        // println!("DBG THIS ROW {:?}", this_row);
        if !galaxy_this_row {
            board_a.push(this_row.clone())
        }
        board_a.push(this_row);
    }

    let num_rows = board_a.len();
    print_board(&board_a, num_rows, num_cols_unexpanded);

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
            board_b.push(this_row.clone())
        }
        board_b.push(this_row);
    }

    let num_rows = board_b.len();
    let num_cols = board_b[0].len();
    print_board(&board_b, num_rows, num_cols);

    let mut galaxy_coordinates: HashMap<i32, Cell> = HashMap::new();
    for r in 0..num_rows {
        for c in 0..num_cols {
            let galaxy_id = board_b[r][c];
            if galaxy_id != 0 {
                println!("INSERT {} {:?}", galaxy_id, (r,c));
                galaxy_coordinates.insert(galaxy_id, (r,c));
            }
        }
    }
    let num_galaxies = next_galaxy_id;
    println!("COORDINATES: {:?}", galaxy_coordinates);
    let mut part_1_solution = 0;
    for g1_id in 1..(num_galaxies+1) {
        for g2_id in (g1_id + 1)..(num_galaxies+1) {
            let row_diff = (galaxy_coordinates[&g2_id].0 as i32 - galaxy_coordinates[&g1_id].0 as i32).abs();
            let col_diff = (galaxy_coordinates[&g2_id].1 as i32 - galaxy_coordinates[&g1_id].1 as i32).abs();
            let path_len = row_diff + col_diff;
            part_1_solution += path_len;
            println!("TEST: {} {}  {} {} PATH LEN {}", g1_id, g2_id, row_diff, col_diff, path_len);
        }
    }

    (part_1_solution as i128, 0)
}

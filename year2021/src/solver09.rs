use aoc::board::Board;
use aoc::solution::{Solution, Solutions};
use itertools::iproduct;

pub fn solve09(input: &[String]) -> Solutions {
    let mut board = Board::from_input(input);
    board.add_border('A');

    let mut sum_risk_levels = 0;
    for (r, c) in iproduct!(1..(board.num_rows - 1), 1..(board.num_cols - 1)) {
        if board.cells[r][c] < board.cells[r - 1][c]
            && board.cells[r][c] < board.cells[r + 1][c]
            && board.cells[r][c] < board.cells[r][c - 1]
            && board.cells[r][c] < board.cells[r][c + 1]
        {
            sum_risk_levels += 1 + board.cells[r][c].to_digit(10).unwrap();
        }
    }

    // Separate the board into either "in a basin" or "between basins".
    board.replace('A', '#');
    board.replace('9', '#');
    board.replace('0', '.');
    board.replace('1', '.');
    board.replace('2', '.');
    board.replace('3', '.');
    board.replace('4', '.');
    board.replace('5', '.');
    board.replace('6', '.');
    board.replace('7', '.');
    board.replace('8', '.');

    // Loop over every board cell. Each time an empty cell is found, flood fill the basin it is in.
    let mut basin_sizes = vec![];
    let mut next_basin_id = 'a';
    let mut queue: Vec<(usize, usize)> = vec![];

    for (r, c) in iproduct!(1..(board.num_rows - 1), 1..(board.num_cols - 1)) {
        if board.cells[r][c] == '.' {
            // Cycle through letters as basin IDs. We don't actually need a unique ID for each
            // basin - just a non-empty ID would be fine.
            if next_basin_id == 'z' {
                next_basin_id = 'a';
            }
            let mut basin_size = 0;
            queue.push((r, c));
            while let Some((test_r, test_c)) = queue.pop() {
                [(-1, 0), (0, 1), (1, 0), (0, -1)]
                    .iter()
                    .for_each(|(dr, dc)| {
                        let new_r = (test_r as i32 + dr) as usize;
                        let new_c = (test_c as i32 + dc) as usize;
                        if board.cells[new_r][new_c] == '.' {
                            board.cells[new_r][new_c] = next_basin_id;
                            basin_size += 1;
                            queue.push((new_r, new_c));
                        }
                    });
            }
            basin_sizes.push(basin_size);
            next_basin_id = ((next_basin_id as u8) + 1) as char;
        }
    }
    basin_sizes.sort();
    let solution_two = basin_sizes[basin_sizes.len() - 1]
        * basin_sizes[basin_sizes.len() - 2]
        * basin_sizes[basin_sizes.len() - 3];

    (Solution::U32(sum_risk_levels), Solution::I32(solution_two))
}

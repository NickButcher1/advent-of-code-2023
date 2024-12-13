use aoc::int_board::IntBoard;
use itertools::iproduct;

fn expand(
    flash_count: &mut i32,
    queue: &mut Vec<(usize, usize)>,
    board: &mut IntBoard,
    r: usize,
    c: usize,
) {
    board.cells[r][c] += 1;
    if board.cells[r][c] == 10 {
        *flash_count += 1;
        // Increment the neighbours.
        [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ]
        .iter()
        .for_each(|(rd, cd)| {
            let new_r = r as i32 + rd;
            let new_c = c as i32 + cd;
            if new_r >= 0
                && new_c >= 0
                && new_r < (board.num_rows as i32)
                && new_c < (board.num_rows as i32)
            {
                queue.push((new_r as usize, new_c as usize));
            }
        });
    }
}

pub fn solve11(input: &[String]) -> (i128, i128) {
    let mut board = IntBoard::from_input(input);
    let mut flash_count: i32 = 0;
    let mut solution_one = 0;

    for step in 1..=i32::MAX {
        let mut new_board = board.clone();
        new_board.increment_all();

        let mut queue = vec![];

        for (c, r) in iproduct!(0..board.num_cols, 0..board.num_rows) {
            if new_board.cells[r][c] == 10 {
                new_board.cells[r][c] -= 1;
                queue.push((r, c));
            }
        }
        while let Some((r, c)) = queue.pop() {
            expand(&mut flash_count, &mut queue, &mut new_board, r, c);
        }

        for (c, r) in iproduct!(0..board.num_cols, 0..board.num_rows) {
            if new_board.cells[r][c] >= 10 {
                new_board.cells[r][c] = 0;
            }
        }

        if step == 100 {
            solution_one = flash_count;
        }

        if new_board.count(0) as usize == board.num_rows * board.num_cols {
            return (solution_one as i128, step as i128);
        }

        board = new_board;
    }

    unreachable!()
}

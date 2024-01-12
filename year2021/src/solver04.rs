use aoc::input::string_to_vec_u64;

#[derive(Debug, PartialEq)]
struct Board {
    rows: Vec<Vec<u64>>,
}

pub fn solve04(input: &[String]) -> (i128, i128) {
    let numbers = string_to_vec_u64(&input[0], ',');

    let mut boards: Vec<Board> = vec![];
    let num_boards = (input.len() - 1) / 6;
    for i in 0..num_boards {
        let mut rows: Vec<Vec<u64>> = vec![];
        for r in 0..5 {
            rows.push(string_to_vec_u64(&input[2 + i * 6 + r], ' '));
        }
        for c in 0..5 {
            rows.push(vec![
                rows[0][c], rows[1][c], rows[2][c], rows[3][c], rows[4][c],
            ]);
        }
        boards.push(Board { rows });
    }

    let mut part_1 = 0;

    for number in numbers {
        let mut winning_number = u64::MAX;

        for board in &mut boards {
            for row in &mut board.rows {
                row.retain(|&x| x != number);
                if row.is_empty() {
                    winning_number = number;
                }
            }
        }

        let mut board_ids_to_remove = vec![];
        for i in 0..boards.len() {
            let mut is_winning_board = false;
            let mut sum_board = 0;
            for row in &boards[i].rows {
                sum_board += row.iter().sum::<u64>();
                if row.is_empty() {
                    is_winning_board = true;
                }
            }

            // All numbers are tracked twice, once for their row and once for their column.
            sum_board /= 2;

            if is_winning_board {
                if part_1 == 0 {
                    part_1 = sum_board * winning_number;
                }
                if boards.len() == 1 {
                    let part_2 = sum_board * winning_number;
                    return (part_1 as i128, part_2 as i128);
                }
                board_ids_to_remove.push(i);
            }
        }

        board_ids_to_remove.reverse();
        for i in board_ids_to_remove {
            boards.remove(i);
        }
    }
    unreachable!();
}

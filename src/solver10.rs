use string_builder::Builder;

#[derive(Clone, PartialEq)]
enum CellType {
    Unknown,
    Loop,
    Outside,
    Inside,
}

#[derive(Clone, Debug, PartialEq)]
enum PipeType {
    Horizontal,
    Vertical,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    Start,
    Empty,
}

fn print_cell_types(
    cell_types: &Vec<Vec<CellType>>,
    num_rows: usize,
    num_cols: usize,
    reason: &str,
) {
    let mut num_inside = 0;
    let mut num_outside = 0;
    let mut num_unknown = 0;
    let mut builder = Builder::default();

    for r in 0..num_rows {
        builder.append('\n');
        for c in 0..num_cols {
            let ch = match &cell_types[r][c] {
                CellType::Unknown => {
                    num_unknown += 1;
                    ' '
                }
                CellType::Loop => '#',
                CellType::Inside => {
                    num_inside += 1;
                    'I'
                }
                CellType::Outside => {
                    num_outside += 1;
                    'O'
                }
            };
            builder.append(ch);
        }
    }

    println!(
        "CELLS: {}\nINSIDE: {}\nOUTSIDE: {}\nUNKNOWN: {}{}",
        reason,
        num_inside,
        num_outside,
        num_unknown,
        builder.string().unwrap()
    );
}

pub fn solve10(input: Vec<String>) -> (i128, i128) {
    let num_rows = input.len();
    let num_cols = input[0].len();

    let mut board: Vec<Vec<PipeType>> = vec![vec![]; num_rows];
    for r in 0..num_rows {
        let chars: Vec<char> = input[r].chars().collect();
        for c in chars {
            let value = match c {
                '-' => PipeType::Horizontal,
                '|' => PipeType::Vertical,
                'F' => PipeType::TopLeft,
                'J' => PipeType::BottomRight,
                'L' => PipeType::BottomLeft,
                '7' => PipeType::TopRight,
                'S' => PipeType::Start,
                '.' => PipeType::Empty,
                _ => panic!("ERROR!"),
            };
            board[r].push(value);
        }
    }

    // Build pair of where cell goes to.
    let mut start_cell = (0, 0);
    let mut goes_to: Vec<Vec<((i32, i32), (i32, i32))>> = vec![vec![]; num_rows];
    for row in 0..num_rows {
        for col in 0..num_cols {
            let r = row as i32;
            let c = col as i32;
            let x = match &board[r as usize][c as usize] {
                PipeType::Horizontal => ((r, c - 1), (r, c + 1)),
                PipeType::Vertical => ((r - 1, c), (r + 1, c)),
                PipeType::TopLeft => ((r, c + 1), (r + 1, c)),
                PipeType::TopRight => ((r, c - 1), (r + 1, c)),
                PipeType::BottomLeft => ((r, c + 1), (r - 1, c)),
                PipeType::BottomRight => ((r, c - 1), (r - 1, c)),
                PipeType::Start => {
                    start_cell = (r, c);
                    let mut start_cell_exit_up = false;
                    let mut start_cell_exit_down = false;
                    let mut start_cell_exit_right = false;
                    let mut start_cell_exit_left = false;

                    let mut start_cell_goes_to: Vec<(i32, i32)> = vec![];
                    if r != 0 {
                        let cell = &board[row - 1][col];
                        if *cell == PipeType::Vertical
                            || *cell == PipeType::TopLeft
                            || *cell == PipeType::TopRight
                        {
                            start_cell_goes_to.push((r - 1, c));
                            start_cell_exit_up = true;
                        }
                    }
                    if r != (num_rows as i32 - 1) {
                        let cell = &board[row + 1][col];
                        if *cell == PipeType::Vertical
                            || *cell == PipeType::BottomLeft
                            || *cell == PipeType::BottomRight
                        {
                            start_cell_goes_to.push((r + 1, c));
                            start_cell_exit_down = true;
                        }
                    }
                    if c != 0 {
                        let cell = &board[row][col - 1];
                        if *cell == PipeType::Horizontal
                            || *cell == PipeType::TopLeft
                            || *cell == PipeType::TopRight
                            || *cell == PipeType::BottomLeft
                            || *cell == PipeType::BottomRight
                        {
                            start_cell_goes_to.push((r, c - 1));
                            start_cell_exit_left = true;
                        }
                    }
                    if c != (num_cols as i32 - 1) {
                        let cell = &board[row][col + 1];
                        if *cell == PipeType::Horizontal
                            || *cell == PipeType::TopLeft
                            || *cell == PipeType::TopRight
                            || *cell == PipeType::BottomLeft
                            || *cell == PipeType::BottomRight
                        {
                            start_cell_goes_to.push((r, c + 1));
                            start_cell_exit_right = true;
                        }
                    }
                    println!(
                        "start_cell_exit_right {}, start_cell_exit_up {}",
                        start_cell_exit_right, start_cell_exit_up
                    );
                    println!(
                        "start_cell_exit_left {}, start_cell_exit_down {}",
                        start_cell_exit_left, start_cell_exit_down
                    );
                    if start_cell_exit_up && start_cell_exit_right {
                        board[r as usize][c as usize] = PipeType::BottomLeft;
                    } else if start_cell_exit_up && start_cell_exit_down {
                        board[r as usize][c as usize] = PipeType::Vertical;
                    } else if start_cell_exit_up && start_cell_exit_left {
                        board[r as usize][c as usize] = PipeType::BottomRight;
                    } else if start_cell_exit_right && start_cell_exit_down {
                        board[r as usize][c as usize] = PipeType::TopLeft;
                    } else if start_cell_exit_right && start_cell_exit_left {
                        board[r as usize][c as usize] = PipeType::Horizontal;
                    } else if start_cell_exit_down && start_cell_exit_left {
                        board[r as usize][c as usize] = PipeType::TopRight;
                    } else {
                        unreachable!();
                    }
                    (start_cell_goes_to[0], start_cell_goes_to[1])
                }
                PipeType::Empty => ((-1, -1), (-1, -1)),
            };
            goes_to[row].push(x);
        }
    }

    let mut path: Vec<(i32, i32)> = vec![];
    path.push(start_cell);
    let mut prev_goes_to = start_cell;
    let mut prev_but_one_goes_to = start_cell;

    loop {
        let goes_to_a = goes_to[prev_goes_to.0 as usize][prev_goes_to.1 as usize].0;
        let goes_to_b = goes_to[prev_goes_to.0 as usize][prev_goes_to.1 as usize].1;
        if path.len() > 2 && (goes_to_a == start_cell || goes_to_b == start_cell) {
            break;
        } else if prev_but_one_goes_to != goes_to_a {
            path.push(goes_to_a);
            prev_but_one_goes_to = prev_goes_to;
            prev_goes_to = goes_to_a;
        } else if prev_but_one_goes_to != goes_to_b {
            path.push(goes_to_b);
            prev_but_one_goes_to = prev_goes_to;
            prev_goes_to = goes_to_b;
        } else {
            unreachable!();
        }
    }

    let part_1 = path.len() / 2;

    // Part 2 - flood fill.

    let mut cell_types: Vec<Vec<CellType>> = vec![vec![CellType::Unknown; num_cols]; num_rows];
    // print_cell_types(&cell_types, num_rows, num_cols, "init to empty");
    for cell in path {
        cell_types[cell.0 as usize][cell.1 as usize] = CellType::Loop;
    }
    // print_cell_types(&cell_types, num_rows, num_cols, "add loop");

    for row in 0..num_rows {
        for col in 0..num_cols {
            if cell_types[row][col] == CellType::Unknown
                && (row == 0 || col == 0 || row == (num_rows - 1) || col == (num_cols - 1))
            {
                cell_types[row][col] = CellType::Outside;
            }
        }
    }
    // print_cell_types(&cell_types, num_rows, num_cols, "set outer to OUTSIDE");
    //
    // while set_outside_if_possible(&mut cell_types, num_rows, num_cols) {
    //     print_cell_types(&cell_types, num_rows, num_cols, "Fill in more OUTSIDE");
    // }

    // Abandon trying to flood fill from outside in. Never going to work with the zero width gaps between pipes.

    // Part 2 - try again. There must be a way to calculate for each empty cell whether it is inside or outside the loop.
    // Something to do with parity
    // - odd number of horizontal bars above or below means cell is inside loop.
    // - odd number of vertical bars to left or right means cell is inside loop.
    // What if odd number one side and even number other side? Is that possible?
    // No, don't think that's possible. Try drawing it, but when you add the loop you always end up adding another vertical.
    // Something else? Also need to match the corner pipes. F7 is logically ||. Ditto LJ. But FJ is not.
    // When looking at horizontal bars, F and 7 both cancel out because two horizontal pipes crossing that column.
    //                                  L     J

    // Remove all non-loop pipes from the board, leaving just the loop and empty cells.
    for row in 0..num_rows {
        for col in 0..num_cols {
            if cell_types[row][col] != CellType::Loop {
                board[row][col] = PipeType::Empty;
            }
        }
    }

    let mut part_2_num_inside = 0;
    for r in 1..(num_rows - 1) {
        for c in 1..(num_cols - 1) {
            if cell_types[r][c] != CellType::Loop
                && is_cell_inside_loop(r, c, &board, num_rows, num_cols)
            {
                part_2_num_inside += 1;
            }
        }
    }

    (part_1 as i128, part_2_num_inside as i128)
}

fn is_cell_inside_loop(
    r: usize,
    c: usize,
    board: &Vec<Vec<PipeType>>,
    num_rows: usize,
    num_cols: usize,
) -> bool {
    let mut count_to_top_v1 = 0;
    let mut count_to_top_v2 = 0;
    let mut count_to_bottom_v1 = 0;
    let mut count_to_bottom_v2 = 0;
    for r2 in 0..num_rows {
        if board[r2][c] == PipeType::Horizontal
            || board[r2][c] == PipeType::TopLeft
            || board[r2][c] == PipeType::BottomLeft
        {
            if r2 < r {
                count_to_top_v1 += 1;
            } else {
                count_to_bottom_v1 += 1;
            }
        }

        if board[r2][c] == PipeType::Horizontal
            || board[r2][c] == PipeType::TopRight
            || board[r2][c] == PipeType::BottomRight
        {
            if r2 < r {
                count_to_top_v2 += 1;
            } else {
                count_to_bottom_v2 += 1;
            }
        }
    }
    if count_to_top_v1 % 2 != count_to_bottom_v1 % 2 {
        panic!("ERROR!");
    }
    if count_to_top_v2 % 2 != count_to_bottom_v2 % 2 {
        panic!("ERROR!");
    }
    if count_to_top_v1 % 2 != count_to_top_v2 % 2 {
        panic!("ERROR!");
    }
    count_to_top_v1 % 2 == 1
}

// Puzzle is symmetrical, so looking left/right must work too. Check just in case I've missed something.
fn is_cell_inside_loop_horizontal(
    r: usize,
    c: usize,
    board: &Vec<Vec<PipeType>>,
    num_rows: usize,
    num_cols: usize,
) -> bool {
    let mut count_to_left_v1 = 0;
    let mut count_to_left_v2 = 0;
    let mut count_to_right_v1 = 0;
    let mut count_to_right_v2 = 0;
    for c2 in 0..num_cols {
        if board[r][c2] == PipeType::Vertical
            || board[r][c2] == PipeType::BottomLeft
            || board[r][c2] == PipeType::BottomRight
        {
            if c2 < c {
                count_to_left_v1 += 1;
            } else {
                count_to_right_v1 += 1;
            }
        }

        if board[r][c2] == PipeType::Vertical
            || board[r][c2] == PipeType::TopLeft
            || board[r][c2] == PipeType::TopRight
        {
            if c2 < c {
                count_to_left_v2 += 1;
            } else {
                count_to_right_v2 += 1;
            }
        }
    }
    if count_to_left_v1 % 2 != count_to_right_v1 % 2 {
        panic!("ERROR!");
    }
    if count_to_left_v2 % 2 != count_to_right_v2 % 2 {
        panic!("ERROR!");
    }
    if count_to_left_v1 % 2 != count_to_left_v2 % 2 {
        panic!("ERROR!");
    }
    count_to_left_v1 % 2 == 1
}

// fn set_outside_if_possible(cell_types: &mut Vec<Vec<CellType>>, num_rows: usize, num_cols: usize) -> bool {
//     let mut added_outside = false;
//     for row in 1..(num_rows-1) {
//         for col in 1..(num_cols-1) {
//             if cell_types[row][col] == CellType::Unknown {
//                 if cell_types[row+1][col] == CellType::Outside || cell_types[row-1][col] == CellType::Outside || cell_types[row][col+1] == CellType::Outside || cell_types[row][col-1] == CellType::Outside {
//                     cell_types[row][col] = CellType::Outside;
//                     added_outside = true;
//                 }
//             }
//         }
//     }
//     added_outside
// }

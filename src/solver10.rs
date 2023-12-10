#[derive(Clone, PartialEq)]
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

                    let mut start_cell_goes_to: Vec<(i32, i32)> = vec![];
                    if r != 0 {
                        let cell = &board[row - 1][col];
                        if *cell == PipeType::Vertical
                            || *cell == PipeType::TopLeft
                            || *cell == PipeType::TopRight
                        {
                            start_cell_goes_to.push((r - 1, c));
                        }
                    }
                    if r != (num_rows as i32 - 1) {
                        let cell = &board[row + 1][col];
                        if *cell == PipeType::Vertical
                            || *cell == PipeType::TopLeft
                            || *cell == PipeType::TopRight
                        {
                            start_cell_goes_to.push((r + 1, c));
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
                        }
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

    (part_1 as i128, 0)
}

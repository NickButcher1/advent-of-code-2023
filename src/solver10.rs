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

type Board = Vec<Vec<PipeType>>;
type Cell = (i32, i32);

fn init_board(input: Vec<String>, num_rows: usize) -> Board {
    let mut board: Board = vec![vec![]; num_rows];
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
    board
}

// This is awful. There must be a better way!
fn build_start_cell_goes_to(
    board: &mut Board,
    num_rows: usize,
    num_cols: usize,
    r: i32,
    c: i32,
) -> (Cell, Cell) {
    let row: usize = r as usize;
    let col: usize = c as usize;
    let mut start_cell_exit_up = false;
    let mut start_cell_exit_down = false;
    let mut start_cell_exit_right = false;
    let mut start_cell_exit_left = false;

    let mut start_cell_goes_to: Vec<Cell> = vec![];
    if r != 0 {
        let cell = &board[row - 1][col];
        if *cell == PipeType::Vertical || *cell == PipeType::TopLeft || *cell == PipeType::TopRight
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
    let start_cell_type = if start_cell_exit_up && start_cell_exit_right {
        PipeType::BottomLeft
    } else if start_cell_exit_up && start_cell_exit_down {
        PipeType::Vertical
    } else if start_cell_exit_up && start_cell_exit_left {
        PipeType::BottomRight
    } else if start_cell_exit_right && start_cell_exit_down {
        PipeType::TopLeft
    } else if start_cell_exit_right && start_cell_exit_left {
        PipeType::Horizontal
    } else if start_cell_exit_down && start_cell_exit_left {
        PipeType::TopRight
    } else {
        unreachable!();
    };
    board[r as usize][c as usize] = start_cell_type;
    (start_cell_goes_to[0], start_cell_goes_to[1])
}

fn build_cell_goes_to(
    board: &mut Board,
    num_rows: usize,
    num_cols: usize,
    start_cell: &mut Cell,
) -> Vec<Vec<(Cell, Cell)>> {
    let mut goes_to: Vec<Vec<(Cell, Cell)>> = vec![vec![]; num_rows];
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
                    *start_cell = (r, c);
                    build_start_cell_goes_to(board, num_rows, num_cols, r, c)
                }
                PipeType::Empty => ((-1, -1), (-1, -1)),
            };
            goes_to[row].push(x);
        }
    }
    goes_to
}

fn build_path(start_cell: Cell, cell_goes_to: Vec<Vec<(Cell, Cell)>>) -> Vec<Cell> {
    let mut path: Vec<Cell> = vec![];
    path.push(start_cell);
    path.push(cell_goes_to[start_cell.0 as usize][start_cell.1 as usize].0);

    loop {
        let prev_cell = path[path.len() - 1];
        let goes_to_a = cell_goes_to[prev_cell.0 as usize][prev_cell.1 as usize].0;
        let goes_to_b = cell_goes_to[prev_cell.0 as usize][prev_cell.1 as usize].1;

        if path.len() > 2 && (goes_to_a == start_cell || goes_to_b == start_cell) {
            break;
        } else if path[path.len() - 2] != goes_to_a {
            path.push(goes_to_a);
        } else if path[path.len() - 2] != goes_to_b {
            path.push(goes_to_b);
        } else {
            unreachable!();
        }
    }
    path
}

// Puzzle is symmetrical, so looking left/right must work too. Check just in case I've missed something.
fn is_cell_inside_loop(r: usize, c: usize, board: &Board, num_cols: usize) -> bool {
    let mut count_to_left = 0;
    let mut count_to_right = 0;

    for c2 in 0..num_cols {
        // We could also have tested for Vertical || BottomLeft || BottomRight, but not any other combination.
        // We could also have scanned by rows and tested for either of:
        // - Horizontal || TopLeft || BottomLeft
        // - Horizontal || TopRight || BottomRight
        if board[r][c2] == PipeType::Vertical
            || board[r][c2] == PipeType::TopLeft
            || board[r][c2] == PipeType::TopRight
        {
            if c2 < c {
                count_to_left += 1;
            } else {
                count_to_right += 1;
            }
        }
    }

    // Counting to both left and right, then checking, isn't necessary, but increases confidence that code is correct.
    assert_eq!(count_to_left % 2, count_to_right % 2);

    count_to_left % 2 == 1
}

pub fn solve10(input: Vec<String>) -> (i128, i128) {
    let num_rows = input.len();
    let num_cols = input[0].len();

    let mut board = init_board(input, num_rows);

    // For each cell, build a pair of cells that its pipe joins to. Also (both done inside build_cell_goes_to):
    // - set the starting cell
    // - set the actual pipe type of the starting cell in board.
    let mut start_cell = (0, 0);
    let cell_goes_to = build_cell_goes_to(&mut board, num_rows, num_cols, &mut start_cell);

    let path = build_path(start_cell, cell_goes_to);

    let part_1_solution = path.len() / 2;

    let mut cell_type_is_part_of_loop: Vec<Vec<bool>> = vec![vec![false; num_cols]; num_rows];
    for cell in path {
        cell_type_is_part_of_loop[cell.0 as usize][cell.1 as usize] = true;
    }

    // Remove all non-loop pipes from the board, leaving just the loop pipes and the empty cells.
    for row in 0..num_rows {
        for col in 0..num_cols {
            if !cell_type_is_part_of_loop[row][col] {
                board[row][col] = PipeType::Empty;
            }
        }
    }

    // Count the number of empty cells inside the loop.
    //
    // We can determine if a cell is inside the loop if there is an odd number of vertical paths to its left and to its right.
    //
    // Because there is nothing particularly special about the orientation, then a cell is also inside
    // the loop if there is an odd number of horizontal paths above and below it.
    //
    // Examples of an even number of vertical paths "closed loops".
    // - ||
    // - F7
    // - LJ
    // Examples of an odd number of vertical paths "open loops".
    // - |
    // - FJ
    let mut num_empty_cells_inside_the_loop = 0;
    for r in 1..(num_rows - 1) {
        for c in 1..(num_cols - 1) {
            if !cell_type_is_part_of_loop[r][c] && is_cell_inside_loop(r, c, &board, num_cols) {
                num_empty_cells_inside_the_loop += 1;
            }
        }
    }

    (
        part_1_solution as i128,
        num_empty_cells_inside_the_loop as i128,
    )
}

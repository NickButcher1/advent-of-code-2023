use crate::board::Board;

const PIPE_HORIZONTAL: char = '-';
const PIPE_VERTICAL: char = '|';
const PIPE_TOP_LEFT: char = 'F';
const PIPE_TOP_RIGHT: char = '7';
const PIPE_BOTTOM_LEFT: char = 'L';
const PIPE_BOTTOM_RIGHT: char = 'J';
const PIPE_START: char = 'S';
const EMPTY: char = '.';

type Cell = (usize, usize);

// This is awful. There must be a better way!
fn build_start_cell_goes_to(board: &mut Board, row: usize, col: usize) -> (Cell, Cell) {
    let mut start_cell_exit_up = false;
    let mut start_cell_exit_down = false;
    let mut start_cell_exit_right = false;
    let mut start_cell_exit_left = false;

    let mut start_cell_goes_to: Vec<Cell> = vec![];
    if row != 0 {
        let cell = &board.cells[row - 1][col];
        if *cell == PIPE_VERTICAL || *cell == PIPE_TOP_LEFT || *cell == PIPE_TOP_RIGHT {
            start_cell_goes_to.push((row - 1, col));
            start_cell_exit_up = true;
        }
    }
    if row != (board.num_rows - 1) {
        let cell = &board.cells[row + 1][col];
        if *cell == PIPE_VERTICAL || *cell == PIPE_BOTTOM_LEFT || *cell == PIPE_BOTTOM_RIGHT {
            start_cell_goes_to.push((row + 1, col));
            start_cell_exit_down = true;
        }
    }
    if col != 0 {
        let cell = &board.cells[row][col - 1];
        if *cell == PIPE_HORIZONTAL
            || *cell == PIPE_TOP_LEFT
            || *cell == PIPE_TOP_RIGHT
            || *cell == PIPE_BOTTOM_LEFT
            || *cell == PIPE_BOTTOM_RIGHT
        {
            start_cell_goes_to.push((row, col - 1));
            start_cell_exit_left = true;
        }
    }
    if col != (board.num_cols - 1) {
        let cell = &board.cells[row][col + 1];
        if *cell == PIPE_HORIZONTAL
            || *cell == PIPE_TOP_LEFT
            || *cell == PIPE_TOP_RIGHT
            || *cell == PIPE_BOTTOM_LEFT
            || *cell == PIPE_BOTTOM_RIGHT
        {
            start_cell_goes_to.push((row, col + 1));
            start_cell_exit_right = true;
        }
    }
    let start_cell_type = if start_cell_exit_up && start_cell_exit_right {
        PIPE_BOTTOM_LEFT
    } else if start_cell_exit_up && start_cell_exit_down {
        PIPE_VERTICAL
    } else if start_cell_exit_up && start_cell_exit_left {
        PIPE_BOTTOM_RIGHT
    } else if start_cell_exit_right && start_cell_exit_down {
        PIPE_TOP_LEFT
    } else if start_cell_exit_right && start_cell_exit_left {
        PIPE_HORIZONTAL
    } else if start_cell_exit_down && start_cell_exit_left {
        PIPE_TOP_RIGHT
    } else {
        unreachable!();
    };
    board.cells[row][col] = start_cell_type;
    (start_cell_goes_to[0], start_cell_goes_to[1])
}

fn build_cell_goes_to(board: &mut Board, start_cell: &mut Cell) -> Vec<Vec<(Cell, Cell)>> {
    let mut goes_to: Vec<Vec<(Cell, Cell)>> = vec![vec![]; board.num_rows];
    for row in 0..board.num_rows {
        for col in 0..board.num_cols {
            let x = match board.cells[row][col] {
                PIPE_HORIZONTAL => ((row, col - 1), (row, col + 1)),
                PIPE_VERTICAL => ((row - 1, col), (row + 1, col)),
                PIPE_TOP_LEFT => ((row, col + 1), (row + 1, col)),
                PIPE_TOP_RIGHT => ((row, col - 1), (row + 1, col)),
                PIPE_BOTTOM_LEFT => ((row, col + 1), (row - 1, col)),
                PIPE_BOTTOM_RIGHT => ((row, col - 1), (row - 1, col)),
                PIPE_START => {
                    *start_cell = (row, col);
                    build_start_cell_goes_to(board, row, col)
                }
                EMPTY => ((0, 0), (0, 0)),
                _ => unreachable!(),
            };
            goes_to[row].push(x);
        }
    }
    goes_to
}

fn build_path(start_cell: Cell, cell_goes_to: Vec<Vec<(Cell, Cell)>>) -> Vec<Cell> {
    let mut path: Vec<Cell> = vec![];
    path.push(start_cell);
    path.push(cell_goes_to[start_cell.0][start_cell.1].0);

    loop {
        let prev_cell = path[path.len() - 1];
        let goes_to_a = cell_goes_to[prev_cell.0][prev_cell.1].0;
        let goes_to_b = cell_goes_to[prev_cell.0][prev_cell.1].1;

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
fn is_cell_inside_loop(r: usize, c: usize, board: &Board) -> bool {
    let mut count_to_left = 0;
    let mut count_to_right = 0;

    for c2 in 0..board.num_cols {
        // We could also have tested for Vertical || BottomLeft || BottomRight, but not any other combination.
        // We could also have scanned by rows and tested for either of:
        // - Horizontal || TopLeft || BottomLeft
        // - Horizontal || TopRight || BottomRight
        if board.cells[r][c2] == PIPE_VERTICAL
            || board.cells[r][c2] == PIPE_TOP_LEFT
            || board.cells[r][c2] == PIPE_TOP_RIGHT
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
    let mut board: Board = Board::from_input(input);

    // For each cell, build a pair of cells that its pipe joins to. Also (both done inside build_cell_goes_to):
    // - set the starting cell
    // - set the actual pipe type of the starting cell in board.
    let mut start_cell = (0, 0);
    let cell_goes_to = build_cell_goes_to(&mut board, &mut start_cell);

    let path = build_path(start_cell, cell_goes_to);

    let part_1_solution = path.len() / 2;

    let mut cell_type_is_part_of_loop: Vec<Vec<bool>> =
        vec![vec![false; board.num_cols]; board.num_rows];
    for cell in path {
        cell_type_is_part_of_loop[cell.0][cell.1] = true;
    }

    // Remove all non-loop pipes from the board, leaving just the loop pipes and the empty cells.
    for row in 0..board.num_rows {
        for col in 0..board.num_cols {
            if !cell_type_is_part_of_loop[row][col] {
                board.cells[row][col] = EMPTY;
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

    for r in 1..(board.num_rows - 1) {
        for c in 1..(board.num_cols - 1) {
            if !cell_type_is_part_of_loop[r][c] && is_cell_inside_loop(r, c, &board) {
                num_empty_cells_inside_the_loop += 1;
            }
        }
    }

    (
        part_1_solution as i128,
        num_empty_cells_inside_the_loop as i128,
    )
}

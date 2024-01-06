use aoc::board::Board;
use std::collections::HashSet;

const PATH: char = '.';
const FOREST: char = '#';
const SLOPE_RIGHT: char = '>';
const SLOPE_DOWN: char = 'v';

type Cell = (usize, usize);

const START_CELL: Cell = (1, 2);

// From a junction, represents an immediate neighbour junction and the cost (number of steps) to reach it.
#[derive(Clone, Debug)]
struct Step {
    to: Cell,
    cost: usize,
}

// A path between some number of junctions, always starting from the start junction, and currently
// ending on the `loc` cell.
#[derive(Clone, Debug)]
struct Path {
    loc: Cell,
    cost: usize,
    junctions_visited: HashSet<Cell>,
}

// Parts 1 and 2 are identical, except that in part 2 the slopes are replaced with paths.
pub fn solve23(input: &[String]) -> (i128, i128) {
    let mut part_1_board = Board::from_input(input);
    part_1_board.add_border(FOREST);

    let mut part_2_board = part_1_board.clone();
    part_2_board.replace(SLOPE_DOWN, PATH);
    part_2_board.replace(SLOPE_RIGHT, PATH);

    (solve(&part_1_board) as i128, solve(&part_2_board) as i128)
}

pub fn solve(board: &Board) -> usize {
    // This is a bit convoluted. At the same time, build both of:
    // - A list of valid moves from each cell.
    // - A list of junctions, where a junction is one of:
    //   - The start cell.
    //   - The end cell.
    //   - Any grid cell where the path forks.
    let (valid_moves, junctions) = build_valid_moves_between_cells(board);

    // Now build the list of valid moves between junctions, instead of between cells. This is
    // much smaller. Then brute force all possible paths from start cell to end cell via junctions.
    find_longest_path(
        board,
        &build_valid_moves_between_junctions(board, &junctions, &valid_moves),
    )
}

#[allow(clippy::needless_range_loop)]
fn build_valid_moves_between_cells(board: &Board) -> (Vec<Vec<Vec<Cell>>>, Vec<Cell>) {
    let mut valid_moves: Vec<Vec<Vec<Cell>>> = vec![vec![vec![]; board.num_cols]; board.num_rows];
    let mut junctions: Vec<Cell> = vec![];

    for r in 1..board.num_rows - 1 {
        for c in 1..board.num_cols - 1 {
            match board.cells[r][c] {
                PATH => {
                    let mut inbound_count = 0;
                    if board.cells[r - 1][c] == PATH {
                        valid_moves[r][c].push((r - 1, c));
                        inbound_count += 1;
                    }
                    if board.cells[r + 1][c] == PATH {
                        valid_moves[r][c].push((r + 1, c));
                        inbound_count += 1;
                    }
                    if board.cells[r][c - 1] == PATH {
                        valid_moves[r][c].push((r, c - 1));
                        inbound_count += 1;
                    }
                    if board.cells[r][c + 1] == PATH {
                        valid_moves[r][c].push((r, c + 1));
                        inbound_count += 1;
                    }
                    if board.cells[r + 1][c] == SLOPE_DOWN {
                        valid_moves[r][c].push((r + 1, c));
                    }
                    if board.cells[r][c + 1] == SLOPE_RIGHT {
                        valid_moves[r][c].push((r, c + 1));
                    }
                    if inbound_count != 2 {
                        junctions.push((r, c));
                    }
                }
                SLOPE_DOWN => valid_moves[r][c].push((r + 1, c)),
                SLOPE_RIGHT => valid_moves[r][c].push((r, c + 1)),
                _ => {}
            }
        }
    }
    (valid_moves, junctions)
}

fn build_valid_moves_between_junctions(
    board: &Board,
    junctions: &Vec<Cell>,
    valid_moves: &[Vec<Vec<Cell>>],
) -> Vec<Vec<Vec<Step>>> {
    let mut new_valid_moves: Vec<Vec<Vec<Step>>> =
        vec![vec![vec![]; board.num_cols]; board.num_rows];

    // Now find all the ways to traverse between junctions.
    for junction in junctions {
        for valid_move in &valid_moves[junction.0][junction.1] {
            // For a specific valid move out of a specific junction, there must be a single path to
            // the next junction. Follow it and count the cost (number of steps), then condense it
            // into a valid move between two junctions.
            let mut prev_cell = junction;
            let mut this_move = valid_move;
            let mut cost = 1;

            while !junctions.contains(this_move) {
                let next_moves = &valid_moves[this_move.0][this_move.1];
                if next_moves.len() == 2 {
                    this_move = if next_moves[0] == *prev_cell {
                        prev_cell = &this_move;
                        &next_moves[1]
                    } else if next_moves[1] == *prev_cell {
                        prev_cell = &this_move;
                        &next_moves[0]
                    } else {
                        panic!();
                    };
                } else {
                    assert_eq!(next_moves.len(), 1);
                    prev_cell = &this_move;
                    this_move = &next_moves[0];
                }
                cost += 1;
            }
            new_valid_moves[junction.0][junction.1].push(Step {
                to: *this_move,
                cost,
            });
        }
    }

    new_valid_moves
}

fn find_longest_path(board: &Board, valid_moves: &[Vec<Vec<Step>>]) -> usize {
    let end_cell = (board.num_rows - 2, board.num_cols - 3);
    let mut longest_path_found = 0;
    let mut partial_paths: Vec<Path> = vec![Path {
        loc: START_CELL,
        cost: 0,
        junctions_visited: HashSet::from([START_CELL]),
    }];

    // Expand all possible paths. For each partial path, add all outgoing partial paths back into
    // the list.
    while let Some(path) = partial_paths.pop() {
        for valid_move in &valid_moves[path.loc.0][path.loc.1] {
            let mut junctions_visited = path.junctions_visited.clone();

            if !junctions_visited.contains(&valid_move.to) {
                junctions_visited.insert(valid_move.to);
                let new_path = Path {
                    loc: valid_move.to,
                    cost: path.cost + valid_move.cost,
                    junctions_visited,
                };
                if valid_move.to != end_cell {
                    partial_paths.push(new_path);
                } else if new_path.cost > longest_path_found {
                    longest_path_found = new_path.cost;
                }
            }
        }
    }

    longest_path_found
}

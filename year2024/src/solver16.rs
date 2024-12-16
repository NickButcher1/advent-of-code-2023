use aoc::board::Board;
use itertools::iproduct;
use std::collections::{HashMap, HashSet};

const START: char = 'S';
const END: char = 'E';
const EMPTY: char = '.';

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct Position {
    r: usize,
    c: usize,
    dir: usize,
}

fn move_position(position: &Position) -> Position {
    let (dr, dc) = [(-1, 0), (0, 1), (1, 0), (0, -1)][position.dir];
    Position {
        r: (position.r as i32 + dr) as usize,
        c: (position.c as i32 + dc) as usize,
        dir: position.dir,
    }
}

pub fn solve16(input: &[String]) -> (i128, i128) {
    let mut board = Board::from_input(input);
    let start = board.find(START);
    let end = board.find(END);
    let end_position = Position {
        r: end.0,
        c: end.1,
        dir: 0,
    };

    board.cells[start.0][start.1] = EMPTY;
    board.cells[end.0][end.1] = EMPTY;

    // Map from position to the lowest score found so far at that position.
    let mut lowest_score: HashMap<Position, usize> = HashMap::new();

    // Map from position to all tiles on all lowest-score paths to that position.
    let mut lowest_cells: HashMap<Position, HashSet<(usize, usize)>> = HashMap::new();

    // Tips of paths that need to be explored.
    // N=0, E=1, S=2, W= 3.
    let mut live_points: Vec<Position> = vec![];

    // Initialise the starting state.
    let start_position = Position {
        r: start.0,
        c: start.1,
        dir: 1,
    };
    live_points.push(start_position.clone());
    let mut initial_tiles_visited = HashSet::new();
    initial_tiles_visited.insert((start.0, start.1));
    lowest_score.insert(start_position.clone(), 0);
    for (c, r) in iproduct!(0..board.num_cols, 0..board.num_rows) {
        lowest_cells.insert(Position { r, c, dir: 0 }, HashSet::new());
        lowest_cells.insert(Position { r, c, dir: 1 }, HashSet::new());
        lowest_cells.insert(Position { r, c, dir: 2 }, HashSet::new());
        lowest_cells.insert(Position { r, c, dir: 3 }, HashSet::new());

        lowest_score.insert(Position { r, c, dir: 0 }, usize::MAX);
        lowest_score.insert(Position { r, c, dir: 1 }, usize::MAX);
        lowest_score.insert(Position { r, c, dir: 2 }, usize::MAX);
        lowest_score.insert(Position { r, c, dir: 3 }, usize::MAX);
    }
    lowest_cells.insert(start_position.clone(), initial_tiles_visited);
    lowest_score.insert(start_position, 0);

    let mut loops = 0;
    while let Some(position) = live_points.pop() {
        let score = *lowest_score.get(&position).unwrap_or(&0);

        loops += 1;
        // if loops % 100000 == 0 {
        println!(
            "Loop {loops}, live_positons len {}    {:?}",
            live_points.len(),
            live_points
        );
        let x1 = lowest_score
            .get(&Position {
                r: end.0,
                c: end.1,
                dir: 0,
            })
            .unwrap();
        let x2 = lowest_score
            .get(&Position {
                r: end.0,
                c: end.1,
                dir: 1,
            })
            .unwrap();
        let x3 = lowest_score
            .get(&Position {
                r: end.0,
                c: end.1,
                dir: 2,
            })
            .unwrap();
        let x4 = lowest_score
            .get(&Position {
                r: end.0,
                c: end.1,
                dir: 3,
            })
            .unwrap();
        let y1 = lowest_cells
            .get(&Position {
                r: end.0,
                c: end.1,
                dir: 0,
            })
            .unwrap()
            .len();
        let y2 = lowest_cells
            .get(&Position {
                r: end.0,
                c: end.1,
                dir: 1,
            })
            .unwrap()
            .len();
        let y3 = lowest_cells
            .get(&Position {
                r: end.0,
                c: end.1,
                dir: 2,
            })
            .unwrap()
            .len();
        let y4 = lowest_cells
            .get(&Position {
                r: end.0,
                c: end.1,
                dir: 3,
            })
            .unwrap()
            .len();
        println!("    {x1}  {y1}");
        println!("    {x2}  {y2}");
        println!("    {x3}  {y3}");
        println!("    {x4}  {y4}");
        // }

        for new_dir in 0..=3 {
            if new_dir != position.dir {
                let new_position = Position {
                    r: position.r,
                    c: position.c,
                    dir: new_dir,
                };
                let next_position = move_position(&new_position);
                if board.cells[next_position.r][next_position.c] == EMPTY {
                    let old_lowest_score = *lowest_score.get(&new_position).unwrap_or(&0);
                    let new_score = score + 1000;

                    if new_score <= *lowest_score.get(&end_position).unwrap() {
                        if new_score < old_lowest_score {
                            let old_cell_tiles_visited = lowest_cells.get(&position).unwrap();
                            let new_tiles_visited = old_cell_tiles_visited.clone();
                            lowest_cells.insert(new_position.clone(), new_tiles_visited);

                            lowest_score.insert(new_position.clone(), new_score);
                            live_points.push(new_position);
                        } else if new_score == old_lowest_score {
                            let old_cell_tiles_visited = lowest_cells.get(&position).unwrap();
                            let new_cell_tiles_visited = lowest_cells.get(&new_position).unwrap();
                            let mut new_tiles_visited = new_cell_tiles_visited.clone();
                            new_tiles_visited.extend(old_cell_tiles_visited);
                            if new_tiles_visited != *new_cell_tiles_visited {
                                live_points.push(new_position.clone());
                            }
                            lowest_cells.insert(new_position, new_tiles_visited);
                        }
                    }
                }
            }
        }

        let new_position = move_position(&position);

        if board.cells[new_position.r][new_position.c] == EMPTY {
            let old_lowest_score = lowest_score.get(&new_position).unwrap_or(&0);
            let new_score = score + 1;
            if new_score <= *lowest_score.get(&end_position).unwrap() {
                if new_score < *old_lowest_score {
                    let old_cell_tiles_visited = lowest_cells.get(&position).unwrap();
                    let mut new_tiles_visited = old_cell_tiles_visited.clone();
                    new_tiles_visited.insert((new_position.r, new_position.c));
                    lowest_cells.insert(new_position.clone(), new_tiles_visited);

                    lowest_score.insert(new_position.clone(), new_score);
                    live_points.push(new_position);
                } else if new_score == *old_lowest_score {
                    let old_cell_tiles_visited = lowest_cells.get(&position).unwrap();
                    let new_cell_tiles_visited = lowest_cells.get(&new_position).unwrap();
                    let mut new_tiles_visited = new_cell_tiles_visited.clone();
                    new_tiles_visited.extend(old_cell_tiles_visited);
                    if new_tiles_visited != *new_cell_tiles_visited {
                        live_points.push(new_position.clone());
                    }
                    lowest_cells.insert(new_position, new_tiles_visited);
                }
            }
        }
    }

    let (min_dir, min_score) = (0..=3)
        .map(|i| {
            (
                i,
                *lowest_score
                    .get(&Position {
                        r: end.0,
                        c: end.1,
                        dir: i,
                    })
                    .unwrap(),
            )
        })
        .min_by_key(|&(_, score)| score)
        .unwrap();

    let solution_one = min_score;
    let solution_two = lowest_cells
        .get(&Position {
            r: end.0,
            c: end.1,
            dir: min_dir,
        })
        .unwrap()
        .len();

    (solution_one as i128, solution_two as i128)
}

use std::collections::{HashMap, HashSet};
use itertools::iproduct;
use aoc::board::Board;

const START: char = 'S';
const END: char = 'E';
const EMPTY: char = '.';

fn new_position(r: usize, c: usize, dir: usize) -> (usize, usize) {
    let (dr, dc) = [(-1, 0), (0, 1), (1, 0), (0, -1)][dir];
    ((r as i32 + dr) as usize, (c as i32 + dc) as usize)
}
pub fn solve16(input: &[String]) -> (i128, i128) {
    let mut board = Board::from_input(input);
    let start = board.find(START);
    let end = board.find(END);

    board.cells[start.0][start.1] = EMPTY;
    board.cells[end.0][end.1] = EMPTY;

    // Map from r,c,dir to lowest score found so far.
    let mut lowest_score: HashMap<(usize, usize, usize), usize> = HashMap::new();

    // Map from r,c,dir to all tiles on all lowest-score paths to that position.
    let mut lowest_cells: HashMap<(usize, usize, usize), HashSet<(usize, usize)>> = HashMap::new();

    // Tips of paths that need to be explored: r,c,dir.
    // N=0, E=1, S=2, W= 3.
    let mut live_points: Vec<(usize, usize, usize)> = vec![];

    // Initialise the starting state.
    live_points.push((start.0, start.1, 1));
    let mut initial_tiles_visited = HashSet::new();
    initial_tiles_visited.insert((start.0, start.1));
    lowest_score.insert((start.0, start.1, 1), 0);
    for (c, r) in iproduct!(0..board.num_cols, 0..board.num_rows) {
        lowest_cells.insert((r,c, 0), HashSet::new());
        lowest_cells.insert((r,c, 1), HashSet::new());
        lowest_cells.insert((r,c, 2), HashSet::new());
        lowest_cells.insert((r,c, 3), HashSet::new());

        lowest_score.insert((r,c, 0), usize::MAX);
        lowest_score.insert((r,c, 1), usize::MAX);
        lowest_score.insert((r,c, 2), usize::MAX);
        lowest_score.insert((r,c, 3), usize::MAX);
    }
    lowest_cells.insert((start.0, start.1, 1), initial_tiles_visited);
    lowest_score.insert((start.0, start.1, 1), 0);

    let mut loops = 0;
    while !live_points.is_empty() {
        let (r,c,dir) = live_points.pop().unwrap();
        let xx = lowest_score.get(&(r,c,dir)).unwrap_or(&0);
        let score = xx.clone();

        loops += 1;
        if loops % 100000 == 0 {
            println!("Loop {loops}");
            let x1 = lowest_score.get(&(end.0, end.1, 0)).unwrap();
            let x2 = lowest_score.get(&(end.0, end.1, 1)).unwrap();
            let x3 = lowest_score.get(&(end.0, end.1, 2)).unwrap();
            let x4 = lowest_score.get(&(end.0, end.1, 3)).unwrap();
            let y1 = lowest_cells.get(&(end.0, end.1, 0)).unwrap().len();
            let y2 = lowest_cells.get(&(end.0, end.1, 1)).unwrap().len();
            let y3 = lowest_cells.get(&(end.0, end.1, 2)).unwrap().len();
            let y4 = lowest_cells.get(&(end.0, end.1, 3)).unwrap().len();
            println!("    {x1}  {y1}");
            println!("    {x2}  {y2}");
            println!("    {x3}  {y3}");
            println!("    {x4}  {y4}");
        }

        for new_dir in 0..=3 {
            if new_dir != dir {
                let (new_r, new_c) = new_position(r, c, new_dir);
                if board.cells[new_r][new_c] == EMPTY {
                    let x = lowest_score.get(&(r,c,new_dir)).unwrap_or(&0);
                    let old_lowest_score = x.clone();
                    let new_score = score + 1000;

                    if new_score > *lowest_score.get(&(end.0, end.1, 0)).unwrap() {
                    } else {
                        if new_score < old_lowest_score {
                            let old_cell_tiles_visited = lowest_cells.get(&(r, c, dir)).unwrap();
                            let new_tiles_visited = old_cell_tiles_visited.clone();
                            lowest_cells.insert((r, c, new_dir), new_tiles_visited);

                            lowest_score.insert((r, c, new_dir), new_score);
                            live_points.push((r, c, new_dir));
                        } else if new_score == old_lowest_score {
                            let old_cell_tiles_visited = lowest_cells.get(&(r, c, dir)).unwrap();
                            let new_cell_tiles_visited = lowest_cells.get(&(r, c, new_dir)).unwrap();
                            let mut new_tiles_visited= new_cell_tiles_visited.clone();
                            new_tiles_visited.extend(old_cell_tiles_visited);
                            if new_tiles_visited != *new_cell_tiles_visited {
                                live_points.push((r, c, new_dir));
                            }
                            lowest_cells.insert((r, c, new_dir), new_tiles_visited);
                        }
                    }
                }
            }
        }

        let (new_r, new_c) = new_position(r, c, dir);

        if board.cells[new_r][new_c] == EMPTY {
            let old_lowest_score = lowest_score.get(&(new_r,new_c,dir)).unwrap_or(&0);
            let new_score = score + 1;
            if new_score < *old_lowest_score {
                let old_cell_tiles_visited = lowest_cells.get(&(r, c, dir)).unwrap();
                let mut new_tiles_visited= old_cell_tiles_visited.clone();
                new_tiles_visited.insert((new_r, new_c));
                lowest_cells.insert((new_r, new_c, dir), new_tiles_visited);

                lowest_score.insert((new_r, new_c, dir), new_score);
                live_points.push((new_r, new_c, dir));
            } else if new_score == *old_lowest_score {
                let old_cell_tiles_visited = lowest_cells.get(&(r, c, dir)).unwrap();
                let new_cell_tiles_visited = lowest_cells.get(&(new_r, new_c, dir)).unwrap();
                let mut new_tiles_visited= new_cell_tiles_visited.clone();
                new_tiles_visited.extend(old_cell_tiles_visited);
                if new_tiles_visited != *new_cell_tiles_visited {
                    live_points.push((new_r, new_c, dir));
                }
                lowest_cells.insert((new_r, new_c, dir), new_tiles_visited);
            }
        }
    }

    let (min_dir, min_score) = (0..=3)
        .map(|i| (i, *lowest_score.get(&(end.0, end.1, i)).unwrap()))
        .min_by_key(|&(_, score)| score)
        .unwrap();

    let solution_one = min_score;
    let solution_two = lowest_cells.get(&(end.0, end.1, min_dir)).unwrap().len();

    (solution_one as i128, solution_two as i128)
}

use std::collections::VecDeque;
use itertools::iproduct;
use aoc::board::Board;
use aoc::int_board::IntBoard;
use aoc::point::read_points;
use aoc::solution::{Solution, Solutions};

const SAFE: char = '.';
const CORRUPT: char = '#';

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct Position {
    r: usize,
    c: usize,
    steps: usize,
}

pub fn solve18(input: &[String]) -> Solutions {
    let points = read_points(input);
    let (board_size, max_points) = if points.len() == 25 { (7, 12) } else { (71, 1024) };
    let mut board = Board::create_empty(board_size, board_size, SAFE);
    board.add_border(CORRUPT);

    println!("POINTS: {points:?}");
    board.print();

    let (mut r,mut c) = (1,1);
    let (target_r, target_c) = (board_size, board_size);
    println!("{target_r}");

    // let mut corrupt_board = IntBoard::create_empty(board_size, board_size);
    // corrupt_board.add_border(-1);
    // points.iter().enumerate().for_each(|(i, point)| {
    //     corrupt_board.cells[point.y as usize + 1][point.x as usize + 1] = i as i32 + 1;
    // });
    // corrupt_board.print();

    for i in 0..max_points {
        board.cells[points[i].y as usize + 1][points[i].x as usize + 1] = CORRUPT;
    }
    board.print();

    // Tips of paths that need to be explored.
    let start_position = Position { r: 1, c: 1, steps: 0 };
    let mut live_points: VecDeque<Position> = VecDeque::new();
    live_points.push_back(start_position.clone());

    let mut lowest_steps_to_end = IntBoard::create_empty(board_size + 2, board_size + 2);
    for (c, r) in iproduct!(0..lowest_steps_to_end.num_cols, 0..lowest_steps_to_end.num_rows) {
        lowest_steps_to_end.cells[r][c] = i32::MAX;
    }
    lowest_steps_to_end.cells[1][1] = 0;

    while let Some(position) = live_points.pop_front() {
        println!("Process: {position:?}");
        for new_dir in 0..=3 {
            let (dr, dc) = [(-1, 0), (0, 1), (1, 0), (0, -1)][new_dir];
            let new_position = Position {
                r: (position.r as i32 + dr) as usize,
                c: (position.c as i32 + dc) as usize,
                steps: position.steps + 1
            };

            if board.cells[new_position.r][new_position.c] == SAFE {
                println!("    can move: {dr},{dc}");
                if new_position.steps < lowest_steps_to_end.cells[new_position.r][new_position.c] as usize {
                    println!("        and new lowest at {},{} with {} steps", new_position.r, new_position.c, new_position.steps);
                    lowest_steps_to_end.cells[new_position.r][new_position.c] = new_position.steps as i32;

                    if new_position.r != target_r || new_position.c != target_c {
                        live_points.push_back(new_position);
                    } else {
                        println!("            DON'T QUEUE - FINAL POSITION");
                    }
                }
            }
        }
    }

    println!("LOWEST STEPS:");
    lowest_steps_to_end.print();
    let solution_one = lowest_steps_to_end.cells[target_r][target_c];
    println!("SOLUTION ONE: {solution_one}");

    (Solution::I32(solution_one), Solution::U32(0))
}

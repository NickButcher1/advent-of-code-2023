use std::collections::HashMap;
use itertools::iproduct;
use aoc::board::Board;

const COMPLETE: char = '.';

fn perimeta_count(board: &Board, r: usize, c: usize, key: char) -> i32 {
    println!("PC for {r},{c} KEY {key}");
    let mut perimeta_count = 0;
    [(-1, 0), (0, 1), (1, 0), (0, -1)]
        .iter()
        .for_each(|(dr, dc)| {
            let new_r = (r as i32 + dr) as usize;
            let new_c = (c as i32 + dc) as usize;
            if board.cells[new_r][new_c] != key {
                perimeta_count += 1;
            }
        });
    perimeta_count
}

pub fn solve12(input: &[String]) -> (i128, i128) {
    let mut input_board = Board::from_input(input);
    input_board.add_border(COMPLETE);
    let mut board = Board::from_input(input);
    board.add_border(COMPLETE);
    // let mut perimeta_of_regions: HashMap<char, i32> = HashMap::new();
    // let mut area_of_regions: HashMap<char, i32> = HashMap::new();
    let mut regions = vec![];

    // Loop over every board cell. Each time a cell we haven't processed yet is found, flood fill it.
    let mut queue: Vec<(usize, usize)> = vec![];
    board.print();

    for (r, c) in iproduct!(1..(board.num_rows - 1), 1..(board.num_cols - 1)) {
        if board.cells[r][c] != COMPLETE {
            let key = board.cells[r][c];
            println!("REGION {key} AT {r},{c}");

            let mut area = 0;
            let mut perimeta = 0;

            board.cells[r][c] = COMPLETE;
            area += 1;
            perimeta += perimeta_count(&input_board, r, c, key);
            queue.push((r, c));

            while !queue.is_empty() {
                let (test_r, test_c) = queue.pop().unwrap();
                [(-1, 0), (0, 1), (1, 0), (0, -1)]
                    .iter()
                    .for_each(|(dr, dc)| {
                        let new_r = (test_r as i32 + dr) as usize;
                        let new_c = (test_c as i32 + dc) as usize;
                        if board.cells[new_r][new_c] == key {
                            board.cells[new_r][new_c] = COMPLETE;
                            area += 1;
                            perimeta += perimeta_count(&input_board, new_r, new_c, key);
                            queue.push((new_r, new_c));
                        }
                    });
            }

            regions.push((key, area, perimeta));
            // *area_of_regions.entry(key).or_insert(0) += area;
            board.print();
        }
    }

    let mut total_cost = 0;
    for (key, area, perimeta) in regions {
        println!("REGION    {key}    {area}    {perimeta}");
        total_cost += area * perimeta;
    }

    (total_cost as i128, 0 as i128)
}

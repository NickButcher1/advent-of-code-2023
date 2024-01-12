use aoc::point::{max_x_y_for_pairs, read_point_pairs};
use std::cmp::{max, min};

pub fn solve05(input: &[String]) -> (i128, i128) {
    let points = read_point_pairs(input);
    let (max_x, max_y) = max_x_y_for_pairs(&points);

    let mut cells: Vec<Vec<isize>> = vec![vec![0; 1 + max_y as usize]; 1 + max_x as usize];
    let mut cells_for_diagonal: Vec<Vec<isize>> =
        vec![vec![0; 1 + max_y as usize]; 1 + max_x as usize];

    for (point_from, point_to) in points {
        if point_from.x == point_to.x {
            let from_y = min(point_from.y, point_to.y);
            let to_y = max(point_from.y, point_to.y);
            for y in from_y..=to_y {
                cells[point_from.x as usize][y as usize] += 1;
            }
        } else if point_from.y == point_to.y {
            let from_x = min(point_from.x, point_to.x);
            let to_x = max(point_from.x, point_to.x);
            for x in from_x..=to_x {
                cells[x as usize][point_from.y as usize] += 1;
            }
        } else {
            // Diagonal.
            let from_x = min(point_from.x, point_to.x);
            let to_x = max(point_from.x, point_to.x);
            let (from_y, to_y) = if point_from.x > point_to.x {
                (point_from.y, point_to.y)
            } else {
                (point_to.y, point_from.y)
            };
            if from_y > to_y {
                let y_range = to_y..=from_y;
                for (x, y) in (from_x..=to_x).zip(y_range) {
                    cells_for_diagonal[x as usize][y as usize] += 1;
                }
            } else {
                let y_range = (from_y..=to_y).rev();
                for (x, y) in (from_x..=to_x).zip(y_range) {
                    cells_for_diagonal[x as usize][y as usize] += 1;
                }
            };
        }
    }

    let mut part_1 = 0;
    let mut part_2 = 0;
    for x in 0..=max_x {
        for y in 0..=max_y {
            if cells[x as usize][y as usize] > 1 {
                part_1 += 1;
            }
            if cells[x as usize][y as usize] + cells_for_diagonal[x as usize][y as usize] > 1 {
                part_2 += 1;
            }
        }
    }

    (part_1 as i128, part_2 as i128)
}

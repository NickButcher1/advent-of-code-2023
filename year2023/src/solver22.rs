use aoc::input::string_to_vec_usize;
use std::collections::HashSet;

#[derive(Clone, Debug)]
struct Cell {
    x: usize,
    y: usize,
    z: usize,
}

struct InputBrick {
    id: usize,
    from: Cell,
    to: Cell,
}

#[derive(Clone, Debug)]
struct Brick {
    id: usize,
    cells: Vec<Cell>,
}

fn read_bricks_from_input(input: &[String]) -> (Vec<Brick>, Vec<Vec<Vec<usize>>>) {
    let mut input_bricks: Vec<InputBrick> = vec![];
    let mut next_brick_id = 1;

    for line in input {
        let split_1: Vec<&str> = line.split('~').collect();
        let from_vec = string_to_vec_usize(split_1[0], ',');
        let to_vec = string_to_vec_usize(split_1[1], ',');
        let from = Cell {
            x: from_vec[0],
            y: from_vec[1],
            z: from_vec[2],
        };
        let to = Cell {
            x: to_vec[0],
            y: to_vec[1],
            z: to_vec[2],
        };

        input_bricks.push(InputBrick {
            id: next_brick_id,
            from,
            to,
        });
        next_brick_id += 1;
    }

    let max_x = input_bricks.iter().max_by_key(|b| b.to.x).unwrap().to.x;
    let max_y = input_bricks.iter().max_by_key(|b| b.to.y).unwrap().to.y;
    let max_z = input_bricks.iter().max_by_key(|b| b.to.z).unwrap().to.z;

    let mut bricks: Vec<Brick> = vec![];
    for b in &input_bricks {
        let mut cells = vec![];
        if b.to.x == b.from.x && b.to.y == b.from.y {
            for z in b.from.z..=b.to.z {
                cells.push(Cell {
                    x: b.from.x,
                    y: b.from.y,
                    z,
                });
            }
        } else if b.to.x == b.from.x && b.to.z == b.from.z {
            for y in b.from.y..=b.to.y {
                cells.push(Cell {
                    x: b.from.x,
                    y: y,
                    z: b.from.z,
                });
            }
        } else {
            for x in b.from.x..=b.to.x {
                cells.push(Cell {
                    x,
                    y: b.from.y,
                    z: b.from.z,
                });
            }
        };

        bricks.push(Brick { id: b.id, cells });
    }

    // Build a 3D grid where each cell contains the ID of the brick occupying it, or zero if empty.
    let mut occupied_by: Vec<Vec<Vec<usize>>> =
        vec![vec![vec![0; max_z + 1]; max_y + 1]; max_x + 1];
    for brick in &bricks {
        for cell in &brick.cells {
            occupied_by[cell.x][cell.y][cell.z] = brick.id;
        }
    }

    (bricks, occupied_by)
}

pub fn solve22(input: &[String]) -> (i128, i128) {
    let (mut bricks, mut occupied_by) = read_bricks_from_input(input);

    // Drop all bricks as far as possible.
    drop_bricks(&mut bricks, &mut occupied_by, 0);

    // At this point, both bricks and occupied_by are now fixed, representing the state after dropping as far as possible.

    // Calculate part 1 and part 2 at the same time. Loop over each block and:
    // - Remove that block.
    // - Try and drop the remaining blocks as far as possible.
    // - If nothing dropped further, it counts as safe to disintegrate for part 1. Sum those.
    // - If any blocks did drop, count the number of blocks that dropped for part 2. Sum those.

    let mut part_1_safe_to_disintegrate = 0;
    let mut part_2_total_bricks_dropped = 0;

    for brick_id in 1..=bricks.len() {
        let num_bricks_dropped =
            drop_bricks(&mut bricks.clone(), &mut occupied_by.clone(), brick_id);

        if num_bricks_dropped == 0 {
            part_1_safe_to_disintegrate += 1;
        }
        part_2_total_bricks_dropped += num_bricks_dropped;
    }

    (
        part_1_safe_to_disintegrate,
        part_2_total_bricks_dropped as i128,
    )
}

fn drop_bricks(
    bricks: &mut Vec<Brick>,
    occupied_by: &mut [Vec<Vec<usize>>],
    ignore_brick_id: usize,
) -> usize {
    let mut brick_ids_dropped: HashSet<usize> = HashSet::new();
    let mut dropped_in_last_loop = true;

    while dropped_in_last_loop {
        dropped_in_last_loop = false;
        for brick in &mut *bricks {
            // Can't drop a brick at z=1. It is already on the ground.
            if brick.id == ignore_brick_id || brick.cells[0].z == 1 {
                continue;
            }

            // Check the cell directly under each cell in this brick. If they are all empty, or part
            // of the brick to ignore or part of this brick (for vertical bricks only) then we can drop this brick.
            if brick.cells.iter().all(|cell| {
                let check_id = occupied_by[cell.x][cell.y][cell.z - 1];
                check_id == 0 || check_id == ignore_brick_id || check_id == brick.id
            }) {
                // Move both the cells and the occupied list down one.
                dropped_in_last_loop = true;
                brick_ids_dropped.insert(brick.id);
                brick.cells.iter_mut().for_each(|cell| {
                    cell.z -= 1;
                    occupied_by[cell.x][cell.y][cell.z] = brick.id;
                    occupied_by[cell.x][cell.y][cell.z + 1] = 0;
                });
            }
        }
    }
    brick_ids_dropped.len()
}

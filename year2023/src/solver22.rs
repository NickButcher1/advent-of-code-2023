use aoc::input::string_to_vec_usize;
use std::collections::HashSet;

#[derive(Clone)]
struct BrickEnd {
    x: usize,
    y: usize,
    z: usize,
}

#[derive(Clone)]
struct Brick {
    id: usize,
    from: BrickEnd,
    to: BrickEnd,
    brick_type: BrickType,
}

#[derive(Clone)]
enum BrickType {
    Unknown,
    Vertical,
    HorizontalAlongX,
    HorizontalAlongY,
}

pub fn solve22(input: &[String]) -> (i128, i128) {
    let mut bricks: Vec<Brick> = vec![];
    let mut next_brick_id = 1;

    for line in input {
        let split_1: Vec<&str> = line.split('~').collect();
        let from_coords = string_to_vec_usize(split_1[0], ',');
        let to_coords = string_to_vec_usize(split_1[1], ',');

        let mut brick = Brick {
            id: next_brick_id,
            from: BrickEnd {
                x: from_coords[0],
                y: from_coords[1],
                z: from_coords[2],
            },
            to: BrickEnd {
                x: to_coords[0],
                y: to_coords[1],
                z: to_coords[2],
            },
            brick_type: BrickType::Unknown,
        };
        next_brick_id += 1;

        // Check assumption - first coordinate is always smaller.
        assert!(brick.to.x >= brick.from.x);
        assert!(brick.to.y >= brick.from.y);
        assert!(brick.to.z >= brick.from.z);

        // Check assumption - brick is a single cube or straight line.
        assert!(
            (brick.to.x == brick.from.x && (brick.to.y == brick.from.y))
                || (brick.to.x == brick.from.x && (brick.to.z == brick.from.z))
                || (brick.to.y == brick.from.y && (brick.to.z == brick.from.z))
        );

        brick.brick_type = if brick.to.x == brick.from.x && brick.to.y == brick.from.y {
            BrickType::Vertical // Single cube counts as vertical.
        } else if brick.to.x == brick.from.x && brick.to.z == brick.from.z {
            BrickType::HorizontalAlongY
        } else {
            BrickType::HorizontalAlongX
        };

        bricks.push(brick);
    }

    let max_x = bricks.iter().max_by_key(|brick| brick.to.x).unwrap().to.x;
    let max_y = bricks.iter().max_by_key(|brick| brick.to.y).unwrap().to.y;
    let max_z = bricks.iter().max_by_key(|brick| brick.to.z).unwrap().to.z;

    // Build a 3D grid where each cell contains the ID of the brick occupying it, or zero if empty.
    let mut occupied_by = build_occupied_by(&bricks, max_x, max_y, max_z);

    // Drop all bricks as far as possible.
    drop_bricks(&mut bricks, &mut occupied_by);

    // Calculate part 1 and part 2 at the same time. Loop over each block and:
    // - Remove that block.
    // - Try and drop the remaining blocks as far as possible.
    // - If nothing dropped further, it counts as safe to disintegrate for part 1. Sum those.
    // - If any blocks did drop, count the number of blocks that dropped for part 2. Sum those.

    let mut part_1_safe_to_disintegrate = 0;
    let mut part_2_total_bricks_dropped = 0;

    for brick in &bricks {
        let mut temp_bricks: Vec<Brick> = vec![];
        for temp_brick in &bricks {
            let temp_brick2 = temp_brick.clone();
            if temp_brick2.id != brick.id {
                temp_bricks.push(temp_brick2);
            }
        }

        let mut temp_brick_id = build_occupied_by(&temp_bricks, max_x, max_y, max_z);

        let num_bricks_dropped = drop_bricks(&mut temp_bricks, &mut temp_brick_id);
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

fn drop_bricks(bricks: &mut Vec<Brick>, occupied_by: &mut Vec<Vec<Vec<usize>>>) -> usize {
    let mut brick_ids_dropped: HashSet<usize> = HashSet::new();
    let mut num_bricks_dropped = usize::MAX;
    while num_bricks_dropped != 0 {
        num_bricks_dropped = 0;
        for brick in &mut *bricks {
            // Can't drop a brick at z=1. It is already on the ground.
            if brick.from.z == 1 {
                continue;
            }
            match brick.brick_type {
                BrickType::Unknown => {
                    panic!("    ERROR!");
                }
                BrickType::Vertical => {
                    if occupied_by[brick.from.x][brick.from.y][brick.from.z - 1] == 0 {
                        brick.from.z -= 1;
                        brick.to.z -= 1;
                        num_bricks_dropped += 1;
                        brick_ids_dropped.insert(brick.id);
                        occupied_by[brick.from.x][brick.from.y][brick.from.z] = brick.id;
                        occupied_by[brick.from.x][brick.from.y][brick.to.z + 1] = 0;
                    }
                }
                BrickType::HorizontalAlongX => {
                    let mut can_drop_by = 1;
                    for x in brick.from.x..=brick.to.x {
                        if occupied_by[x][brick.from.y][brick.from.z - 1] != 0 {
                            can_drop_by = 0;
                            break;
                        }
                    }
                    if can_drop_by == 1 {
                        brick.from.z -= 1;
                        brick.to.z -= 1;
                        num_bricks_dropped += 1;
                        brick_ids_dropped.insert(brick.id);
                        for x in brick.from.x..=brick.to.x {
                            occupied_by[x][brick.from.y][brick.from.z] = brick.id;
                            occupied_by[x][brick.from.y][brick.to.z + 1] = 0;
                        }
                    }
                }
                BrickType::HorizontalAlongY => {
                    let mut can_drop_by = 1;
                    for y in brick.from.y..=brick.to.y {
                        if occupied_by[brick.from.x][y][brick.from.z - 1] != 0 {
                            can_drop_by = 0;
                            break;
                        }
                    }
                    if can_drop_by == 1 {
                        brick.from.z -= 1;
                        brick.to.z -= 1;
                        num_bricks_dropped += 1;
                        brick_ids_dropped.insert(brick.id);
                        for y in brick.from.y..=brick.to.y {
                            occupied_by[brick.from.x][y][brick.from.z] = brick.id;
                            occupied_by[brick.from.x][y][brick.to.z + 1] = 0;
                        }
                    }
                }
            }
        }
    }
    brick_ids_dropped.len()
}

fn build_occupied_by(
    bricks: &Vec<Brick>,
    max_x: usize,
    max_y: usize,
    max_z: usize,
) -> Vec<Vec<Vec<usize>>> {
    let mut occupied_by: Vec<Vec<Vec<usize>>> =
        vec![vec![vec![0; max_z + 1]; max_y + 1]; max_x + 1];
    for brick in bricks {
        match brick.brick_type {
            BrickType::Unknown => {
                panic!("    ERROR!");
            }
            BrickType::Vertical => {
                for z in brick.from.z..=brick.to.z {
                    occupied_by[brick.from.x][brick.from.y][z] = brick.id;
                }
            }
            BrickType::HorizontalAlongX => {
                for x in brick.from.x..=brick.to.x {
                    occupied_by[x][brick.from.y][brick.from.z] = brick.id;
                }
            }
            BrickType::HorizontalAlongY => {
                for y in brick.from.y..=brick.to.y {
                    occupied_by[brick.from.x][y][brick.from.z] = brick.id;
                }
            }
        }
    }
    occupied_by
}

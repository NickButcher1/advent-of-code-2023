use std::collections::HashSet;

#[derive(Clone, Debug)]
struct BrickEnd {
    x: usize,
    y: usize,
    z: usize,
}

#[derive(Clone, Debug)]
struct Brick {
    id: usize,
    begin: BrickEnd,
    end: BrickEnd,
    brick_type: BrickType,
}

#[derive(Clone, Debug)]
enum BrickType {
    Unknown,
    Single,
    Vertical,
    HorizontalAlongX,
    HorizontalAlongY,
}

pub fn solve22(input: &[String]) -> (i128, i128) {
    let mut bricks: Vec<Brick> = vec![];
    let mut max_x: usize = 0;
    let mut max_y: usize = 0;
    let mut max_z: usize = 0;
    let mut next_brick_id = 1;
    for line in input {
        let split_1: Vec<&str> = line.split('~').collect();
        let b1: Vec<&str> = split_1[0].split(',').collect();
        let b2: Vec<&str> = split_1[1].split(',').collect();
        let mut brick = Brick {
            id: next_brick_id,
            begin: BrickEnd {
                x: b1[0].parse::<usize>().unwrap(),
                y: b1[1].parse::<usize>().unwrap(),
                z: b1[2].parse::<usize>().unwrap(),
            },
            end: BrickEnd {
                x: b2[0].parse::<usize>().unwrap(),
                y: b2[1].parse::<usize>().unwrap(),
                z: b2[2].parse::<usize>().unwrap(),
            },
            brick_type: BrickType::Unknown,
        };
        next_brick_id += 1;
        // Check assumption.
        assert!(brick.end.x >= brick.begin.x);
        assert!(brick.end.y >= brick.begin.y);
        assert!(brick.end.z >= brick.begin.z);

        // Check assumption - brick is single cube or straight line.
        assert!(
            (brick.end.x == brick.begin.x && (brick.end.y == brick.begin.y))
                || (brick.end.x == brick.begin.x && (brick.end.z == brick.begin.z))
                || (brick.end.y == brick.begin.y && (brick.end.z == brick.begin.z))
        );
        brick.brick_type = if brick.end.x == brick.begin.x
            && brick.end.y == brick.begin.y
            && brick.end.z == brick.begin.z
        {
            BrickType::Single
        } else if brick.end.x == brick.begin.x && brick.end.y == brick.begin.y {
            BrickType::Vertical
        } else if brick.end.x == brick.begin.x && brick.end.z == brick.begin.z {
            BrickType::HorizontalAlongY
        } else {
            BrickType::HorizontalAlongX
        };

        if brick.end.x > max_x {
            max_x = brick.end.x;
        }
        if brick.end.y > max_y {
            max_y = brick.end.y;
        }
        if brick.end.z > max_z {
            max_z = brick.end.z;
        }
        bricks.push(brick);
    }

    let mut brick_id = build_occupied_brick_id(&bricks, max_x, max_y, max_z);

    try_drop_bricks(&mut bricks, &mut brick_id);

    // Now we have dropped all the bricks, can we disintegrate any.

    // Safe to disintegrate if no brick above it.
    let mut total_num_bricks_dropped = 0;
    let mut num_disintegratable = 0;
    for brick in &bricks {
        let mut temp_bricks: Vec<Brick> = vec![];
        for temp_brick in &bricks {
            let temp_brick2 = temp_brick.clone();
            if temp_brick2.id != brick.id {
                temp_bricks.push(temp_brick2);
            }
        }

        let mut temp_brick_id = build_occupied_brick_id(&temp_bricks, max_x, max_y, max_z);

        let num_bricks_dropped = try_drop_bricks(&mut temp_bricks, &mut temp_brick_id);
        if num_bricks_dropped == 0 {
            num_disintegratable += 1;
        }
        total_num_bricks_dropped += num_bricks_dropped;
    }

    (num_disintegratable, total_num_bricks_dropped as i128)
}

fn try_drop_bricks(bricks: &mut Vec<Brick>, brick_id: &mut Vec<Vec<Vec<usize>>>) -> usize {
    let mut brick_ids_dropped: HashSet<usize> = HashSet::new();
    let mut num_bricks_dropped = usize::MAX;
    while num_bricks_dropped != 0 {
        num_bricks_dropped = 0;
        for brick in &mut *bricks {
            // Can't drop a brick at z=1. It is already on the ground.
            if brick.begin.z != 1 {
                // The brick is either all at its start height across x,y, or vertical column
                match brick.brick_type {
                    BrickType::Unknown => {
                        panic!("    ERROR!");
                    }
                    BrickType::Single => {
                        if brick_id[brick.begin.x][brick.begin.y][brick.begin.z - 1] == 0 {
                            brick.begin.z -= 1;
                            brick.end.z -= 1;
                            num_bricks_dropped += 1;
                            brick_ids_dropped.insert(brick.id);
                            brick_id[brick.begin.x][brick.begin.y][brick.begin.z] = brick.id;
                            brick_id[brick.begin.x][brick.begin.y][brick.end.z + 1] = 0;
                        }
                    }
                    BrickType::Vertical => {
                        if brick_id[brick.begin.x][brick.begin.y][brick.begin.z - 1] == 0 {
                            brick.begin.z -= 1;
                            brick.end.z -= 1;
                            num_bricks_dropped += 1;
                            brick_ids_dropped.insert(brick.id);
                            brick_id[brick.begin.x][brick.begin.y][brick.begin.z] = brick.id;
                            brick_id[brick.begin.x][brick.begin.y][brick.end.z + 1] = 0;
                        }
                    }
                    BrickType::HorizontalAlongX => {
                        let mut can_drop_by = 1;
                        for x in brick.begin.x..=brick.end.x {
                            if brick_id[x][brick.begin.y][brick.begin.z - 1] != 0 {
                                can_drop_by = 0;
                                break;
                            }
                        }
                        if can_drop_by == 1 {
                            brick.begin.z -= 1;
                            brick.end.z -= 1;
                            num_bricks_dropped += 1;
                            brick_ids_dropped.insert(brick.id);
                            for x in brick.begin.x..=brick.end.x {
                                brick_id[x][brick.begin.y][brick.begin.z] = brick.id;
                                brick_id[x][brick.begin.y][brick.end.z + 1] = 0;
                            }
                        }
                    }
                    BrickType::HorizontalAlongY => {
                        let mut can_drop_by = 1;
                        for y in brick.begin.y..=brick.end.y {
                            if brick_id[brick.begin.x][y][brick.begin.z - 1] != 0 {
                                can_drop_by = 0;
                                break;
                            }
                        }
                        if can_drop_by == 1 {
                            brick.begin.z -= 1;
                            brick.end.z -= 1;
                            num_bricks_dropped += 1;
                            brick_ids_dropped.insert(brick.id);
                            for y in brick.begin.y..=brick.end.y {
                                brick_id[brick.begin.x][y][brick.begin.z] = brick.id;
                                brick_id[brick.begin.x][y][brick.end.z + 1] = 0;
                            }
                        }
                    }
                }
            }
        }
    }
    brick_ids_dropped.len()
}

fn build_occupied_brick_id(
    bricks: &Vec<Brick>,
    max_x: usize,
    max_y: usize,
    max_z: usize,
) -> Vec<Vec<Vec<usize>>> {
    let mut brick_id: Vec<Vec<Vec<usize>>> = vec![vec![vec![0; max_z + 1]; max_y + 1]; max_x + 1];
    for brick in bricks {
        match brick.brick_type {
            BrickType::Unknown => {
                panic!("    ERROR!");
            }
            BrickType::Single => {
                brick_id[brick.begin.x][brick.begin.y][brick.begin.z] = brick.id;
            }
            BrickType::Vertical => {
                for z in brick.begin.z..=brick.end.z {
                    brick_id[brick.begin.x][brick.begin.y][z] = brick.id;
                }
            }
            BrickType::HorizontalAlongX => {
                for x in brick.begin.x..=brick.end.x {
                    brick_id[x][brick.begin.y][brick.begin.z] = brick.id;
                }
            }
            BrickType::HorizontalAlongY => {
                for y in brick.begin.y..=brick.end.y {
                    brick_id[brick.begin.x][y][brick.begin.z] = brick.id;
                }
            }
        }
    }
    brick_id
}

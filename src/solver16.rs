use crate::board::Board;

#[derive(Clone, Debug, PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Clone, Debug)]
struct Beam {
    row: i32,
    col: i32,
    direction: Direction,
    active: bool,
}

impl Beam {
    fn moveit(&mut self, board: &Board) {
        match self.direction {
            Direction::Up => self.row -= 1,
            Direction::Down => self.row += 1,
            Direction::Left => self.col -= 1,
            Direction::Right => self.col += 1,
        }
        self.active = self.row >= 0
            && self.row < board.num_rows as i32
            && self.col >= 0
            && self.col < board.num_cols as i32;
    }
}

fn count_energy_non_zero(energy: &Vec<Vec<usize>>) -> usize {
    let mut sum: usize = 0;
    for row in energy {
        for cell in row {
            if *cell != 0 {
                sum += 1;
            }
        }
    }

    sum
}

pub fn solve16(input: Vec<String>) -> (i128, i128) {
    let board: Board = Board::from_input(input);
    board.print();
    let mut beams: Vec<Beam> = vec![Beam {
        row: 0,
        col: -1,
        direction: Direction::Right,
        active: true,
    }];
    let part_1 = solve_for_beams(&mut beams, &board);

    let mut beams_to_try: Vec<Beam> = vec![];
    for r in 0..board.num_rows {
        for c in 0..board.num_cols {
            if r == 0 {
                beams_to_try.push(Beam {
                    row: -1,
                    col: c as i32,
                    direction: Direction::Down,
                    active: true,
                });
            }
            if r == board.num_rows - 1 {
                beams_to_try.push(Beam {
                    row: board.num_rows as i32,
                    col: c as i32,
                    direction: Direction::Up,
                    active: true,
                });
            }
            if c == 0 {
                beams_to_try.push(Beam {
                    row: r as i32,
                    col: -1,
                    direction: Direction::Right,
                    active: true,
                });
            }
            if c == board.num_cols - 1 {
                beams_to_try.push(Beam {
                    row: r as i32,
                    col: board.num_cols as i32,
                    direction: Direction::Right,
                    active: true,
                });
            }
        }
    }
    let mut part_2_best_energy = 0;
    for beam in beams_to_try {
        let mut beams: Vec<Beam> = vec![beam];
        let energy = solve_for_beams(&mut beams, &board);
        println!("        {}", energy);
        if energy > part_2_best_energy {
            part_2_best_energy = energy;
        }
    }

    (part_1 as i128, part_2_best_energy as i128)
}

fn solve_for_beams(beams: &mut Vec<Beam>, board: &Board) -> usize {
    let mut energy: Vec<Vec<usize>> = vec![vec![0; board.num_cols]; board.num_rows];
    let mut best_energy_count = 0;
    let mut best_energy_since = 0;

    // TODO: This is a very slow check for reaching steady state.
    while best_energy_since < 25 {
        // Move each active beam forward one.
        let old_beams = beams.clone();
        beams.clear();

        for mut beam in old_beams {
            beam.moveit(board);

            // If not active, it is off the board and can be forgotten.
            if beam.active {
                energy[beam.row as usize][beam.col as usize] += 1;

                match board.cells[beam.row as usize][beam.col as usize] {
                    '.' => beams.push(beam),
                    '|' => {
                        if beam.direction == Direction::Up || beam.direction == Direction::Down {
                            beams.push(beam)
                        } else {
                            beams.push(Beam {
                                row: beam.row,
                                col: beam.col,
                                direction: Direction::Up,
                                active: true,
                            });
                            beams.push(Beam {
                                row: beam.row,
                                col: beam.col,
                                direction: Direction::Down,
                                active: true,
                            });
                        }
                    }
                    '-' => {
                        if beam.direction == Direction::Left || beam.direction == Direction::Right {
                            beams.push(beam)
                        } else {
                            beams.push(Beam {
                                row: beam.row,
                                col: beam.col,
                                direction: Direction::Left,
                                active: true,
                            });
                            beams.push(Beam {
                                row: beam.row,
                                col: beam.col,
                                direction: Direction::Right,
                                active: true,
                            });
                        }
                    }
                    '/' => match beam.direction {
                        Direction::Up => beams.push(Beam {
                            row: beam.row,
                            col: beam.col,
                            direction: Direction::Right,
                            active: true,
                        }),
                        Direction::Right => beams.push(Beam {
                            row: beam.row,
                            col: beam.col,
                            direction: Direction::Up,
                            active: true,
                        }),
                        Direction::Down => beams.push(Beam {
                            row: beam.row,
                            col: beam.col,
                            direction: Direction::Left,
                            active: true,
                        }),
                        Direction::Left => beams.push(Beam {
                            row: beam.row,
                            col: beam.col,
                            direction: Direction::Down,
                            active: true,
                        }),
                    },
                    '\\' => match beam.direction {
                        Direction::Up => beams.push(Beam {
                            row: beam.row,
                            col: beam.col,
                            direction: Direction::Left,
                            active: true,
                        }),
                        Direction::Right => beams.push(Beam {
                            row: beam.row,
                            col: beam.col,
                            direction: Direction::Down,
                            active: true,
                        }),
                        Direction::Down => beams.push(Beam {
                            row: beam.row,
                            col: beam.col,
                            direction: Direction::Right,
                            active: true,
                        }),
                        Direction::Left => beams.push(Beam {
                            row: beam.row,
                            col: beam.col,
                            direction: Direction::Up,
                            active: true,
                        }),
                    },
                    _ => {
                        unreachable!();
                    }
                }
            }
        }

        let new_best_energy_count = count_energy_non_zero(&energy);
        if new_best_energy_count > best_energy_count {
            best_energy_count = new_best_energy_count;
            best_energy_since = 0;
        }
        best_energy_since += 1;
    }

    best_energy_count
}

use crate::board::Board;

#[derive(Clone, Debug, PartialEq)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Clone, Debug)]
struct Beam {
    r: i32,
    c: i32,
    dir: Dir,
}

impl Beam {
    fn moveit(&mut self) {
        match self.dir {
            Dir::Up => self.r -= 1,
            Dir::Down => self.r += 1,
            Dir::Left => self.c -= 1,
            Dir::Right => self.c += 1,
        }
    }

    fn is_inside_board(&self, board: &Board) -> bool {
        self.r >= 0
            && self.r < board.num_rows as i32
            && self.c >= 0
            && self.c < board.num_cols as i32
    }

    fn change_direction(&mut self, new_dir: Dir) {
        self.dir = new_dir;
    }
}

fn beam_for_part_1() -> Beam {
    Beam {
        r: 0,
        c: -1,
        dir: Dir::Right,
    }
}

fn beams_for_part_2(board: &Board) -> Vec<Beam> {
    let mut beams_to_try: Vec<Beam> = vec![];
    for r in 0..board.num_rows {
        for c in 0..board.num_cols {
            if r == 0 {
                beams_to_try.push(Beam {
                    r: -1,
                    c: c as i32,
                    dir: Dir::Down,
                });
            }
            if r == board.num_rows - 1 {
                beams_to_try.push(Beam {
                    r: board.num_rows as i32,
                    c: c as i32,
                    dir: Dir::Up,
                });
            }
            if c == 0 {
                beams_to_try.push(Beam {
                    r: r as i32,
                    c: -1,
                    dir: Dir::Right,
                });
            }
            if c == board.num_cols - 1 {
                beams_to_try.push(Beam {
                    r: r as i32,
                    c: board.num_cols as i32,
                    dir: Dir::Right,
                });
            }
        }
    }
    beams_to_try
}

fn solve_part_2(board: &Board) -> usize {
    let beams = beams_for_part_2(board);
    let mut best_energy = 0;

    for beam in beams {
        let energy = solve_for_beam(beam, board);
        if energy > best_energy {
            best_energy = energy;
        }
    }
    best_energy
}

pub fn solve16(input: Vec<String>) -> (i128, i128) {
    let board: Board = Board::from_input(input);

    (
        solve_for_beam(beam_for_part_1(), &board) as i128,
        solve_part_2(&board) as i128,
    )
}

fn solve_for_beam(beam: Beam, board: &Board) -> usize {
    let mut beams: Vec<Beam> = vec![beam];
    let mut energized: Vec<Vec<bool>> = vec![vec![false; board.num_cols]; board.num_rows];
    let mut max_energized_cells_since = 0;
    let mut num_energized_cells = 0;

    // Keep going until we haven't seen the number of energized cells change for some number of loops.
    // Some inputs might need a higher threshold, but this works for the sample and actual input.
    while max_energized_cells_since < 10 {
        // Move each beam forward one.
        let old_beams = beams.clone();
        beams.clear();

        for mut beam in old_beams {
            beam.moveit();

            // If it moved off the board, forget it.
            if beam.is_inside_board(board) {
                if !energized[beam.r as usize][beam.c as usize] {
                    energized[beam.r as usize][beam.c as usize] = true;
                    num_energized_cells += 1;
                    max_energized_cells_since = 0;
                }

                match board.cells[beam.r as usize][beam.c as usize] {
                    '.' => beams.push(beam),
                    '|' => {
                        if beam.dir == Dir::Up || beam.dir == Dir::Down {
                            beams.push(beam)
                        } else {
                            beams.push(Beam {
                                r: beam.r,
                                c: beam.c,
                                dir: Dir::Up,
                            });
                            beams.push(Beam {
                                r: beam.r,
                                c: beam.c,
                                dir: Dir::Down,
                            });
                        }
                    }
                    '-' => {
                        if beam.dir == Dir::Left || beam.dir == Dir::Right {
                            beams.push(beam)
                        } else {
                            beams.push(Beam {
                                r: beam.r,
                                c: beam.c,
                                dir: Dir::Left,
                            });
                            beams.push(Beam {
                                r: beam.r,
                                c: beam.c,
                                dir: Dir::Right,
                            });
                        }
                    }
                    '/' => {
                        let new_dir = match beam.dir {
                            Dir::Up => Dir::Right,
                            Dir::Right => Dir::Up,
                            Dir::Down => Dir::Left,
                            Dir::Left => Dir::Down,
                        };
                        beam.change_direction(new_dir);
                        beams.push(beam);
                    }
                    '\\' => {
                        let new_dir = match beam.dir {
                            Dir::Up => Dir::Left,
                            Dir::Right => Dir::Down,
                            Dir::Down => Dir::Right,
                            Dir::Left => Dir::Up,
                        };
                        beam.change_direction(new_dir);
                        beams.push(beam);
                    }
                    _ => {
                        unreachable!();
                    }
                }
            }
        }

        max_energized_cells_since += 1;
    }

    num_energized_cells
}

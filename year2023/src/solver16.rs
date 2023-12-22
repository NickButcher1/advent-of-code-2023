use aoc::board::Board;
use aoc::dir::Dir;
use std::collections::HashSet;

#[derive(Clone, Eq, Debug, Hash, PartialEq)]
struct Beam {
    r: i32,
    c: i32,
    dir: Dir,
}

impl Beam {
    const fn from(r: i32, c: i32, dir: Dir) -> Self {
        Self { r, c, dir }
    }

    fn moveit(&mut self) {
        match self.dir {
            Dir::Up => self.r -= 1,
            Dir::Down => self.r += 1,
            Dir::Left => self.c -= 1,
            Dir::Right => self.c += 1,
        }
    }

    const fn is_inside_board(&self, board: &Board) -> bool {
        self.r >= 0
            && self.r < board.num_rows as i32
            && self.c >= 0
            && self.c < board.num_cols as i32
    }

    fn change_direction(&mut self, new_dir: Dir) {
        self.dir = new_dir;
    }
}

const fn beam_for_part_1() -> Beam {
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
                beams_to_try.push(Beam::from(-1, c as i32, Dir::Down));
            }
            if r == board.num_rows - 1 {
                beams_to_try.push(Beam::from(board.num_rows as i32, c as i32, Dir::Up));
            }
            if c == 0 {
                beams_to_try.push(Beam::from(r as i32, -1, Dir::Right));
            }
            if c == board.num_cols - 1 {
                beams_to_try.push(Beam::from(r as i32, board.num_cols as i32, Dir::Right));
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

pub fn solve16(input: &[String]) -> (i128, i128) {
    let board: Board = Board::from_input(input);

    (
        solve_for_beam(beam_for_part_1(), &board) as i128,
        solve_part_2(&board) as i128,
    )
}

fn solve_for_beam(beam: Beam, board: &Board) -> usize {
    let mut beams: Vec<Beam> = vec![beam];
    let mut energized: Vec<Vec<bool>> = vec![vec![false; board.num_cols]; board.num_rows];
    let mut num_energized_cells = 0;
    let mut seen_beams: HashSet<Beam> = HashSet::new();

    // Keep going until we haven't seen the number of energized cells change for some number of loops.
    // Some inputs might need a higher threshold, but this works for the sample and actual input.
    // while max_energized_cells_since < 10 {
    while !beams.is_empty() {
        let mut beam = beams.remove(0);
        beam.moveit();

        // No need to process a beam in a given cell in a given direction more than once.
        if seen_beams.contains(&beam) {
            continue;
        }
        seen_beams.insert(beam.clone());

        // If it moved off the board, forget it.
        if beam.is_inside_board(board) {
            if !energized[beam.r as usize][beam.c as usize] {
                energized[beam.r as usize][beam.c as usize] = true;
                num_energized_cells += 1;
            }

            match board.cells[beam.r as usize][beam.c as usize] {
                '.' => beams.push(beam),
                '|' => {
                    if beam.dir.is_vertical() {
                        beams.push(beam);
                    } else {
                        beams.push(Beam::from(beam.r, beam.c, Dir::Up));
                        beams.push(Beam::from(beam.r, beam.c, Dir::Down));
                    }
                }
                '-' => {
                    if beam.dir.is_horizontal() {
                        beams.push(beam);
                    } else {
                        beams.push(Beam::from(beam.r, beam.c, Dir::Left));
                        beams.push(Beam::from(beam.r, beam.c, Dir::Right));
                    }
                }
                '/' => {
                    beam.change_direction(beam.dir.reflect_forward_slash());
                    beams.push(beam);
                }
                '\\' => {
                    beam.change_direction(beam.dir.reflect_back_slash());
                    beams.push(beam);
                }
                _ => {
                    unreachable!();
                }
            }
        }
    }

    num_energized_cells
}

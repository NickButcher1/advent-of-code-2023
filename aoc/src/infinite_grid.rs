use crate::dir::Dir;
use std::collections::HashSet;

type Cell = (i64, i64);

// Represents an infinite 2 dimensional grid, with a person at a specific location, and facing in a
// specific direction.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct InfiniteGrid {
    pub facing: Dir,
    pub pos: Cell,
}

impl InfiniteGrid {
    // Make a series of moves. Each move is of the form "DirDistance", such as "R4" which means turn
    // right 90 degrees then move four cells in the new direction.
    pub fn make_moves(&mut self, moves: Vec<&str>) {
        moves.iter().for_each(|mv| {
            let turn_dir = Dir::from_letter_str(&mv[..1]);
            self.facing = self.facing.turn(&turn_dir);
            let distance = mv[1..].parse::<i64>().unwrap();
            self.make_move(distance)
        });
    }

    pub fn make_moves_stop_when_any_cell_visited_twice(&mut self, moves: Vec<&str>) -> Cell {
        let mut visited_cells: HashSet<Cell> = HashSet::from([self.pos]);

        for mv in moves.iter() {
            let turn_dir = Dir::from_letter_str(&mv[..1]);
            self.facing = self.facing.turn(&turn_dir);
            let distance = mv[1..].parse::<i64>().unwrap();

            for _ in 0..distance {
                self.make_move(1);
                if visited_cells.contains(&self.pos) {
                    return self.pos;
                }
                visited_cells.insert(self.pos);
            }
        }

        unreachable!();
    }

    pub fn make_move(&mut self, distance: i64) {
        let offset = self.facing.offset();
        if offset.0 != 0 {
            self.pos = (self.pos.0 + distance * offset.0, self.pos.1);
        } else {
            self.pos = (self.pos.0, self.pos.1 + distance * offset.1);
        };
    }

    pub fn taxicab_distance(&self, other_cell: &Cell) -> u64 {
        (other_cell.0).abs_diff(self.pos.0) + (other_cell.1).abs_diff(self.pos.1)
    }
}

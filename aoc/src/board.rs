// Represents a mutable square or rectangular board of cells. Each cell is a character.

use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub type Cells = Vec<Vec<char>>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Board {
    pub cells: Cells,
    pub num_rows: usize,
    pub num_cols: usize,
}

#[allow(dead_code)]
impl Board {
    pub fn from_input(input: &[String]) -> Self {
        let cells: Cells = input
            .iter()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect();

        let num_rows = cells.len();
        let num_cols = cells[0].len();

        Self {
            cells,
            num_rows,
            num_cols,
        }
    }

    // Input is multiple boards separated by blank lines.
    pub fn from_input_multiple(input: &[String]) -> Vec<Self> {
        let slices: Vec<_> = input.split(std::string::String::is_empty).collect();
        slices.iter().map(|slice| Self::from_input(slice)).collect()
    }

    pub fn create_empty(num_rows: usize, num_cols: usize, empty_char: char) -> Self {
        Self {
            cells: vec![vec![empty_char; num_cols]; num_rows],
            num_rows,
            num_cols,
        }
    }

    pub fn rotate_clockwise(&mut self) {
        assert_eq!(self.num_rows, self.num_cols);

        let old_board = self.clone();

        for r in 0..self.num_rows {
            for c in 0..self.num_cols {
                self.cells[c][old_board.num_rows - 1 - r] = old_board.cells[r][c];
            }
        }
    }

    // Flip the board on the diagonal axis.
    pub fn flip(&mut self) -> &mut Self {
        let mut new_cells: Cells = vec![];

        for c in 0..self.num_cols {
            let mut row_vec: Vec<char> = vec![];
            for r in 0..self.num_rows {
                row_vec.push(self.cells[r][c]);
            }
            new_cells.push(row_vec);
        }

        (self.num_rows, self.num_cols) = (self.num_cols, self.num_rows);
        self.cells = new_cells;

        self
    }

    pub fn replace(&mut self, find_char: char, replace_char: char) {
        for r in 0..self.num_rows {
            for c in 0..self.num_cols {
                if self.cells[r][c] == find_char {
                    self.cells[r][c] = replace_char;
                }
            }
        }
    }

    pub fn find_and_replace(&mut self, find_char: char, replace_char: char) -> (usize, usize) {
        for r in 0..self.num_rows {
            for c in 0..self.num_cols {
                if self.cells[r][c] == find_char {
                    self.cells[r][c] = replace_char;
                    return (r, c);
                }
            }
        }
        unreachable!();
    }

    // Useful for avoiding edge checks when a board doesn't wrap.
    pub fn add_border(&mut self, border_char: char) {
        let new_cols = self.num_cols + 2;

        let mut new_cells: Cells = vec![];
        let mut top_or_bottom_row: Vec<char> = vec![];
        for _ in 0..new_cols {
            top_or_bottom_row.push(border_char);
        }
        new_cells.push(top_or_bottom_row.clone());
        for r in 0..self.num_rows {
            let mut row = vec![border_char];
            row.extend(&self.cells[r]);
            row.push(border_char);
            new_cells.push(row);
        }
        new_cells.push(top_or_bottom_row);

        self.num_rows += 2;
        self.num_cols += 2;
        self.cells = new_cells;
    }

    pub fn count(&mut self, count_char: char) -> u64 {
        let mut num_matches = 0;

        for r in 0..self.num_rows {
            for c in 0..self.num_cols {
                if self.cells[r][c] == count_char {
                    num_matches += 1;
                }
            }
        }
        num_matches
    }

    pub fn neighbour_cells(&self, r: usize, c: usize) -> [char; 8] {
        [
            self.cells[r - 1][c - 1],
            self.cells[r - 1][c],
            self.cells[r - 1][c + 1],
            self.cells[r][c - 1],
            self.cells[r][c + 1],
            self.cells[r + 1][c - 1],
            self.cells[r + 1][c],
            self.cells[r + 1][c + 1],
        ]
    }

    // Assumes the board has a border of empty cells, which are never modified.
    pub fn game_of_life_step(&mut self, off_char: char, on_char: char) {
        let mut new_cells: Cells = vec![];

        for r in 0..=self.num_rows - 1 {
            let mut row_vec: Vec<char> = vec![];
            for c in 0..=self.num_cols - 1 {
                if r == 0 || r == self.num_rows - 1 || c == 0 || c == self.num_cols - 1 {
                    row_vec.push(off_char);
                } else {
                    let num_neighbours_on = self
                        .neighbour_cells(r, c)
                        .iter()
                        .filter(|&&c| c == on_char)
                        .count();
                    let new_char = if self.cells[r][c] == on_char {
                        if (2..=3).contains(&num_neighbours_on) {
                            on_char
                        } else {
                            off_char
                        }
                    } else if num_neighbours_on == 3 {
                        on_char
                    } else {
                        off_char
                    };
                    row_vec.push(new_char);
                }
            }
            new_cells.push(row_vec);
        }

        self.cells = new_cells;
    }

    pub fn set_corners_to(&mut self, set_char: char, has_border: bool) {
        let offset = if has_border { 1 } else { 0 };
        self.cells[offset][offset] = set_char;
        self.cells[self.num_rows - 1 - offset][offset] = set_char;
        self.cells[offset][self.num_cols - 1 - offset] = set_char;
        self.cells[self.num_rows - 1 - offset][self.num_cols - 1 - offset] = set_char;
    }

    pub fn hash(&self) -> Vec<u8> {
        md5::compute(format!("{self:?}")).to_ascii_lowercase()
    }

    #[allow(dead_code)]
    pub fn print(&self) {
        println!("BOARD {},{}", self.num_rows, self.num_cols);
        for row in &self.cells {
            println!("{}", row.iter().collect::<String>());
        }
    }

    // Find the cheapest path from top left to bottom right. The cost for entering a cell is the
    // value in that cell. There is no cost for the top left cell unless re-entered.
    // Assumes cells contain a digit 1-9 as a char.
    pub fn cheapest_path(&self) -> u32 {
        // Initialize minimum cost for every cell - all to +infinity except for the top left cell
        // which is defined to have zero cost. We'll subtract that at the end.
        let mut min_cost = vec![vec![u32::MAX; self.num_cols]; self.num_rows];
        min_cost[0][0] = self.cells[0][0].to_digit(10).unwrap();

        // Keep a list of cells we have visited. Reverse is used to prioritise cheapest cells.
        let mut heap = BinaryHeap::new();
        heap.push(Reverse((self.cells[0][0].to_digit(10).unwrap(), 0, 0)));

        let directions: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

        // Loop, taking the cheapest item off the heap each time.
        while let Some(Reverse((current_cost, x, y))) = heap.pop() {
            // Ignore if we already found a cheaper path to this cell.
            if current_cost > min_cost[x][y] {
                continue;
            }

            // Try the adjacent cells in all four directions.
            for (dx, dy) in directions.iter() {
                let new_x_isize = x as isize + dx;
                let new_y_isize = y as isize + dy;

                if new_x_isize >= 0 && new_y_isize >= 0 {
                    let new_x = new_x_isize as usize;
                    let new_y = new_y_isize as usize;

                    if new_x < self.num_cols && new_y < self.num_rows {
                        let new_cost =
                            current_cost + self.cells[new_x][new_y].to_digit(10).unwrap();

                        if new_cost < min_cost[new_x][new_y] {
                            min_cost[new_x][new_y] = new_cost;
                            heap.push(Reverse((new_cost, new_x, new_y)));
                        }
                    }
                }
            }
        }

        // Subtract the cost of the starting cell.
        let cheapest_path_cost =
            min_cost[self.num_cols - 1][self.num_rows - 1] - self.cells[0][0].to_digit(10).unwrap();
        cheapest_path_cost
    }
}

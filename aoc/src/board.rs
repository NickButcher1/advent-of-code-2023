// Represents a mutable square or rectangular board of cells. Each cell is a character.

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
}

// Represents a mutable square or rectangular board of cells. Each cell is a character.

type Cells = Vec<Vec<char>>;
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

    #[allow(dead_code)]
    pub fn print(&self) {
        for row in &self.cells {
            println!("{}", row.iter().collect::<String>());
        }
    }
}

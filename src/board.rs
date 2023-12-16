// Represents a mutable square or rectangular board of cells. Each cell is a character.

type Cells = Vec<Vec<char>>;
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Board {
    pub cells: Cells,
    pub num_rows: usize,
    pub num_cols: usize,
}

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

    pub(crate) fn rotate_clockwise(&mut self) {
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

    #[allow(dead_code)]
    pub fn print(&self) {
        for row in &self.cells {
            println!("{}", row.iter().collect::<String>());
        }
    }
}

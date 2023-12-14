// Represents a mutable square or rectangular board of cells. Each cell is a character.

#[derive(Clone, Debug, PartialEq)]
pub struct Board {
    pub cells: Vec<Vec<char>>,
    pub num_rows: usize,
    pub num_cols: usize,
}

impl Board {
    pub fn from_input(input: Vec<String>) -> Board {
        let cells: Vec<Vec<char>> = input
            .iter()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect();

        let num_rows = cells.len();
        let num_cols = cells[0].len();

        Board {
            cells,
            num_rows,
            num_cols,
        }
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
}

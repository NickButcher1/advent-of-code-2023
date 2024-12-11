// Represents a mutable square or rectangular board of cells. Each cell is an integer.

use itertools::iproduct;

pub type Cells = Vec<Vec<i32>>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IntBoard {
    pub cells: Cells,
    pub num_rows: usize,
    pub num_cols: usize,
}

#[allow(dead_code)]
impl IntBoard {
    pub fn from_input(input: &[String]) -> Self {
        let cells: Cells = input
            .iter()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap() as i32)
                    .collect::<Vec<i32>>()
            })
            .collect();

        let num_rows = cells.len();
        let num_cols = cells[0].len();

        Self {
            cells,
            num_rows,
            num_cols,
        }
    }

    // Useful for avoiding edge checks when a board doesn't wrap.
    pub fn add_border(&mut self, border_value: i32) {
        let new_cols = self.num_cols + 2;

        let mut new_cells: Cells = vec![];
        let mut top_or_bottom_row: Vec<i32> = vec![];
        for _ in 0..new_cols {
            top_or_bottom_row.push(border_value);
        }
        new_cells.push(top_or_bottom_row.clone());
        for r in 0..self.num_rows {
            let mut row = vec![border_value];
            row.extend(&self.cells[r]);
            row.push(border_value);
            new_cells.push(row);
        }
        new_cells.push(top_or_bottom_row);

        self.num_rows += 2;
        self.num_cols += 2;
        self.cells = new_cells;
    }

    pub fn count(&mut self, count_value: i32) -> u64 {
        let mut num_matches = 0;

        for (c, r) in iproduct!(0..self.num_cols, 0..self.num_rows) {
            if self.cells[r][c] == count_value {
                num_matches += 1;
            }
        }
        num_matches
    }

    #[allow(dead_code)]
    pub fn print(&self) {
        println!("BOARD {},{}", self.num_rows, self.num_cols);
        for row in &self.cells {
            println!("{row:?}");
        }
    }

    #[allow(dead_code)]
    pub fn increment_all(&mut self) {
        for (c, r) in iproduct!(0..self.num_cols, 0..self.num_rows) {
            self.cells[r][c] += 1;
        }
    }
}

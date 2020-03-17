use std::fmt;
use std::collections::HashSet;
use arr_macro::arr;

// A single cell
pub struct Cell {
    digit: Option<u8>
}

impl Cell {
    fn only_possibility(&self) -> HashSet<u8>{
        //[self.digit.unwrap()].iter().cloned().collect()
        HashSet::new()
    }

    fn to_int(&self) -> u8 {
        match self.digit {
            Some(value) => value,
            None => 0
        }
    }
}

// A full Sudoku grid of 9x9
pub struct Sudoku {
    cells: [Cell; 81],
    posibilities: [HashSet<u8>; 81]
}

impl Sudoku {

    fn index(row_number: usize, column_number: usize) -> usize {
        (row_number * 9) + column_number
    }

    fn row_indexes(&self, row_number: usize) -> HashSet<usize> {
        (0..9).map(|column_number| Sudoku::index(row_number, column_number))
            .collect()
    }

    fn column_indexes(&self, column_number: usize) -> HashSet<usize> {
        (0..9).map(|row_number| Sudoku::index(row_number, column_number))
            .collect()
    }

    fn square_indexes(&self, square_number: usize) -> HashSet<usize> {
        let column_number = (square_number % 3) * 3;
        let row_number = (square_number / 3) * 3;
        (0..9).map(|i| Sudoku::index(row_number + (i / 3), column_number + (i % 3)))
            .collect()
    }

    fn taken_digits(&self, indexes: &HashSet<usize>) -> HashSet<u8> {
        indexes.iter().cloned()
            .map(|cell_index| self.cells[cell_index].to_int())
            .collect()
    }

    fn restrict_indexes(&mut self, cell_indexes: HashSet<usize>) {
        let taken_digits = self.taken_digits(&cell_indexes);

        for cell_index in cell_indexes {
            for digit in &taken_digits {
                self.posibilities[cell_index].remove(&digit);
            }
        }
    }

    fn restrict(&mut self) {
        // Iterate over rows and restrict all rows based on given digits
        for row_number in 0..9 {
            self.restrict_indexes(self.row_indexes(row_number));
        }

        // Iterate over columns and restrict all rows based on given digits
        for column_number in 0..9 {
            self.restrict_indexes(self.column_indexes(column_number));
        }

        // Iterate over squares and restrict all rows based on given digits
        for square_number in 0..9 {
            self.restrict_indexes(self.square_indexes(square_number));
        }

        // Find non empty cells
        let active_cells = self.cells.iter()
            .enumerate()
            .filter(|(_, cell)| cell.digit.is_some());

        // Set only posibility for any non empty cells
        for (cell_index, cell) in active_cells {
            self.posibilities[cell_index] = cell.only_possibility();
        }
    }

    pub fn debug_posibilities(&self) {
        for (cell_index, posibilities) in self.posibilities.iter().enumerate() {
            println!("{:?}", posibilities);
            if cell_index % 9 == 8 {
                println!("\n");
            }
        }
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.digit.is_none() {
            write!(f, "·")?;
        } else {
            write!(f, "{}", self.digit.unwrap())?;
        }
        Ok(())
    }
}

impl fmt::Display for Sudoku {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Cannot use the Square->fmt as this introduces newlines
        for (cell_index, cell) in self.cells.iter().enumerate() {

            // A horizontal line above each group of 3 rows of 9 cells (3x9=27)
            if cell_index % 27 == 0 {
                write!(f, "+-------+-------+-------+\n")?;
            }

            // A vertical line before each group of 3 columns
            if cell_index % 3 == 0 {
                write!(f, "| ")?;
            }

            write!(f, "{} ", cell)?;

            // A closing vertical line at the end of a row of 9 cells
            if cell_index % 9 == 8 {
                write!(f, "|\n")?;
            }

            // A closing horizontal line at the end of all cells
            if cell_index == 80 {
                write!(f, "+-------+-------+-------+")?;
            }
        }

        Ok(())
    }
}

pub fn build_cell(input: &str) -> Cell {
    let result = input.parse::<u8>();
    match result {
        Ok(value) => Cell {digit: Some(value)},
        _ => Cell {digit: None}
    }
}

fn build_posibilities() -> [HashSet<u8>; 81] {
    arr![(1..=9).collect(); 81]
}

pub fn build_sudoku(cells: [Cell; 81]) -> Sudoku {
    let mut sudoku = Sudoku {
        cells: cells,
        posibilities: build_posibilities()
    };
    sudoku.restrict();
    return sudoku
}

#[cfg(test)]
mod tests {
    #[test]
    fn truth() {
        assert!(true);
    }
}

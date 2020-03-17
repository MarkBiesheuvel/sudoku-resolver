use std::fmt;
use std::collections::HashSet;

// A single cell
pub struct Cell {
    digit: Option<u8>,
    posibilities: HashSet<u8>
}

impl Cell {

    pub fn new(character: char) -> Cell {
        match character.to_digit(10) {
            Some(value) => Cell {
                digit: Some(value as u8),
                posibilities: HashSet::new()
            },
            _ => Cell {
                digit: None,
                posibilities: (1..=9).collect()
            }
        }
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
    cells: [Cell; 81]
}

impl Sudoku {

    pub fn new(cells: [Cell; 81]) -> Sudoku {
        let mut sudoku = Sudoku {
            cells: cells,
        };
        sudoku.restrict();
        return sudoku
    }

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
                self.cells[cell_index].posibilities.remove(&digit);
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
    }

    pub fn debug_posibilities(&self) {
        for cell in self.cells.iter() {
            println!("{:?}", cell.posibilities);
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

#[cfg(test)]
mod tests {
    #[test]
    fn truth() {
        assert!(true);
    }
}

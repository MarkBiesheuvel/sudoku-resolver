use std::fmt;
use arraytools::ArrayTools;
use std::collections::HashSet;

// A single cell
#[derive(Debug, Clone, Copy)]
pub struct Cell {
    pub digit: Option<u8>
}

// A group of 9 cells forming a row, column or square
#[derive(Debug)]
pub struct Group {
    pub cells: [Cell; 9]
}

impl Group {
    pub fn is_valid(&self) -> bool {
        // Try inserting digits into a set
        // When there is a duplicate HashSet->insert will return false
        let mut set: HashSet<u8> = HashSet::new();
        self.cells.iter()
            .filter(|cell| cell.digit.is_some())
            .map(|cell| cell.digit.unwrap())
            .all(|digit| set.insert(digit))
    }
}

// A full Sudoku grid of 9x9
#[derive(Debug)]
pub struct Sudoku {
    pub cells: [[Cell; 9]; 9]
}

impl Sudoku {
    pub fn row(&self, number: usize) -> Group {
        Group {
            cells: self.cells[number].clone()
        }
    }

    pub fn column(&self, number: usize) -> Group {
        Group {
            cells: self.cells.map(|row| row[number])
        }
    }

    pub fn square(&self, number: usize) -> Group {
        let column_index: usize = (number % 3) * 3;
        let row_index: usize = (number / 3) * 3;
        Group {
            cells: [
                self.cells[row_index][column_index],
                self.cells[row_index][column_index + 1],
                self.cells[row_index][column_index + 2],
                self.cells[row_index + 1][column_index],
                self.cells[row_index + 1][column_index + 1],
                self.cells[row_index + 1][column_index + 2],
                self.cells[row_index + 2][column_index],
                self.cells[row_index + 2][column_index + 1],
                self.cells[row_index + 2][column_index + 2]
            ]
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

impl fmt::Display for Group {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for cell in self.cells.iter() {
            write!(f, "{}", cell)?;
        }
        Ok(())
    }
}

impl fmt::Display for Sudoku {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Cannot use the Square->fmt as this introduces newlines
        for (row_index, row) in self.cells.iter().enumerate() {
            if row_index % 3 == 0 {
                write!(f, "+-------+-------+-------+\n")?;
            }
            for (column_index, cell) in row.iter().enumerate() {
                if column_index % 3 == 0 {
                    write!(f, "| ")?;
                }
                write!(f, "{} ", cell)?;
            }
            write!(f, "|\n")?;
        }
        write!(f, "+-------+-------+-------+\n")?;
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

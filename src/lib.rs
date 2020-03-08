use std::fmt;

#[derive(Debug)]
pub struct Cell {
    pub digit: Option<u8>
}

#[derive(Debug)]
pub struct Sudoku {
    pub cells: [[Cell; 9]; 9]
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

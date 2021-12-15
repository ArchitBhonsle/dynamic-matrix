//! Error encountered while indexing into a matrix

use std::fmt;

#[derive(Clone, Debug)]
/// The error type of indexing out of bounds
pub struct IndexingError {
    row: usize,
    col: usize,
    nrows: usize,
    ncols: usize,
}

impl fmt::Display for IndexingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let row_error = if self.row >= self.nrows {
            Some(format!(
                "Attemped indexing row {}. The row index should be in [0, {})",
                self.row, self.nrows
            ))
        } else {
            None
        };

        let col_error = if self.col >= self.ncols {
            Some(format!(
                "Attemped indexing column {}. The columns index should be in [0, {})",
                self.col, self.ncols
            ))
        } else {
            None
        };

        match (row_error, col_error) {
            (Some(re), Some(ce)) => writeln!(f, "{}\n{}", re, ce),
            (Some(e), None) | (None, Some(e)) => writeln!(f, "{}", e),
            (None, None) => unreachable!(),
        }
    }
}

impl IndexingError {
    /// Create a new `IndexingError` given the index as a row, column tuple and the shape of the matrix being indexed
    pub fn new(index: (usize, usize), shape: (usize, usize)) -> IndexingError {
        IndexingError {
            row: index.0,
            col: index.1,
            nrows: shape.0,
            ncols: shape.1,
        }
    }
}

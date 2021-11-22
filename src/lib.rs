#![warn(missing_debug_implementations, missing_docs)]

//! A convenient way to work with matrices

mod errors;
use std::vec::Vec;

use errors::ShapeError;

#[derive(Debug)]
/// A contiguous array representing a matrix
pub struct Mat<T> {
    data: Vec<T>,
    rows: usize,
    cols: usize,
}

impl<T> Mat<T> {
    /// Constructs a new Mat<T> with the capacity to represent a matrix with the given shape
    pub fn new(shape: (usize, usize)) -> Self {
        Self {
            data: Vec::with_capacity(shape.0 * shape.1),
            rows: shape.0,
            cols: shape.1,
        }
    }

    /// Extracts a slice containing the underlying Vec<T>
    pub fn as_slice(&self) -> &[T] {
        self.data.as_slice()
    }

    /// Returns the number of rows in the matrix
    pub fn rows(&self) -> usize {
        self.rows
    }

    /// Returns the number of columns in the matrix
    pub fn cols(&self) -> usize {
        self.cols
    }

    /// Returns a tuple containing the number of rows as the first element and number of columns as the second element
    pub fn shape(&self) -> (usize, usize) {
        (self.rows, self.cols)
    }

    /// Appends a new row to the matrix
    pub fn push_row(&mut self, row: Vec<T>) -> Result<(), ShapeError> {
        if row.len() != self.cols {
            Err(ShapeError::new_cols_error(self.cols, row.len()))
        } else {
            self.data.extend(row.into_iter());
            self.rows += 1;
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ROWS: usize = 2;
    const COLS: usize = 3;

    #[test]
    fn test_new() {
        let mat: Mat<isize> = Mat::new((ROWS, COLS));

        assert_eq!(mat.rows(), ROWS);
        assert_eq!(mat.cols(), COLS);
    }

    #[test]
    fn test_push_row() {
        let mut mat: Mat<isize> = Mat::new((ROWS, COLS));

        mat.push_row(vec![1, 2, 3]).unwrap();
        mat.push_row(vec![4, 5, 6]).unwrap();

        assert_eq!(mat.as_slice(), vec![1, 2, 3, 4, 5, 6].as_slice());
    }

    #[test]
    #[should_panic]
    fn test_push_row_fail() {
        let mut mat: Mat<isize> = Mat::new((ROWS, COLS));

        // Trying to push a vector with length 4 into a matrix with only 3 columns
        mat.push_row(vec![1, 2, 3, 4]).unwrap();
    }
}

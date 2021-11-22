use std::vec::Vec;

use crate::errors::ShapeError;

#[derive(Debug)]
/// A row-ordered dynamic matrix. It is cheap to add a new row while adding a new column is expensive
pub struct DynamicMat<T> {
    data: Vec<T>,
    cols: usize,
}

impl<T> DynamicMat<T> {
    /// Constructs a new DynamicMatR<T>.
    pub fn new(cols: usize) -> Self {
        Self {
            data: Vec::new(),
            cols,
        }
    }

    /// Constructs a new DynamicMatR<T> and allocates enough space to accomodate a matrix of the provided shape without
    /// reallocation
    pub fn with_capacity(shape: (usize, usize)) -> Self {
        Self {
            data: Vec::with_capacity(shape.0 * shape.1),
            cols: shape.1,
        }
    }

    /// Extracts a slice containing the underlying Vec<T>
    pub fn as_slice(&self) -> &[T] {
        self.data.as_slice()
    }

    /// Returns the number of rows in the matrix
    pub fn rows(&self) -> usize {
        self.data.len() / self.cols()
    }

    /// Returns the number of columns in the matrix
    pub fn cols(&self) -> usize {
        self.cols
    }

    /// Returns a tuple containing the number of rows as the first element and number of columns as the second element
    pub fn shape(&self) -> (usize, usize) {
        (self.rows(), self.cols())
    }

    /// Appends a new row to the matrix
    pub fn push_row(&mut self, row: Vec<T>) -> Result<(), ShapeError> {
        if row.len() != self.cols {
            Err(ShapeError::new_cols_error(self.cols(), row.len()))
        } else {
            self.data.extend(row.into_iter());
            Ok(())
        }
    }

    /// Appends a new columns to the matrix
    pub fn push_col(&mut self, col: Vec<T>) -> Result<(), ShapeError> {
        if col.len() != self.cols {
            Err(ShapeError::new_rows_error(self.rows(), col.len()))
        } else {
            self.data.extend(col.into_iter());
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ROWS: usize = 3;
    const COLS: usize = 3;

    #[test]
    fn test_new() {
        let mat: DynamicMat<isize> = DynamicMat::new(COLS);

        assert_eq!(mat.rows(), 0);
        assert_eq!(mat.cols(), COLS);
    }

    #[test]
    fn test_with_capacity() {
        let mat: DynamicMat<isize> = DynamicMat::with_capacity((ROWS, COLS));

        assert_eq!(mat.rows(), 0);
        assert_eq!(mat.cols(), COLS);
        assert_eq!(mat.data.capacity(), ROWS * COLS);
    }

    #[test]
    fn test_push_row() {
        let mut mat: DynamicMat<isize> = DynamicMat::new(COLS);

        mat.push_row(vec![1, 2, 3]).unwrap();
        mat.push_row(vec![4, 5, 6]).unwrap();
        mat.push_row(vec![7, 8, 9]).unwrap();

        assert_eq!(mat.as_slice(), vec![1, 2, 3, 4, 5, 6, 7, 8, 9].as_slice()); // checking the elements
        assert_eq!(mat.rows(), ROWS); // checking the number of rows
    }

    #[test]
    #[should_panic]
    fn test_push_row_fail() {
        let mut mat: DynamicMat<isize> = DynamicMat::new(COLS);

        // Trying to push a vector with length 4 into a matrix with only 3 columns
        mat.push_row(vec![1, 2, 3, 4]).unwrap();
    }
}

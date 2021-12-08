use std::vec::Vec;

use crate::errors::ShapeError;

#[derive(Debug)]
/// A row-ordered dynamic matrix. It is cheap to add a new row while adding a new column is expensive
pub struct DynamicMatrix<T> {
    data: Vec<T>,
    cols: usize,
}

impl<T> DynamicMatrix<T> {
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
        if row.len() != self.cols() {
            Err(ShapeError::new_cols_error(self.cols(), row.len()))
        } else {
            self.data.extend(row.into_iter());
            Ok(())
        }
    }

    /// Appends a new columns to the matrix
    pub fn push_col(&mut self, col: Vec<T>) -> Result<(), ShapeError> {
        if col.len() != self.rows() {
            Err(ShapeError::new_rows_error(self.rows(), col.len()))
        } else {
            for (i, e) in col.into_iter().enumerate() {
                self.data.insert(self.cols() + self.cols() * i + i, e);
            }
            self.cols += 1;

            Ok(())
        }
    }

    /// Gives a raw pointer to the underlying Vec's buffer
    pub fn as_ptr(&self) -> *const T {
        self.data.as_ptr()
    }

    /// Gives a raw mutable pointer to the underlying Vec's buffer
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self.data.as_mut_ptr()
    }

    /// Extracts a slice containing the underlying Vec
    pub fn as_slice(&self) -> &[T] {
        self.data.as_slice()
    }

    /// Extracts a mut slice containing the underlying Vec
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        self.data.as_mut_slice()
    }

    /// Decomposes the DynamicMatrix into it's underlying raw components
    /// The returned tuple has three elements: (raw parts of the underlying vector, number of columns)
    // TODO tests
    #[cfg(vec_into_raw_parts)]
    pub fn into_raw_parts(self) -> ((*mut T, usize, usize), usize) {
        let cols = self.cols();

        (self.data.into_raw_parts(), cols)
    }

    /// Creates a DynamicMatrix from it's underlying raw components
    // TODO tests
    pub unsafe fn from_raw_parts(vec_parts: (*mut T, usize, usize), cols: usize) -> Self {
        Self {
            data: Vec::from_raw_parts(vec_parts.0, vec_parts.1, vec_parts.2),
            cols,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ROWS: usize = 3;
    const COLS: usize = 3;

    #[test]
    fn new() {
        let mat: DynamicMatrix<isize> = DynamicMatrix::new(COLS);

        assert_eq!(mat.rows(), 0);
        assert_eq!(mat.cols(), COLS);
    }

    #[test]
    fn with_capacity() {
        let mat: DynamicMatrix<isize> = DynamicMatrix::with_capacity((ROWS, COLS));

        assert_eq!(mat.rows(), 0);
        assert_eq!(mat.cols(), COLS);
        assert_eq!(mat.data.capacity(), ROWS * COLS);
    }

    #[test]
    fn shape_rows_cols() {
        let mut mat: DynamicMatrix<isize> = DynamicMatrix::new(COLS);

        mat.push_row(vec![1, 2, 3]).unwrap();
        mat.push_row(vec![4, 5, 6]).unwrap();
        mat.push_row(vec![7, 8, 9]).unwrap();

        assert_eq!(mat.shape(), (ROWS, COLS));
        assert_eq!(mat.rows(), ROWS);
        assert_eq!(mat.cols(), COLS);
    }

    #[test]
    fn push_row() {
        let mut mat: DynamicMatrix<isize> = DynamicMatrix::new(COLS);

        mat.push_row(vec![1, 2, 3]).unwrap();
        mat.push_row(vec![4, 5, 6]).unwrap();
        mat.push_row(vec![7, 8, 9]).unwrap();

        assert_eq!(mat.as_slice(), vec![1, 2, 3, 4, 5, 6, 7, 8, 9].as_slice()); // checking the elements
        assert_eq!(mat.rows(), ROWS); // checking the number of rows
    }

    #[test]
    #[should_panic]
    fn push_row_fail() {
        let mut mat: DynamicMatrix<isize> = DynamicMatrix::new(COLS);

        // Trying to push a vector with length 4 into a matrix with only 3 columns
        mat.push_row(vec![1, 2, 3, 4]).unwrap();
    }

    #[test]
    fn push_col() {
        let mut mat: DynamicMatrix<isize> = DynamicMatrix::new(COLS - 1);

        // TODO change this to use the macro later
        mat.push_row(vec![1, 2]).unwrap();
        mat.push_row(vec![4, 5]).unwrap();
        mat.push_row(vec![7, 8]).unwrap();

        mat.push_col(vec![3, 6, 9]).unwrap();

        assert_eq!(mat.as_slice(), &[1, 2, 3, 4, 5, 6, 7, 8, 9]); // checking the elements
        assert_eq!(mat.cols(), COLS); // checking the number of rows
    }

    #[test]
    #[should_panic]
    fn push_col_fail() {
        let mut mat: DynamicMatrix<isize> = DynamicMatrix::new(COLS - 1);

        // TODO change this to use the macro later
        mat.push_row(vec![1, 2]).unwrap();
        mat.push_row(vec![4, 5]).unwrap();
        mat.push_row(vec![7, 8]).unwrap();

        // Trying to push a column with less elements than the number of rows
        mat.push_col(vec![3, 6]).unwrap();
    }

    #[test]
    fn as_ptr() {
        let mut mat: DynamicMatrix<isize> = DynamicMatrix::with_capacity((ROWS, COLS));

        mat.push_row(vec![1, 2, 3]).unwrap();
        mat.push_row(vec![4, 5, 6]).unwrap();

        let mat_ptr = mat.as_ptr();
        for i in 0..mat.data.len() {
            assert_eq!(unsafe { *mat_ptr.add(i) }, i as isize + 1);
        }
    }

    #[test]
    fn as_mut_ptr() {
        let mut mat: DynamicMatrix<isize> = DynamicMatrix::with_capacity((ROWS, COLS));

        mat.push_row(vec![1, 2, 3]).unwrap();
        mat.push_row(vec![4, 5, 6]).unwrap();

        let mat_ptr = mat.as_mut_ptr();
        for i in 0..mat.data.len() {
            unsafe {
                *mat_ptr.add(i) = i as isize + 7;
            }
        }

        assert_eq!(mat.as_slice(), &[7, 8, 9, 10, 11, 12]);
    }
}

use std::{
    ops::{Index, IndexMut},
    vec::Vec,
};

use crate::errors::{indexing_error::IndexingError, shape_error::ShapeError};

#[macro_export]
/// A macro to construct a DynamicMatrix
///
/// There are three ways to invoke this macro:
///
/// 1. With a single argument, the number of columns in this DynamicMatrix.
/// ```
/// # use simple_matrices::{dynamic_matrix, dynamic::row_major::{DynamicMatrix}};
///
/// let mat: DynamicMatrix<isize> = dynamic_matrix!(3);
///
/// assert_eq!(mat.shape(), (0, 3));
/// ```
///
/// 2. With a list of arguments followed the number of columns in this DynamicMatrix.
/// ```
/// # use simple_matrices::{dynamic_matrix, dynamic::row_major::{DynamicMatrix}};
///
/// let mat = dynamic_matrix![1, 2, 3, 4, 5, 6, 7, 8, 9; 3];
///
/// assert_eq!(mat.shape(), (3, 3));
/// assert_eq!(mat.as_slice(), [1, 2, 3, 4, 5, 6, 7, 8, 9]);
/// ```
///
/// 3. A "nested array". "," seperating elements at the row level and ";" at the column level.
/// ```
/// # use simple_matrices::{dynamic_matrix, dynamic::row_major::{DynamicMatrix}};
///
/// let mat = dynamic_matrix![1, 2, 3; 4, 5, 6; 7, 8, 9];
///
/// assert_eq!(mat.shape(), (3, 3));
/// assert_eq!(mat.as_slice(), [1, 2, 3, 4, 5, 6, 7, 8, 9]);
/// ```
macro_rules! dynamic_matrix {
    ($cols:expr) => {
        $crate::dynamic::row_major::DynamicMatrix::new_with_cols($cols)
    };
    ($($elem:expr),+; $cols:expr) => (
            $crate::dynamic::row_major::DynamicMatrix::from_boxed_slice(::std::boxed::Box::new([$($elem),+]), $cols)
    );
    ($($($elem:expr),+);+) => (
        $crate::dynamic::row_major::DynamicMatrix::new([$([$($elem),+]),+])
    )
}

#[derive(Debug)]
/// A dynamic matrix in row-major order
/// Adding a new row is cheap while adding a new column is expensive.
pub struct DynamicMatrix<T> {
    data: Vec<T>,
    cols: usize,
}

impl<T> DynamicMatrix<T> {
    /// Constructs a new DynamicMatrix from a nested array
    ///
    /// ```
    /// # use simple_matrices::dynamic::row_major::DynamicMatrix;
    ///
    /// let mat: DynamicMatrix<isize> = DynamicMatrix::new([[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
    ///
    /// assert_eq!(mat.shape(), (3, 3));
    /// assert_eq!(mat.as_slice(), [1, 2, 3, 4, 5, 6, 7, 8, 9]);
    /// ```
    pub fn new<const COLS: usize, const ROWS: usize>(data: [[T; COLS]; ROWS]) -> Self {
        let cols = data[0].len();

        Self {
            data: data.into_iter().flatten().collect(),
            cols,
        }
    }

    /// Constructs a new empty DynamicMatrix with a set number of columns
    ///
    /// ```
    /// # use simple_matrices::dynamic::row_major::DynamicMatrix;
    ///
    /// let mat: DynamicMatrix<isize> = DynamicMatrix::new_with_cols(3);
    ///
    /// assert_eq!(mat.rows(), 0);
    /// assert_eq!(mat.cols(), 3);
    /// ```
    pub fn new_with_cols(cols: usize) -> Self {
        Self {
            data: Vec::new(),
            cols,
        }
    }

    /// Constructs a new DynamicMatrix and allocates enough space to accomodate a matrix of the provided shape without
    /// reallocation
    ///
    /// ```
    /// # use simple_matrices::dynamic::row_major::DynamicMatrix;
    ///
    /// let mat: DynamicMatrix<isize> = DynamicMatrix::with_capacity((3, 3));
    ///
    /// assert_eq!(mat.rows(), 0);
    /// assert_eq!(mat.cols(), 3);
    /// assert_eq!(mat.capacity(), 9);
    /// ```
    pub fn with_capacity(shape: (usize, usize)) -> Self {
        Self {
            data: Vec::with_capacity(shape.0 * shape.1),
            cols: shape.1,
        }
    }

    /// Returns the number of rows in the DynamicMatrix
    ///
    /// ```
    /// # use simple_matrices::{dynamic_matrix, dynamic::row_major::DynamicMatrix};
    ///
    /// let mat = dynamic_matrix![1, 2, 3; 4, 5, 6; 7, 8, 9];
    ///
    /// assert_eq!(mat.rows(), 3);
    /// ```
    pub fn rows(&self) -> usize {
        self.data.len() / self.cols()
    }

    /// Returns the number of columns in the DynamicMatrix
    ///
    /// ```
    /// # use simple_matrices::{dynamic_matrix, dynamic::row_major::DynamicMatrix};
    ///
    /// let mat = dynamic_matrix![1, 2, 3; 4, 5, 6; 7, 8, 9];
    ///
    /// assert_eq!(mat.cols(), 3);
    /// ```
    pub fn cols(&self) -> usize {
        self.cols
    }

    /// Returns a tuple containing the number of rows as the first element and number of columns as the second element
    ///
    /// ```
    /// # use simple_matrices::{dynamic_matrix, dynamic::row_major::DynamicMatrix};
    ///
    /// let mat = dynamic_matrix![1, 2, 3; 4, 5, 6; 7, 8, 9];
    ///
    /// assert_eq!(mat.shape(), (3, 3));
    /// ```
    pub fn shape(&self) -> (usize, usize) {
        (self.rows(), self.cols())
    }

    /// Returns the length of the underlying Vec
    ///
    /// ```
    /// # use simple_matrices::{dynamic_matrix, dynamic::row_major::DynamicMatrix};
    ///
    /// let mat = dynamic_matrix![1, 2, 3; 4, 5, 6; 7, 8, 9];
    ///
    /// assert_eq!(mat.len(), 9);
    pub fn len(&self) -> usize {
        self.data.capacity()
    }

    /// Returns the capacity of the underlying Vec
    ///
    /// ```
    /// # use simple_matrices::dynamic::row_major::DynamicMatrix;
    ///
    /// let mat: DynamicMatrix<isize> = DynamicMatrix::with_capacity((3, 3));
    ///
    /// assert_eq!(mat.capacity(), 9);
    pub fn capacity(&self) -> usize {
        self.data.capacity()
    }

    /// Appends a new row to the DynamicMatrix
    ///
    /// ```
    /// # use simple_matrices::dynamic::row_major::DynamicMatrix;
    ///
    /// let mut mat: DynamicMatrix<isize> = DynamicMatrix::new_with_cols(3);
    ///
    /// mat.push_row(vec![1, 2, 3]).unwrap();
    /// mat.push_row(vec![4, 5, 6]).unwrap();
    /// mat.push_row(vec![7, 8, 9]).unwrap();
    ///
    /// assert_eq!(mat.as_slice(), [1, 2, 3, 4, 5, 6, 7, 8, 9]);
    /// assert_eq!(mat.rows(), 3);
    /// ```
    ///
    /// Trying to append a new row with unequal number of columns will return a `ShapeError`:
    /// ```should_panic
    /// # use simple_matrices::dynamic::row_major::DynamicMatrix;
    ///
    /// let mut mat: DynamicMatrix<isize> = DynamicMatrix::new_with_cols(3);
    ///
    /// // Trying to push a vector with length 4 into a matrix with only 3 columns
    /// mat.push_row(vec![1, 2, 3, 4]).unwrap();
    /// ```
    pub fn push_row(&mut self, row: Vec<T>) -> Result<(), ShapeError> {
        if row.len() != self.cols() {
            Err(ShapeError::new_cols_error(self.cols(), row.len()))
        } else {
            self.data.extend(row.into_iter());
            Ok(())
        }
    }

    /// Appends a new columns to the DynamicMatrix
    ///
    /// ```
    /// # use simple_matrices::dynamic::row_major::DynamicMatrix;
    ///
    /// let mut mat: DynamicMatrix<isize> = DynamicMatrix::new_with_cols(2);
    ///
    /// mat.push_row(vec![1, 2]).unwrap();
    /// mat.push_row(vec![4, 5]).unwrap();
    /// mat.push_row(vec![7, 8]).unwrap();
    ///
    /// mat.push_col(vec![3, 6, 9]).unwrap();
    ///
    /// assert_eq!(mat.as_slice(), &[1, 2, 3, 4, 5, 6, 7, 8, 9]);
    /// assert_eq!(mat.cols(), 3);
    /// ```
    ///
    /// Trying to append a new row with unequal number of columns will return a `ShapeError`:
    /// ```should_panic
    /// # use simple_matrices::dynamic::row_major::DynamicMatrix;
    ///
    /// let mut mat: DynamicMatrix<isize> = DynamicMatrix::new_with_cols(2);
    ///
    /// mat.push_row(vec![1, 2]).unwrap();
    /// mat.push_row(vec![4, 5]).unwrap();
    /// mat.push_row(vec![7, 8]).unwrap();
    ///
    /// // Trying to push a column with less elements than the number of rows
    /// mat.push_col(vec![3, 6]).unwrap();
    /// ```
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
    ///
    /// ```
    /// # use simple_matrices::{dynamic_matrix, dynamic::row_major::DynamicMatrix};
    ///
    /// let mat = dynamic_matrix![1, 2, 3; 4, 5, 6; 7, 8, 9];
    ///
    /// let mat_ptr = mat.as_ptr();
    /// for i in 0..(mat.rows() * mat.cols()) {
    ///     assert_eq!(unsafe { *mat_ptr.add(i) }, i as isize + 1);
    /// }
    /// ```
    pub fn as_ptr(&self) -> *const T {
        self.data.as_ptr()
    }

    /// Gives a raw mutable pointer to the underlying Vec's buffer
    ///
    /// ```
    /// # use simple_matrices::{dynamic_matrix, dynamic::row_major::DynamicMatrix};
    ///
    /// let mut mat = dynamic_matrix![1, 2, 3; 4, 5, 6; 7, 8, 9];
    ///
    /// let mat_ptr = mat.as_mut_ptr();
    /// for i in 0..(mat.rows() * mat.cols()) {
    ///     unsafe {
    ///         *mat_ptr.add(i) = i as isize + 10;
    ///     }
    /// }
    ///
    /// assert_eq!(mat.as_slice(), &[10, 11, 12, 13, 14, 15, 16, 17, 18]);
    /// ```
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self.data.as_mut_ptr()
    }

    /// Extracts a slice containing the underlying Vec
    ///
    /// ```
    /// # use simple_matrices::{dynamic_matrix, dynamic::row_major::DynamicMatrix};
    ///
    /// let mut mat = dynamic_matrix![1, 2, 3; 4, 5, 6; 7, 8, 9];
    ///
    /// assert_eq!(mat.as_slice(), &[1, 2, 3, 4, 5, 6, 7, 8, 9]);
    /// ```
    pub fn as_slice(&self) -> &[T] {
        self.data.as_slice()
    }

    /// Extracts a mut slice containing the underlying Vec
    ///
    /// ```
    /// # use simple_matrices::{dynamic_matrix, dynamic::row_major::DynamicMatrix};
    ///
    /// let mut mat = dynamic_matrix![1, 2, 3; 4, 5, 6; 7, 8, 9];
    /// let mut mat_slice = mat.as_mut_slice();
    ///
    /// mat_slice[0] = 10;
    /// mat_slice[1] = 11;
    /// mat_slice[2] = 12;
    ///
    /// assert_eq!(mat.as_slice(), &[10, 11, 12, 4, 5, 6, 7, 8, 9]);
    /// ```
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        self.data.as_mut_slice()
    }

    /// Decomposes the DynamicMatrix into the raw compoenents of it's underlying Vec
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

    /// Decomposes the DynamicMatrix into the boxed slice of it's underlying Vec
    ///
    /// ```
    /// # use simple_matrices::{dynamic_matrix, dynamic::row_major::DynamicMatrix};
    ///
    /// let mat = dynamic_matrix![1, 2, 3; 4, 5, 6; 7, 8, 9];
    ///
    /// let (slice, cols) = mat.into_boxed_slice();
    ///
    /// assert_eq!(cols, 3);
    /// assert_eq!(slice.as_ref(), [1, 2, 3, 4, 5, 6, 7, 8, 9]);
    /// ```
    pub fn into_boxed_slice(self) -> (Box<[T]>, usize) {
        let cols = self.cols();

        (self.data.into_boxed_slice(), cols)
    }

    /// Creates a DynamicMatrix from a Boxed slice
    ///
    /// ```
    /// # use simple_matrices::dynamic::row_major::DynamicMatrix;
    ///
    /// let boxed_slice = Box::new([1, 2, 3, 4, 5, 6, 7, 8, 9]);
    /// let mat = DynamicMatrix::from_boxed_slice(boxed_slice, 3);
    ///
    /// assert_eq!(mat.cols(), 3);
    /// assert_eq!(mat.as_slice(), &[1, 2, 3, 4, 5, 6, 7, 8, 9]);
    /// ```
    pub fn from_boxed_slice(boxed_slice: Box<[T]>, cols: usize) -> Self {
        Self {
            data: boxed_slice.into_vec(),
            cols,
        }
    }

    /// Returns a `Result` containing a shared reference to the value at the given index
    ///
    /// ```
    /// # use simple_matrices::{dynamic_matrix, dynamic::row_major::DynamicMatrix};
    ///
    /// let mat = dynamic_matrix![1, 2, 3; 4, 5, 6; 7, 8, 9];
    ///
    /// for row in 0..mat.rows() {
    ///     for col in 0..mat.cols() {
    ///         assert_eq!(*mat.get((row, col)).unwrap(), 3 * row + col + 1);
    ///     }
    /// }
    /// ```
    ///
    /// Indexing outside bounds will return an `IndexingError`.
    /// ```should_panic
    /// # use simple_matrices::{dynamic_matrix, dynamic::row_major::DynamicMatrix};
    ///
    /// let mat = dynamic_matrix![1, 2, 3; 4, 5, 6; 7, 8, 9];
    ///
    /// mat.get((3, 3)).unwrap();
    /// ```
    pub fn get(&self, index: (usize, usize)) -> Result<&T, IndexingError> {
        let (row, col) = index;
        if row < self.rows() && col < self.cols() {
            match self.data.get(row * self.cols() + col) {
                Some(v) => Ok(v),
                None => unreachable!(),
            }
        } else {
            Err(IndexingError::new(index, self.shape()))
        }
    }

    /// Returns a `Result` containing an exclusive reference to the value at the given index
    ///
    /// ```
    /// # use simple_matrices::{dynamic_matrix, dynamic::row_major::DynamicMatrix};
    ///
    /// let mut mat = dynamic_matrix![1, 2, 3; 4, 5, 6; 7, 8, 9];
    ///
    /// for row in 0..mat.rows() {
    ///     for col in 0..mat.cols() {
    ///         *mat.get_mut((row, col)).unwrap() += 9;
    ///     }
    /// }
    ///
    /// assert_eq!(mat.as_slice(), &[10, 11, 12, 13, 14, 15, 16, 17, 18]);
    /// ```
    ///
    /// Indexing outside bounds will return an `IndexingError`.
    /// ```should_panic
    /// # use simple_matrices::{dynamic_matrix, dynamic::row_major::DynamicMatrix};
    ///
    /// let mut mat = dynamic_matrix![1, 2, 3; 4, 5, 6; 7, 8, 9];
    ///
    /// *mat.get_mut((3, 3)).unwrap() += 1;
    /// ```
    pub fn get_mut(&mut self, index: (usize, usize)) -> Result<&mut T, IndexingError> {
        let (row, col) = index;
        let cols = self.cols();

        if row < self.rows() && col < self.cols() {
            match self.data.get_mut(row * cols + col) {
                Some(v) => Ok(v),
                None => unreachable!(),
            }
        } else {
            Err(IndexingError::new(index, self.shape()))
        }
    }
}

impl<T> Index<(usize, usize)> for DynamicMatrix<T> {
    type Output = T;

    /// Returns a shared reference to the value at the given index
    ///
    /// ```
    /// # use simple_matrices::{dynamic_matrix, dynamic::row_major::DynamicMatrix};
    ///
    /// let mat = dynamic_matrix![1, 2, 3; 4, 5, 6; 7, 8, 9];
    ///
    /// for row in 0..mat.rows() {
    ///     for col in 0..mat.cols() {
    ///         assert_eq!(mat[(row, col)], 3 * row + col + 1);
    ///     }
    /// }
    /// ```
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        self.get(index).unwrap()
    }
}

impl<T> IndexMut<(usize, usize)> for DynamicMatrix<T> {
    /// Returns an exclusive reference to the value at the given index
    ///
    /// ```
    /// # use simple_matrices::{dynamic_matrix, dynamic::row_major::DynamicMatrix};
    ///
    /// let mut mat = dynamic_matrix![1, 2, 3; 4, 5, 6; 7, 8, 9];
    ///
    /// for row in 0..mat.rows() {
    ///     for col in 0..mat.cols() {
    ///         mat[(row, col)] += 9;
    ///     }
    /// }
    ///
    /// assert_eq!(mat.as_slice(), &[10, 11, 12, 13, 14, 15, 16, 17, 18]);
    /// ```
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        self.get_mut(index).unwrap()
    }
}

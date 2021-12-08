use std::fmt;

// The error type for any shape errors
#[derive(Clone, Debug)]
pub struct ShapeError {
    rows: usize,
    cols: usize,
    expected_rows: usize,
    expected_cols: usize,
}

impl fmt::Display for ShapeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.rows != self.expected_rows {
            write!(
                f,
                "The operation performed expected {} rows but the matrix has {}.",
                self.expected_rows, self.rows
            )
        } else {
            write!(
                f,
                "The operation performed expected {} cols but the matrix has {}.",
                self.expected_cols, self.cols
            )
        }
    }
}

impl ShapeError {
    pub fn new(shape: (usize, usize), expected_shape: (usize, usize)) -> ShapeError {
        ShapeError {
            rows: shape.0,
            cols: shape.1,
            expected_rows: expected_shape.0,
            expected_cols: expected_shape.1,
        }
    }

    pub fn new_rows_error(rows: usize, expected_rows: usize) -> ShapeError {
        ShapeError {
            rows,
            cols: 0,
            expected_rows,
            expected_cols: 0,
        }
    }

    pub fn new_cols_error(cols: usize, expected_cols: usize) -> ShapeError {
        ShapeError {
            rows: 0,
            cols,
            expected_rows: 0,
            expected_cols,
        }
    }
}

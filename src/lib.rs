#[warn(missing_debug_implementations, missing_docs)]
use std::vec::Vec;

pub struct Mat<T> {
    data: Vec<T>,
    rows: usize,
    cols: usize,
}

impl<T> Mat<T> {
    /// Constructs a new, empty Mat<T>
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            rows: 0,
            cols: 0,
        }
    }

    pub fn as_slice(&self) -> &[T] {
        self.data.as_slice()
    }

    pub fn nrows(&self) -> usize {
        self.rows
    }

    pub fn ncols(&self) -> usize {
        self.cols
    }

    pub fn shape(&self) -> (usize, usize) {
        (self.rows, self.cols)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_tests() {}
}

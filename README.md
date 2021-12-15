# dynamic-matrix

A crate to work with dynamically sized matrices.

```rust
use dynamic_matrix::{dynamic_matrix, DynamicMatrix};

let mut mat = dynamic_matrix![
    1, 2; 
    4, 5;
];
// let mat: DynamicMatrix<isize> = DynamicMatrix::new([[1, 2], [4, 5]]);

assert_eq!(mat.shape(), (2, 2));

mat.push_row(vec![7, 8]).unwrap();
mat.push_col(vec![3, 6, 10]).unwrap();

assert_eq!(mat.shape(), (3, 3));

assert_eq!(mat[(1, 2)], 6);
mat[(2, 2)] = 9;

assert_eq!(mat.as_slice(), &[1, 2, 3, 4, 5, 6, 7, 8, 9]);
```

## Goals

The idea of this crate is to make working with matrices (internally stored by
flattening the elements into vectors) easy. The implementation will try to
mirror the methods on `std::vec::Vec` and provide methods to interact with the
shape and size of the matrix. However, it does not aim to provide ways to
perform any mathematical operations on these matrices.

Currently the focus is on row-major order but column-major order may be
supported in the future.

## Note

This is just a hobby project of mine since one of my other crates needs an
easier way to work with matrices. Any and all suggestions/contributions are
welcome.

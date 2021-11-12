# simple-matrices

A simple crate to work with matrices.

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

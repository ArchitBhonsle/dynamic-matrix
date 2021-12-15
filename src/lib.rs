#![warn(missing_debug_implementations, missing_docs)]

//! A crate to work with dynamically sized matrices.

/// Contains the errors associated with this crate
pub mod errors;
/// Contains the row major ordered DynamicMatrix
mod row_major;

// Re-exporting for ease-of-use
pub use row_major::DynamicMatrix;

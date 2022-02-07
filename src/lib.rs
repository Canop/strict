//! Provide the NonEmptyVec as I wanted it to be.
//!
//! This small lib has better alternatives.
//! I made it for my own consumption first.
mod non_empty_vec;
pub use non_empty_vec::*;

mod non_empty_slice;
pub use non_empty_slice::*;

#[derive(Debug, Clone)]
pub struct NotEnoughElementsError;

//! Provide some types with inherent bounds, useful when you want to avoid unwrap or want const
//! matching.
//!
mod non_empty_vec;
mod one_to_three;

pub use {non_empty_vec::*, one_to_three::*};

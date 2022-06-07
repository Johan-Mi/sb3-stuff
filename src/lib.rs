#![deny(unsafe_code)]
#![warn(clippy::cargo)]
#![warn(clippy::missing_const_for_fn)]

//! Utilities for working with `.sb3` files.

mod index;
mod value;

pub use index::*;
pub use value::*;

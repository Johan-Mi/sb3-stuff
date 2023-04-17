#![forbid(unsafe_code)]
#![warn(clippy::cargo, clippy::missing_const_for_fn)]
#![no_std]

//! Utilities for working with `.sb3` files.

extern crate alloc;

mod index;
mod value;

pub use index::*;
pub use value::*;

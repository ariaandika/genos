//! Memory management.
pub use mmap::Mmap;
pub use brk::{brk, current_brk};
pub use error::OutOfMemory;

pub mod mmap;
mod brk;
mod error;

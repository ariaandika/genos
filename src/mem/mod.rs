//! Memory allocation.
pub use mmap::{Mmap, Prot, mmap, munmap};
pub use brk::{brk, current_brk};
pub use error::OutOfMemory;

pub mod mmap;
mod brk;
mod error;

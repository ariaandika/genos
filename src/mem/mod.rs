//! Memory allocation.
pub use mmap::{Prot, mmap, munmap};
pub use brk::brk;

pub mod mmap;
mod brk;

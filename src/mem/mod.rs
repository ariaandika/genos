//! Memory allocation.
pub use mmap::{Prot, mmap, munmap};
pub use error::OutOfMemory;
pub use brk::brk;

pub mod mmap;
mod brk;
mod error;

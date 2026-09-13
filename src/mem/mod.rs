//! Memory allocation.
pub use mmap::{Mmap, MmapError, MmapFlags, Protection, mmap, munmap};
pub use brk::{brk, current_brk};
pub use error::OutOfMemory;

mod mmap;
mod brk;
mod error;

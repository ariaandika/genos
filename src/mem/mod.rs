//! Memory allocation.
#[doc(inline)]
pub use mmap::{Prot, mmap, mprotect, munmap};
#[doc(inline)]
pub use memfd::Memfd;
pub use brk::brk;

pub mod mmap;
pub mod memfd;
mod brk;

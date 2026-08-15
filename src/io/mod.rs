//! I/O Abstraction.
pub use read::{Read, ReadError};
pub use write::{Write, WriteError};

mod read;
mod write;

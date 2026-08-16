//! I/O Abstraction.
pub use read::{Read, ReadError};
pub use write::{Write, WriteError};
pub use stream::{Stderr, Stdin, Stdout};

mod read;
mod write;
mod stream;

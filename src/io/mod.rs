//! Input/Output operations.
pub use iovec::{IoVec, IoVecMut};
pub use flags::RWFlags;
pub use read::{ReadError, read, pread, readv, preadv};
pub use write::{WriteError, write, pwrite, writev, pwritev};
pub use stream::{Stderr, Stdin, Stdout};
pub use splice::{SpliceError, SpliceFlags, sendfile, splice, tee, vmsplice};

mod iovec;
mod flags;
mod stream;
mod read;
mod write;
mod splice;

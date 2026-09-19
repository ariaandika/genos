//! Input/Output operations.
pub use iovec::{IoVec, IoVecMut};
pub use flags::RWFlags;
pub use read::{read, pread, readv, preadv};
pub use write::{write, pwrite, writev, pwritev};
pub use stream::{Stderr, Stdin, Stdout};
pub use splice::{SpliceFlags, sendfile, splice, tee, vmsplice};

pub(crate) use read::read_raw;
pub(crate) use write::write_raw;

mod iovec;
mod flags;
mod stream;
mod read;
mod write;
mod splice;

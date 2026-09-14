//! Input/Output operations.
pub use iovec::{IoVec, IoVecMut};
pub use flags::RWFlags;
pub use read::{ReadError, read, pread, readv, preadv};
pub use write::{WriteError, write, pwrite, writev, pwritev};
pub use stream::{Stderr, Stdin, Stdout};
pub use splice::{SpliceError, SpliceFlags, sendfile, splice, tee, vmsplice};

/// Integer representing file size.
///
/// Reference: `off_t(3type)`.
pub type Offset = crate::sys::off_t;

mod iovec;
mod flags;
mod stream;
mod read;
mod write;
mod splice;

/// Write to standard output with a newline.
#[macro_export]
macro_rules! println {
    ($($tt:tt)*) => {{
        use core::fmt::Write;
        let _ = core::writeln!(genos::io::Stdout, $($tt)*);
    }};
}

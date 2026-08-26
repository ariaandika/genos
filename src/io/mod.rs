//! I/O Abstraction.
pub use iovec::{IoVec, IoVecMut};
pub use flags::IOFlags;
pub use read::{Read, ReadError};
pub use write::{Write, WriteError};
pub use stream::{Stderr, Stdin, Stdout};

/// Integer representing file size.
///
/// Reference: `off_t(3type)`.
pub type Offset = crate::sys::off_t;

mod iovec;
mod flags;
mod read;
mod write;
mod stream;

/// Write to standard output with a newline.
#[macro_export]
macro_rules! println {
    ($($tt:tt)*) => {{
        use core::fmt::Write;
        let _ = core::writeln!(genos::io::Stdout, $($tt)*);
    }};
}

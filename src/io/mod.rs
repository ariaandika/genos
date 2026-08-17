//! I/O Abstraction.
pub use read::{Read, ReadError};
pub use write::{Write, WriteError};
pub use stream::{Stderr, Stdin, Stdout};

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

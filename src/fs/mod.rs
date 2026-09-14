//! Filesystem manipulation.
#[doc(inline)]
pub use file::{File, Seek};
pub use path::{link, rename, symlink, truncate, unlink};

pub mod file;
mod path;

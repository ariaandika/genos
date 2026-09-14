//! Filesystem manipulation.
#[doc(inline)]
pub use file::{File, Seek};
pub use path::Path;

pub mod file;
mod path;

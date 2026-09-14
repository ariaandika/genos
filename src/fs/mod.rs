//! Filesystem manipulation.
#[doc(inline)]
pub use file::File;

pub use path::Path;
pub use error::{Error, Result};

pub mod file;
mod path;
mod error;

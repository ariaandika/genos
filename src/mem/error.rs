use core::{error, fmt};

use crate::error::{AsErrCode, ErrCode};

/// An error that may occur when allocating memory.
#[derive(Debug)]
pub struct OutOfMemory;

impl error::Error for OutOfMemory {}

impl fmt::Display for OutOfMemory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_err_code().fmt(f)
    }
}

impl AsErrCode for OutOfMemory {
    #[inline]
    fn as_err_code(&self) -> ErrCode {
        ErrCode::ENOMEM
    }
}

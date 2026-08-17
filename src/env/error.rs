use crate::error::{ErrCode, FromErrCode, os_error_simple};

/// An error that may occur when configuring environment variable.
#[derive(Clone, Copy)]
pub struct Error(ErrCode);

impl Error {
    pub(super) fn from_res(res: i32) -> Result<(), Self> {
        if res == 0 { Ok(()) } else { Err(Self::errno()) }
    }
}

os_error_simple!(Error, "configure environment variable");

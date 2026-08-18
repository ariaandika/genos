use core::error;

use crate::error::{ErrCode, FromErrCode, impl_error_os_simple};
use crate::fd::AsFd;
use crate::sys;

/// A writable file descriptor.
pub trait Write: AsFd {
    /// An error that may occur when writing to this file descriptor.
    type Error: error::Error + From<WriteError>;

    /// Writes bytes from the buffer to this fd.
    ///
    /// On success, the number of bytes written is returned.
    ///
    /// The number of bytes written may be less than count if, for example, there is insufficient
    /// space on the underlying physical medium, or the RLIMIT_FSIZE resource limit is encountered,
    /// or the call was interrupted by a signal handler after having written less than count bytes.
    #[inline]
    fn write(&self, buf: &[u8]) -> Result<usize, Self::Error> {
        let res = sys::call!(RD, __NR_write, self.as_fd(), buf, buf.len());
        WriteError::es(res).map_err(<_>::into)
    }
}

/// An error that may occur when writing to fd.
#[derive(Clone, Copy)]
pub struct WriteError(ErrCode);

impl_error_os_simple!(WriteError, "write to fd");

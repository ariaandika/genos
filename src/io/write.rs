use core::error;

use crate::error::{ErrCode, FromErrCode, os_error_simple};
use crate::fd::{AsFd, AsRawFd};

/// A writable file descriptor.
pub trait Write: AsFd {
    /// An error that may occur when writing to this file descriptor.
    type Error: error::Error + From<WriteError>;

    /// Writes bytes from the buffer to this fd.
    ///
    /// On success, the number of bytes written is returned.
    ///
    /// The number of bytes written may be less than count if, for example, there is insufficient space on the underlying physical medium, or the RLIMIT_FSIZE resource limit is encountered (see setrlimit(2)), or the
    /// call was interrupted by a signal handler after having written less than count bytes.
    #[inline]
    fn write(&self, buf: &[u8]) -> Result<usize, Self::Error> {
        let fd = self.as_fd().as_raw_fd();
        let res = unsafe { libc::write(fd, buf.as_ptr().cast(), buf.len()) };
        usize::try_from(res).map_err(|_| WriteError::errno().into())
    }
}

/// An error that may occur when writing to fd.
#[derive(Clone, Copy)]
pub struct WriteError(ErrCode);

os_error_simple!(WriteError, "write to fd");

use core::error;
use core::mem::MaybeUninit;

use crate::error::{ErrCode, FromErrCode, os_error_simple};
use crate::fd::{AsFd, AsRawFd};

/// A readable file descriptor.
pub trait Read: AsFd {
    /// An error that may occur when reading from this fd.
    type Error: error::Error + From<ReadError>;

    /// Read bytes from this fd into the buffer.
    ///
    /// On success, the number of bytes read is returned (zero indicates end of file), and the file
    /// position is advanced by this number.
    ///
    /// It is not an error if this number is smaller than the number of bytes requested; this may
    /// happen for example because fewer bytes are actually available right now (maybe because
    /// end-of-file were close, or because reading from a pipe, or from a terminal), or because
    /// interrupted by a signal.
    #[inline]
    fn read(&self, buf: &mut [MaybeUninit<u8>]) -> Result<usize, Self::Error> {
        let fd = self.as_fd().as_raw_fd();
        let res = unsafe { libc::read(fd, buf.as_mut_ptr().cast(), buf.len()) };
        usize::try_from(res).map_err(|_| ReadError::errno().into())
    }
}

/// An error that may occur when reading from fd.
#[derive(Clone, Copy)]
pub struct ReadError(ErrCode);

os_error_simple!(ReadError, "read from fd");

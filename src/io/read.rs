use core::mem::MaybeUninit;

use crate::error::{ErrCode, SysResExt};
use crate::fd::AsFd;
use crate::io::{IOFlags, IoVecMut, Offset};
use crate::{error, sys};

/// Read bytes from this fd into the buffer.
///
/// On success, the number of bytes read is returned (zero indicates end of file), and the file
/// position is advanced by this number.
///
/// It is not an error if this number is smaller than the number of bytes requested; this may happen
/// for example because fewer bytes are actually available right now (maybe because end-of-file were
/// close, or because reading from a pipe, or from a terminal), or because interrupted by a signal.
#[inline]
pub fn read<Fd: AsFd>(fd: Fd, buf: &mut [MaybeUninit<u8>]) -> Result<usize, ReadError> {
    sys::call!(sys_read, fd.as_fd(), &mut *buf, buf.len()).io2::<ReadError>()
}

/// A readable file descriptor.
pub trait Read: AsFd {
    /// An error that may occur when reading from this fd.
    type Error: error::Error + From<ReadError>;

    /// Read bytes from this fd into the buffer.
    ///
    /// For more details see [`read`] for more details.
    #[inline]
    fn read(&self, buf: &mut [MaybeUninit<u8>]) -> Result<usize, Self::Error> {
        read(self, buf).map_err(<_>::into)
    }

    /// Read bytes from this fd into the buffer at given offset.
    #[inline]
    fn pread(&self, buf: &mut [MaybeUninit<u8>], offset: Offset) -> Result<usize, Self::Error> {
        sys::call!(sys_pread64, self.as_fd(), &mut *buf, buf.len(), offset)
            .io2::<ReadError>()
            .map_err(<_>::into)
    }

    /// Read bytes from this fd into given scattered buffer.
    #[inline]
    fn readv(&self, buf: &[IoVecMut]) -> Result<usize, Self::Error> {
        sys::call!(sys_readv, self.as_fd(), buf, buf.len())
            .io2::<ReadError>()
            .map_err(<_>::into)
    }

    /// Read bytes from this fd into given scattered buffer at given offset.
    #[inline]
    fn preadv(
        &self,
        buf: &[IoVecMut],
        offset: Offset,
        flags: IOFlags,
    ) -> Result<usize, Self::Error> {
        sys::call!(sys_preadv2, self.as_fd(), buf, buf.len(), offset, i32::from(flags))
            .io2::<ReadError>()
            .map_err(<_>::into)
    }
}

// ===== Error =====

/// An error that may occur when reading from fd.
#[derive(Clone, Copy)]
pub struct ReadError(ErrCode);

error::impl_error_os_simple!(ReadError, "read from fd");

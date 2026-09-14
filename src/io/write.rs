use crate::error::{ErrCode, SysResExt};
use crate::fd::AsFd;
use crate::io::{IOFlags, IoVec, Offset};
use crate::{error, sys};

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
        sys::call_rd!(sys_write, self.as_fd(), buf, buf.len())
            .io2::<WriteError>()
            .map_err(<_>::into)
    }

    /// Writes bytes from the buffer to this fd at given offset.
    #[inline]
    fn pwrite(&self, buf: &[u8], offset: Offset) -> Result<usize, Self::Error> {
        sys::call_rd!(sys_pwrite64, self.as_fd(), buf, buf.len(), offset)
            .io2::<WriteError>()
            .map_err(<_>::into)
    }

    /// Writes bytes from gathered buffer to this fd.
    #[inline]
    fn writev(&self, buf: &[IoVec<'_>]) -> Result<usize, Self::Error> {
        sys::call_rd!(sys_writev, self.as_fd(), buf, buf.len())
            .io2::<WriteError>()
            .map_err(<_>::into)
    }

    /// Writes bytes from gathered buffer to this fd at given offset.
    #[inline]
    fn pwritev(
        &self,
        buf: &[IoVec<'_>],
        offset: Offset,
        flags: IOFlags,
    ) -> Result<usize, Self::Error> {
        sys::call_rd!(sys_pwritev, self.as_fd(), buf, buf.len(), offset, i32::from(flags))
            .io2::<WriteError>()
            .map_err(<_>::into)
    }
}

// ===== Error =====

/// An error that may occur when writing to fd.
#[derive(Clone, Copy)]
pub struct WriteError(ErrCode);

error::impl_error_os_simple!(WriteError, "write to fd");

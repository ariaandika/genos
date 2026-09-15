use crate::error::{ErrCode, SysResExt};
use crate::fd::AsFd;
use crate::ffi::Off;
use crate::io::{IoVec, RWFlags};
use crate::{error, sys};

/// Writes bytes from the buffer to this fd (`write(2)`).
#[inline]
pub fn write<Fd: AsFd + ?Sized>(fd: &Fd, buf: &[u8]) -> Result<usize, WriteError> {
    sys::call!(sys_write, fd.as_fd(), buf.as_ptr(), buf.len()).io2::<WriteError>()
}

/// Writes bytes from the buffer to this fd at given offset (`pwrite(2)`).
#[inline]
pub fn pwrite<Fd: AsFd + ?Sized>(fd: &Fd, buf: &[u8], offset: Off) -> Result<usize, WriteError> {
    sys::call_rd!(sys_pwrite64, fd.as_fd(), buf, buf.len(), offset).io2::<WriteError>()
}

/// Writes bytes from gathered buffer to this fd (`writev(2)`).
#[inline]
pub fn writev<Fd: AsFd + ?Sized>(fd: &Fd, buf: &[IoVec<'_>]) -> Result<usize, WriteError> {
    sys::call_rd!(sys_writev, fd.as_fd(), buf, buf.len()).io2::<WriteError>()
}

/// Writes bytes from gathered buffer to this fd at given offset (`pwritev(2)`).
#[inline]
pub fn pwritev<Fd: AsFd + ?Sized>(
    fd: &Fd,
    buf: &[IoVec<'_>],
    offset: Off,
    flags: RWFlags,
) -> Result<usize, WriteError> {
    sys::call_rd!(sys_pwritev2, fd.as_fd(), buf, buf.len(), offset, i32::from(flags))
        .io2::<WriteError>()
}

// ===== Error =====

/// An error that may occur when writing to fd.
#[derive(Clone, Copy)]
pub struct WriteError(ErrCode);

error::impl_error_os_simple!(WriteError, "write to fd");

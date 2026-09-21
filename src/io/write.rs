use crate::fd::AsFd;
use crate::ffi::Off;
use crate::io::{IoVec, RWFlags};
use crate::sys::{self, Error, arch};

pub(crate) fn write_raw<T>(
    fd: i32,
    buf: *const T,
    size: usize,
) -> Result<usize, Error<arch::sys_write>> {
    sys::call_rd!(sys_write, fd, buf, size)
}

/// Writes bytes from the buffer to this fd (`write(2)`).
#[inline]
pub fn write<Fd: AsFd + ?Sized>(fd: &Fd, buf: &[u8]) -> Result<usize, Error<arch::sys_write>> {
    write_raw(fd.as_raw_fd(), buf.as_ptr(), buf.len())
}

/// Writes bytes from the buffer to this fd at given offset (`pwrite(2)`).
#[inline]
pub fn pwrite<Fd>(fd: &Fd, buf: &[u8], offset: Off) -> Result<usize, Error<arch::sys_pwrite64>>
where
    Fd: AsFd + ?Sized,
{
    sys::call_rd!(sys_pwrite64, fd.as_raw_fd(), buf.as_ptr(), buf.len(), offset)
}

/// Writes bytes from gathered buffer to this fd (`writev(2)`).
#[inline]
pub fn writev<Fd>(fd: &Fd, buf: &[IoVec<'_>]) -> Result<usize, Error<arch::sys_writev>>
where
    Fd: AsFd + ?Sized,
{
    sys::call_rd!(sys_writev, fd.as_raw_fd(), buf.as_ptr(), buf.len())
}

/// Writes bytes from gathered buffer to this fd at given offset (`pwritev(2)`).
#[inline]
pub fn pwritev<Fd: AsFd + ?Sized>(
    fd: &Fd,
    buf: &[IoVec<'_>],
    offset: Off,
    flags: RWFlags,
) -> Result<usize, Error<arch::sys_pwritev2>> {
    sys::call_rd!(sys_pwritev2, fd.as_raw_fd(), buf.as_ptr(), buf.len(), offset, i32::from(flags))
}

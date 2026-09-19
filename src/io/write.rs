use crate::fd::AsFd;
use crate::ffi::Off;
use crate::io::{IoVec, RWFlags};
use crate::sys::{self, SysRes};

/// Writes bytes from the buffer to this fd (`write(2)`).
#[inline]
pub fn write<Fd: AsFd + ?Sized>(fd: &Fd, buf: &[u8]) -> impl SysRes<usize> {
    sys::call_rd!(sys_write, fd.as_raw_fd(), buf.as_ptr(), buf.len())
}

/// Writes bytes from the buffer to this fd at given offset (`pwrite(2)`).
#[inline]
pub fn pwrite<Fd: AsFd + ?Sized>(fd: &Fd, buf: &[u8], offset: Off) -> impl SysRes<usize> {
    sys::call_rd!(sys_pwrite64, fd.as_raw_fd(), buf.as_ptr(), buf.len(), offset)
}

/// Writes bytes from gathered buffer to this fd (`writev(2)`).
#[inline]
pub fn writev<Fd: AsFd + ?Sized>(fd: &Fd, buf: &[IoVec<'_>]) -> impl SysRes<usize> {
    sys::call_rd!(sys_writev, fd.as_raw_fd(), buf.as_ptr(), buf.len())
}

/// Writes bytes from gathered buffer to this fd at given offset (`pwritev(2)`).
#[inline]
pub fn pwritev<Fd>(fd: &Fd, buf: &[IoVec<'_>], offset: Off, flags: RWFlags) -> impl SysRes<usize>
where
    Fd: AsFd + ?Sized,
{
    sys::call_rd!(sys_pwritev2, fd.as_raw_fd(), buf.as_ptr(), buf.len(), offset, i32::from(flags))
}

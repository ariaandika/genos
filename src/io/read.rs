use core::mem::MaybeUninit;

use crate::fd::AsFd;
use crate::ffi::Off;
use crate::io::{IoVecMut, RWFlags};
use crate::sys::{self, Error, arch};

pub(crate) fn read_raw<T>(
    fd: i32,
    buf: *mut T,
    size: usize,
) -> Result<usize, Error<arch::sys_read>> {
    sys::call!(sys_read, fd, buf, size)
}

/// Read bytes from this fd into the buffer (`read(2)`).
#[inline]
pub fn read<Fd>(fd: &Fd, buf: &mut [MaybeUninit<u8>]) -> Result<usize, Error<arch::sys_read>>
where
    Fd: AsFd + ?Sized,
{
    read_raw(fd.as_raw_fd(), buf.as_mut_ptr(), buf.len())
}

/// Read bytes from this fd into the buffer at given offset (`pread(2)`).
#[inline]
pub fn pread<Fd: AsFd + ?Sized>(
    fd: &Fd,
    buf: &mut [MaybeUninit<u8>],
    offset: Off,
) -> Result<usize, Error<arch::sys_pread64>> {
    sys::call!(sys_pread64, fd.as_raw_fd(), buf.as_mut_ptr(), buf.len(), offset)
}

/// Read bytes from this fd into given scattered buffer (`readv(2)`).
#[inline]
pub fn readv<Fd>(fd: &Fd, buf: &[IoVecMut<'_>]) -> Result<usize, Error<arch::sys_readv>>
where
    Fd: AsFd + ?Sized,
{
    sys::call!(sys_readv, fd.as_raw_fd(), buf.as_ptr(), buf.len())
}

/// Read bytes from this fd into given scattered buffer at given offset (`preadv2(2)`).
#[inline]
pub fn preadv<Fd: AsFd + ?Sized>(
    fd: &Fd,
    buf: &[IoVecMut<'_>],
    offset: Off,
    flags: RWFlags,
) -> Result<usize, Error<arch::sys_preadv2>> {
    sys::call!(sys_preadv2, fd.as_raw_fd(), buf.as_ptr(), buf.len(), offset, i32::from(flags))
}

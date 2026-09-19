use core::mem::MaybeUninit;

use crate::fd::AsFd;
use crate::ffi::Off;
use crate::io::{IoVecMut, RWFlags};
use crate::sys::{self, SysRes};

/// Read bytes from this fd into the buffer (`read(2)`).
#[inline]
pub fn read<Fd: AsFd + ?Sized>(fd: &Fd, buf: &mut [MaybeUninit<u8>]) -> impl SysRes<usize> {
    sys::call!(sys_read, fd.as_raw_fd(), buf.as_mut_ptr(), buf.len())
}

/// Read bytes from this fd into the buffer at given offset (`pread(2)`).
#[inline]
pub fn pread<Fd>(fd: &Fd, buf: &mut [MaybeUninit<u8>], offset: Off) -> impl SysRes<usize>
where
    Fd: AsFd + ?Sized,
{
    sys::call!(sys_pread64, fd.as_raw_fd(), buf.as_mut_ptr(), buf.len(), offset)
}

/// Read bytes from this fd into given scattered buffer (`readv(2)`).
#[inline]
pub fn readv<Fd: AsFd + ?Sized>(fd: &Fd, buf: &[IoVecMut<'_>]) -> impl SysRes<usize> {
    sys::call!(sys_readv, fd.as_raw_fd(), buf.as_ptr(), buf.len())
}

/// Read bytes from this fd into given scattered buffer at given offset (`preadv2(2)`).
#[inline]
pub fn preadv<Fd>(fd: &Fd, buf: &[IoVecMut<'_>], offset: Off, flags: RWFlags) -> impl SysRes<usize>
where
    Fd: AsFd + ?Sized,
{
    sys::call!(sys_preadv2, fd.as_raw_fd(), buf.as_ptr(), buf.len(), offset, i32::from(flags))
}

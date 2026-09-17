use core::mem::MaybeUninit;

use crate::error::{ErrCode, SysResExt};
use crate::fd::AsFd;
use crate::ffi::Off;
use crate::io::{IoVecMut, RWFlags};
use crate::{error, sys};

/// Read bytes from this fd into the buffer (`read(2)`).
#[inline]
pub fn read<Fd: AsFd + ?Sized>(fd: &Fd, buf: &mut [MaybeUninit<u8>]) -> Result<usize, ReadError> {
    sys::call!(sys_read, fd.as_raw_fd(), buf.as_mut_ptr(), buf.len()).io2::<ReadError>()
}

/// Read bytes from this fd into the buffer at given offset (`pread(2)`).
#[inline]
pub fn pread<Fd: AsFd + ?Sized>(
    fd: &Fd,
    buf: &mut [MaybeUninit<u8>],
    offset: Off,
) -> Result<usize, ReadError> {
    sys::call!(sys_pread64, fd.as_raw_fd(), buf.as_mut_ptr(), buf.len(), offset).io2::<ReadError>()
}

/// Read bytes from this fd into given scattered buffer (`readv(2)`).
#[inline]
pub fn readv<Fd: AsFd + ?Sized>(fd: &Fd, buf: &[IoVecMut<'_>]) -> Result<usize, ReadError> {
    sys::call!(sys_readv, fd.as_raw_fd(), buf.as_ptr(), buf.len()).io2::<ReadError>()
}

/// Read bytes from this fd into given scattered buffer at given offset (`preadv2(2)`).
#[inline]
pub fn preadv<Fd: AsFd + ?Sized>(
    fd: &Fd,
    buf: &[IoVecMut<'_>],
    offset: Off,
    flags: RWFlags,
) -> Result<usize, ReadError> {
    sys::call!(sys_preadv2, fd.as_raw_fd(), buf.as_ptr(), buf.len(), offset, i32::from(flags))
        .io2::<ReadError>()
}

// ===== Error =====

/// An error that may occur when reading from fd.
#[derive(Clone, Copy)]
pub struct ReadError(ErrCode);

error::impl_error_os_simple!(ReadError, "read from fd");

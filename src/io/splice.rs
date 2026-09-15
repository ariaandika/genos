use crate::error::{ErrCode, SysResExt};
use crate::fd::AsFd;
use crate::ffi::Off;
use crate::io::IoVecMut;
use crate::{error, flags, sys};

/// Copies data between one file descriptor and another (`sendfile(2)`).
#[inline]
pub fn sendfile<O: AsFd + ?Sized, I: AsFd + ?Sized>(
    out_fd: &O,
    in_fd: &I,
    offset: Option<&mut Off>,
    count: usize,
) -> Result<usize, SpliceError> {
    sys::call!(sys_sendfile, out_fd.as_fd(), in_fd.as_fd(), offset, count).io2()
}

/// Splice data to/from a pipe (`splice(2)`).
#[inline]
pub fn splice<I: AsFd + ?Sized, O: AsFd + ?Sized>(
    fd_in: &I,
    off_in: Option<&mut Off>,
    fd_out: &O,
    off_out: Option<&mut Off>,
    size: usize,
    flags: SpliceFlags,
) -> Result<usize, SpliceError> {
    sys::call!(sys_splice, fd_in.as_fd(), off_in, fd_out.as_fd(), off_out, size, flags.0).io2()
}

/// Duplicate pipe content (`tee(2)`).
#[inline]
pub fn tee<I: AsFd + ?Sized, O: AsFd + ?Sized>(
    fd_in: &I,
    fd_out: &O,
    size: usize,
    flags: SpliceFlags,
) -> Result<usize, SpliceError> {
    sys::call!(sys_tee, fd_in.as_fd(), fd_out.as_fd(), size, flags.0).io2()
}

/// Splice user pages to/from a pipe (`vmsplice(2)`).
#[inline]
pub fn vmsplice<Fd: AsFd + ?Sized>(
    fd: &Fd,
    iov: &[IoVecMut<'_>],
    flags: SpliceFlags,
) -> Result<usize, SpliceError> {
    sys::call!(sys_vmsplice, fd.as_fd(), iov, iov.len(), flags.0).io2()
}

// ===== flags =====

/// Flags for buffer copying operations (`splice(2)`).
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct SpliceFlags(u32);

flags::impl_bitops_simple!(SpliceFlags);

// include/linux/splice.h

impl SpliceFlags {
    /// `SPLICE_F_MOVE`
    pub const MOVE: Self = Self(0x01);
    /// `SPLICE_F_NONBLOCK`
    pub const NONBLOCK: Self = Self(0x02);
    /// `SPLICE_F_MORE`
    pub const MORE: Self = Self(0x04);
    /// `SPLICE_F_GIFT`
    pub const GIFT: Self = Self(0x08);
}

// ===== error =====

/// An error that may occur when splicing buffer between fds.
#[derive(Clone, Copy)]
pub struct SpliceError(ErrCode);

error::impl_error_os_simple!(SpliceError, "splice fds");

//! Fd splicing.
use crate::error::{ErrCode, SysResExt};
use crate::fd::AsFd;
use crate::io::{IoVecMut, Offset};
use crate::{error, flags, sys};

/// Copies data between one file descriptor and another.
///
/// Reference: `sendfile(2)`.
#[inline]
pub fn sendfile<O: AsFd, I: AsFd>(
    out_fd: &O,
    in_fd: &I,
    offset: Option<&mut Offset>,
    count: usize,
) -> Result<usize, SpliceError> {
    sys::call!(sys_sendfile, out_fd.as_fd(), in_fd.as_fd(), offset, count).io2()
}

/// Splice data to/from a pipe.
///
/// Reference: `splice(2)`.
#[inline]
pub fn splice<I: AsFd, O: AsFd>(
    fd_in: &I,
    off_in: Option<&mut Offset>,
    fd_out: &O,
    off_out: Option<&mut Offset>,
    size: usize,
    flags: SpliceFlags,
) -> Result<usize, SpliceError> {
    sys::call!(sys_splice, fd_in.as_fd(), off_in, fd_out.as_fd(), off_out, size, flags.0).io2()
}

/// Duplicate pipe content.
///
/// Reference: `tee(2)`.
#[inline]
pub fn tee<I: AsFd, O: AsFd>(
    fd_in: &I,
    fd_out: &O,
    size: usize,
    flags: SpliceFlags,
) -> Result<usize, SpliceError> {
    sys::call!(sys_tee, fd_in.as_fd(), fd_out.as_fd(), size, flags.0).io2()
}

/// Splice user pages to/from a pipe.
///
/// Reference: `vmsplice(2)`.
#[inline]
pub fn vmsplice<Fd: AsFd>(
    fd: &Fd,
    iov: &[IoVecMut<'_>],
    flags: SpliceFlags,
) -> Result<usize, SpliceError> {
    sys::call!(sys_vmsplice, fd.as_fd(), iov, iov.len(), flags.0).io2()
}

// ===== flags =====

/// Flags for buffer copying operations.
///
/// Reference: `splice(2)`.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct SpliceFlags(u32);

impl SpliceFlags {
    /// Attempt to move pages instead of copying.
    ///
    /// Unused for [`vmsplice`], and currently has no effect on [`tee`].
    pub const MOVE: Self = Self(sys::SPLICE_F_MOVE);
    /// Do not block on I/O.
    pub const NONBLOCK: Self = Self(sys::SPLICE_F_NONBLOCK);
    /// More data will be coming in a subsequent splice.
    ///
    /// Currently has no effect on [`vmsplice`] and [`tee`].
    pub const MORE: Self = Self(sys::SPLICE_F_MORE);
    /// The user pages are a gift to the kernel.
    ///
    /// Unused for [`splice`] and [`tee`].
    pub const GIFT: Self = Self(sys::SPLICE_F_GIFT);
}

flags::impl_bitops_simple!(SpliceFlags);

// ===== error =====

/// An error that may occur when splicing buffer between fds.
#[derive(Clone, Copy)]
pub struct SpliceError(ErrCode);

error::impl_error_os_simple!(SpliceError, "splice fds");

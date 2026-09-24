use crate::fd::AsFd;
use crate::ffi::Off;
use crate::io::IoVecMut;
use crate::sys::{Error, arch};
use crate::{flags, sys};

/// Copies data between one file descriptor and another (`sendfile(2)`).
#[inline]
pub fn sendfile<O: AsFd + ?Sized, I: AsFd + ?Sized>(
    out_fd: &O,
    in_fd: &I,
    offset: Option<&mut Off>,
    count: usize,
) -> Result<usize, Error<arch::sys_sendfile64>> {
    let offset = sys::optmut(offset);
    sys::call!(sys_sendfile64, out_fd.as_raw_fd(), in_fd.as_raw_fd(), offset, count)
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
) -> Result<usize, Error<arch::sys_splice>> {
    let off_in = sys::optmut(off_in);
    let off_out = sys::optmut(off_out);
    sys::call!(sys_splice, fd_in.as_raw_fd(), off_in, fd_out.as_raw_fd(), off_out, size, flags.0)
}

/// Duplicate pipe content (`tee(2)`).
#[inline]
pub fn tee<I: AsFd + ?Sized, O: AsFd + ?Sized>(
    fd_in: &I,
    fd_out: &O,
    size: usize,
    flags: SpliceFlags,
) -> Result<usize, Error<arch::sys_tee>> {
    sys::call_rd!(sys_tee, fd_in.as_raw_fd(), fd_out.as_raw_fd(), size, flags.0)
}

/// Splice user pages to/from a pipe (`vmsplice(2)`).
#[inline]
pub fn vmsplice<Fd: AsFd + ?Sized>(
    fd: &Fd,
    iov: &mut [IoVecMut<'_>],
    flags: SpliceFlags,
) -> Result<usize, Error<arch::sys_vmsplice>> {
    sys::call!(sys_vmsplice, fd.as_raw_fd(), iov.as_ptr(), iov.len(), flags.0)
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

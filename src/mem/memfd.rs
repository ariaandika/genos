//! [`Memfd`] associated types.
use crate::error::{self, ErrCode, SysResExt};
use crate::fd::{self, OwnedFd};
use crate::ffi::Char;
use crate::{flags, sys};

// ===== Memfd =====

/// A handle to an anonymus file.
#[derive(Debug)]
pub struct Memfd(OwnedFd);

fd::impl_fd_simple!(Memfd);

impl Memfd {
    /// Creates new [`Memfd`] (`memfd_create(2)`).
    #[inline]
    pub fn create(name: &Char, flags: Flags) -> Result<Self, Error> {
        sys::call_rd!(sys_memfd_create, name, flags.0).fd2()
    }
}

// ===== Flags =====

/// [`Memfd::create`] flags.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Flags(u32);

flags::impl_bitops_simple!(Flags);

impl Memfd {
    /// `MFD_CLOEXEC`
    pub const CLOEXEC: Flags = Flags(MFD_CLOEXEC);
    /// `MFD_ALLOW_SEALING`
    pub const ALLOW_SEALING: Flags = Flags(MFD_ALLOW_SEALING);
    /// `MFD_HUGETLB`
    pub const HUGETLB: Flags = Flags(MFD_HUGETLB);
    /// `MFD_NOEXEC_SEAL`
    pub const NOEXEC_SEAL: Flags = Flags(MFD_NOEXEC_SEAL);
    /// `MFD_EXEC`
    pub const EXEC: Flags = Flags(MFD_EXEC);
}

// ===== Error =====

/// An error that may occur when creating [`Memfd`].
#[derive(Copy, Clone)]
pub struct Error(ErrCode);

error::impl_error_os_simple!(Error, "create memfd");

// ===== extern =====

// include/uapi/linux/memfd.h

const MFD_CLOEXEC: u32 = 0x0001;
const MFD_ALLOW_SEALING: u32 = 0x0002;
const MFD_HUGETLB: u32 = 0x0004;
const MFD_NOEXEC_SEAL: u32 = 0x0008;
const MFD_EXEC: u32 = 0x0010;

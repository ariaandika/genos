//! [`Eventfd`] associated types.
use crate::fd::{self, OwnedFd};
use crate::flags;
use crate::sys::{self, Error, arch};

// ===== Eventfd =====

/// Wait/notify mechanism.
#[derive(Debug)]
pub struct Eventfd(OwnedFd);

fd::impl_fd_simple!(Eventfd);

impl Eventfd {
    /// Creates new [`Eventfd`] (`eventfd(2)`).
    #[inline]
    pub fn create(initval: u32, flags: i32) -> Result<Self, Error<arch::sys_eventfd2>> {
        sys::call_rd!(sys_eventfd2, initval, flags)
    }
}

// ===== Flags =====

/// [`Eventfd`] creation flags.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Flags(i32);

flags::impl_bitops_simple!(Flags);

impl Eventfd {
    /// `EFD_SEMAPHORE`
    pub const SEMAPHORE: Flags = Flags(EFD_SEMAPHORE);
    /// `EFD_CLOEXEC`
    pub const CLOEXEC: Flags = Flags(EFD_CLOEXEC);
    /// `EFD_NONBLOCK`
    pub const NONBLOCK: Flags = Flags(EFD_NONBLOCK);
}

// ===== extern =====

// include/uapi/linux/eventfd.h

const EFD_SEMAPHORE: i32 = 1 << 0;
const EFD_CLOEXEC: i32 = fd::O_CLOEXEC;
const EFD_NONBLOCK: i32 = fd::O_NONBLOCK;

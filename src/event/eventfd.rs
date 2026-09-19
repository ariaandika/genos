//! [`Eventfd`] associated types.
use core::mem::MaybeUninit;

use crate::fd::{self, AsFd, OwnedFd};
use crate::sys::{self, SysRes};
use crate::{flags, io};

// ===== Eventfd =====

/// Wait/notify mechanism.
#[derive(Debug)]
pub struct Eventfd(OwnedFd);

fd::impl_fd_simple!(Eventfd);

impl Eventfd {
    /// Creates new [`Eventfd`] (`eventfd(2)`).
    #[inline]
    pub fn create(initval: u32, flags: i32) -> impl SysRes<Self> {
        sys::call_rd!(sys_eventfd2, initval, flags)
    }

    /// Read and consume the counter to given buffer.
    #[inline]
    pub fn read_to(&self, buf: &mut MaybeUninit<u64>) -> impl SysRes<()> {
        io::read_raw(self.as_raw_fd(), buf, size_of::<u64>()).drop()
    }

    /// Read, consume, and returns the counter.
    #[inline]
    pub fn read(&self) -> impl SysRes<u64> {
        let mut buf = MaybeUninit::<u64>::uninit();
        io::read_raw(self.as_raw_fd(), buf.as_mut_ptr(), size_of::<u64>())
            .map(|_| unsafe { buf.assume_init() as _ })
    }

    /// Add the counter by `add`.
    #[inline]
    pub fn write(&self, add: u64) -> impl SysRes<()> {
        io::write_raw(self.as_raw_fd(), &add, size_of::<u64>()).drop()
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

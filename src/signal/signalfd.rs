//! [`Signalfd`] associated types.
use core::mem::MaybeUninit;

use crate::fd::{AsFd, OwnedFd};
use crate::signal::{Signo, Sigset};
use crate::sys::{SysRes, SysResRaw, arch};
use crate::{fd, flags, io, sys};

// ===== Signalfd =====

/// File descriptor for accepting signals.
#[derive(Debug)]
pub struct Signalfd(OwnedFd);

fd::impl_fd_simple!(Signalfd);

impl Signalfd {
    /// Create new [`Signalfd`] (`signalfd(2)`).
    #[inline]
    pub fn create(sigset: &Sigset, flags: Flags) -> impl SysRes<Self> {
        signalfd(-1, sigset, flags)
    }

    /// Replace the associated signal set (`signalfd(2)`).
    #[inline]
    pub fn set_signal(&self, sigset: &Sigset) -> impl SysRes<()> {
        signalfd(self.as_raw_fd(), sigset, <_>::default())
    }

    /// Read and consume caught pending signals.
    ///
    /// Returns the number of `Siginfo` struct returned.
    #[inline]
    pub fn read(&self, buf: &mut [MaybeUninit<Siginfo>]) -> impl SysRes<usize> {
        io::read_raw(self.as_raw_fd(), buf.as_mut_ptr(), size_of_val(buf))
            .map(|read| (read as usize).wrapping_div(size_of::<Siginfo>()) as _)
    }
}

fn signalfd<T>(fd: i32, sigset: &Sigset, flags: Flags) -> SysResRaw<T, arch::sys_signalfd> {
    sys::call_rd!(sys_signalfd, fd, sigset.as_ref(), flags.0)
}

// ===== Siginfo =====

/// Pending signal information (`signalfd(2)`).
#[derive(Debug, Clone)]
#[repr(C)]
pub struct Siginfo {
    /// `signalfd_siginfo.ssi_signo`
    pub signo: u32,
    /// `signalfd_siginfo.ssi_errno`
    pub errno: i32,
    /// `signalfd_siginfo.ssi_code`
    pub code: i32,
    /// `signalfd_siginfo.ssi_pid`
    pub pid: u32,
    /// `signalfd_siginfo.ssi_uid`
    pub uid: u32,
    /// `signalfd_siginfo.ssi_fd`
    pub fd: i32,
    /// `signalfd_siginfo.ssi_tid`
    pub tid: u32,
    /// `signalfd_siginfo.ssi_band`
    pub band: u32,
    /// `signalfd_siginfo.ssi_overrun`
    pub overrun: u32,
    /// `signalfd_siginfo.ssi_trapno`
    pub trapno: u32,
    /// `signalfd_siginfo.ssi_status`
    pub status: i32,
    /// `signalfd_siginfo.ssi_int`
    pub int: i32,
    /// `signalfd_siginfo.ssi_ptr`
    pub ptr: u64,
    /// `signalfd_siginfo.ssi_utime`
    pub utime: u64,
    /// `signalfd_siginfo.ssi_stime`
    pub stime: u64,
    /// `signalfd_siginfo.ssi_addr`
    pub addr: u64,
    /// `signalfd_siginfo.ssi_addr_lsb`
    pub addr_lsb: u16,
    __pad: [u8; 46],
}

impl Siginfo {
    /// Returns the pending [`Signo`].
    #[inline]
    pub fn signo(&self) -> Signo {
        Signo::from_raw(self.signo)
    }
}

// ===== Flags =====

/// [`Signalfd`] flags.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Flags(i32);

flags::impl_bitops_simple!(Flags);

impl Signalfd {
    /// Set the close-on-exec (FD_CLOEXEC) flag on the new fd.
    pub const CLOEXEC: Flags = Flags(SFD_CLOEXEC);
    /// Set the `O_NONBLOCK` file status flag on the new fd.
    pub const NONBLOCK: Flags = Flags(SFD_NONBLOCK);
}

// ===== extern =====

// include/uapi/linux/signalfd.h

const SFD_CLOEXEC: i32 = fd::O_CLOEXEC;
const SFD_NONBLOCK: i32 = fd::O_NONBLOCK;

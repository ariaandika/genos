//! [`Signalfd`] associated types.
use crate::fd::{AsFd, FromRawFd, OwnedFd};
use crate::signal::{Signo, Sigset};
use crate::sys::{Error, arch};
use crate::{fd, flags, sys};

// ===== Signalfd =====

/// File descriptor for accepting signals.
#[derive(Debug)]
pub struct Signalfd(OwnedFd);

fd::impl_fd_simple!(Signalfd);

impl Signalfd {
    /// Create new [`Signalfd`] (`signalfd(2)`).
    #[inline]
    pub fn create(sigset: &Sigset, flags: Flags) -> Result<Self, Error<arch::sys_signalfd4>> {
        signalfd(-1, sigset, flags).map(|fd| unsafe { <_>::from_raw_fd(fd) })
    }

    /// Replace the associated signal set (`signalfd(2)`).
    #[inline]
    pub fn set_signal(&self, sigset: &Sigset) -> Result<(), Error<arch::sys_signalfd4>> {
        signalfd(self.as_raw_fd(), sigset, <_>::default()).map(drop)
    }
}

fn signalfd(fd: i32, sigset: &Sigset, flags: Flags) -> Result<i32, Error<arch::sys_signalfd4>> {
    sys::call_rd!(sys_signalfd4, fd, sigset.as_ref(), flags.0)
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

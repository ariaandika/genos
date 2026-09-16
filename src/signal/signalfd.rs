//! [`Signalfd`] associated types.
use core::{fmt, result};

use crate::error::{ErrCode, SysResExt};
use crate::fd::{AsFd, Open, OwnedFd};
use crate::signal::{Signo, Sigset};
use crate::{error, fd, flags, sys};

// ===== Signalfd =====

/// File descriptor for accepting signals.
#[derive(Debug)]
pub struct Signalfd(OwnedFd);

fd::impl_fd_simple!(Signalfd);

impl Signalfd {
    /// Create new [`Signalfd`] (`signalfd(2)`).
    #[inline]
    pub fn create(sigset: &Sigset, flags: Flags) -> Result<Self> {
        signalfd(-1, sigset, flags).fd(Kind::Create)
    }

    /// Replace the associated signal set (`signalfd(2)`).
    #[inline]
    pub fn set_signal(&self, sigset: &Sigset) -> Result<()> {
        signalfd(self.as_raw_fd(), sigset, <_>::default()).e(Kind::Set)
    }
}

fn signalfd(fd: i32, sigset: &Sigset, flags: Flags) -> impl SysResExt {
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

/// [`Signalfd`] creation flags.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Flags(i32);

flags::impl_bitops_simple!(Flags);

impl Flags {
    /// Set the close-on-exec (FD_CLOEXEC) flag on the new fd.
    pub const CLOEXEC: Self = Self(SFD_CLOEXEC);
    /// Set the `O_NONBLOCK` file status flag on the new fd.
    pub const NONBLOCK: Self = Self(SFD_NONBLOCK);
}

// ===== Error =====

/// Type alias for result of [`Signalfd`] operations.
pub type Result<T, E = Error> = result::Result<T, E>;

/// An error that may occur during any [`Signalfd`] operations.
#[derive(Debug, Clone)]
pub struct Error {
    kind: Kind,
    code: ErrCode,
}

#[derive(Debug, Clone, Copy)]
enum Kind {
    Create,
    Set,
}

error::impl_error_with_kind!(Error, Kind);

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self { kind, code } = self;
        let msg = match kind {
            Kind::Create => "create signalfd",
            Kind::Set => "set signalfd sigset",
        };
        write!(f, "failed to {msg}: {code}")
    }
}

// ===== extern =====

// include/uapi/linux/signalfd.h

const SFD_CLOEXEC: i32 = Open::CLOEXEC.raw();
const SFD_NONBLOCK: i32 = Open::NONBLOCK.raw();

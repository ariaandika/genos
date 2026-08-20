//! [`Signalfd`] associated types.
use core::mem::MaybeUninit;
use core::{fmt, result};

use crate::error::{ErrCode, SysResExt};
use crate::fd::{AsFd, OwnedFd};
use crate::signal::{Signo, Sigset};
use crate::{error, fd, flags, sys};

// ===== Signalfd =====

/// File descriptor for accepting signals.
#[derive(Debug)]
pub struct Signalfd(OwnedFd);

fd::impl_fd_simple!(Signalfd);

impl Signalfd {
    /// Create new [`Signalfd`].
    #[inline]
    pub fn new(sigset: &Sigset, flags: Flags) -> Result<Self> {
        sys::call!(__NR_signalfd, -1, sigset.as_ref(), flags.0).fd(Kind::Create)
    }

    /// Read for pending signal.
    #[inline]
    pub fn read(&self) -> Result<Siginfo> {
        const LEN: usize = size_of::<Siginfo>();
        let mut buf = MaybeUninit::<Siginfo>::uninit();
        let mut n = 0;
        while let Some(rem) = LEN.checked_sub(n)
            && rem != 0
        {
            let ptr = unsafe { buf.as_mut_ptr().byte_add(n) };
            let read = sys::call!(__NR_read, self.as_fd(), ptr, rem).io(Kind::Read)?;
            n += read;
        }
        Ok(unsafe { buf.assume_init() })
    }
}

// ===== Siginfo =====

/// Pending signal information.
#[derive(Debug, Clone)]
#[repr(transparent)]
pub struct Siginfo(sys::signalfd_siginfo);

impl Siginfo {
    /// Returns the pending [`Signo`].
    #[inline]
    pub fn signo(&self) -> Signo {
        Signo::from_raw(self.0.ssi_signo)
    }
}

// ===== Flags =====

/// [`Signalfd`] creation flags.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Flags(i32);

impl Flags {
    /// Set the close-on-exec (FD_CLOEXEC) flag on the new fd.
    pub const CLOEXEC: Self = Self(sys::SFD_CLOEXEC);
    /// Set the `O_NONBLOCK` file status flag on the new fd.
    pub const NONBLOCK: Self = Self(sys::SFD_NONBLOCK);
}

impl flags::OpenFlag for Flags {
    const CLOEXEC: Self = Self::CLOEXEC;
    const NONBLOCK: Self = Self::NONBLOCK;
}

flags::impl_bitops_simple!(Flags);

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
    Read,
}

error::impl_error_with_kind!(Error, Kind);

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self { kind, code } = self;
        let msg = match kind {
            Kind::Create => "create signalfd",
            Kind::Read => "read signalfd pending signal",
        };
        write!(f, "failed to {msg}: {code}")
    }
}

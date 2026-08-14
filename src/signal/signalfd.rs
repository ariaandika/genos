//! [`Signalfd`] associated types.
use core::mem::MaybeUninit;
use core::task::Poll;

use crate::error::{ErrCode, FromErrCode, os_error_simple};
use crate::fd::{AsRawFd, OwnedFd, impl_fd_simple};
use crate::flags::impl_bitops_simple;
use crate::net::OpenFlag;
use crate::signal::{Signo, Sigset};

// ===== Signalfd =====

/// File descriptor for accepting signals.
#[derive(Debug)]
pub struct Signalfd(OwnedFd);

impl_fd_simple!(Signalfd);

impl Signalfd {
    /// Create new [`Signalfd`].
    #[inline]
    pub fn new(sigset: &Sigset) -> Result<Self, CreateError> {
        unsafe { <_>::fd(libc::signalfd(-1, sigset.as_ref(), 0)) }
    }

    /// Read for pending signal.
    #[inline]
    pub fn read(&self) -> Result<Siginfo, CreateError> {
        const LEN: usize = size_of::<Siginfo>();
        let mut buf = MaybeUninit::<Siginfo>::uninit();
        let mut n = 0;
        while let Some(rem) = LEN.checked_sub(n).filter(|e| *e != 0) {
            let read = unsafe {
                let ptr = buf.as_mut_ptr().byte_add(n).cast();
                libc::read(self.as_raw_fd(), ptr, rem)
            };
            let Ok(read) = usize::try_from(read) else {
                return Err(<_>::errno());
            };
            n += read;
        }
        Ok(unsafe { buf.assume_init() })
    }

    /// Poll read for pending signal.
    #[inline]
    pub fn poll_read(&self) -> Poll<Result<Siginfo, CreateError>> {
        <_>::ep(Self::read(self))
    }
}

// ===== Siginfo =====

/// Pending signal information.
#[derive(Debug, Clone)]
#[repr(transparent)]
pub struct Siginfo(libc::signalfd_siginfo);

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

// `/usr/include/bits/socket.h`
impl Flags {
    /// Set the close-on-exec (FD_CLOEXEC) flag on the new fd.
    pub const CLOEXEC: Self = Self(libc::SFD_CLOEXEC);
    /// Set the `O_NONBLOCK` file status flag on the new fd.
    pub const NONBLOCK: Self = Self(libc::SFD_NONBLOCK);
}

impl OpenFlag for Flags {
    const CLOEXEC: Self = Self::CLOEXEC;
    const NONBLOCK: Self = Self::NONBLOCK;
}

impl_bitops_simple!(Flags);

// ===== errors =====

/// An error that may occur when creating signalfd.
#[derive(Clone, Copy)]
pub struct CreateError(ErrCode);

os_error_simple!(CreateError, "create signalfd");

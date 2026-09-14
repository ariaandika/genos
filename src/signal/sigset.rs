//! [`Sigset`] associated types.
use core::result;

use crate::error::{ErrCode, SysResExt};
use crate::signal::Signo;
use crate::{error, sys};

// ===== Sigset =====

/// Signal set.
#[derive(Debug, Clone)]
#[repr(transparent)]
pub struct Sigset(sys::sigset_t);

impl Sigset {
    /// Creates new empty [`Sigset`].
    #[inline]
    pub const fn new() -> Self {
        Self(sys::sigemptyset())
    }

    /// Creates new full [`Sigset`], including all signal.
    #[inline]
    pub const fn fill() -> Self {
        Self(sys::sigfillset())
    }

    pub(crate) fn as_ref(&self) -> &sys::sigset_t {
        &self.0
    }

    /// Add signal in the set.
    #[inline]
    pub const fn add(&mut self, sig: Signo) {
        sys::sigaddset(&mut self.0, sig.as_raw());
    }

    /// Delete signal in the set.
    #[inline]
    pub const fn delete(&mut self, sig: Signo) {
        sys::sigdelset(&mut self.0, sig.as_raw());
    }

    /// Returns `true` if given signo is a member of the set.
    #[inline]
    pub const fn is_member(&self, sig: Signo) -> bool {
        sys::sigismember(&self.0, sig.as_raw()) != 0
    }

    /// Set the blocked signals to the union of the current set and this set.
    #[inline]
    pub fn block(&self) -> Result<()> {
        self.rt_sigprocmask(sys::SIG_BLOCK)
    }

    /// Remove the blocked signals that is in this set.
    #[inline]
    pub fn unblock(&self) -> Result<()> {
        self.rt_sigprocmask(sys::SIG_UNBLOCK)
    }

    /// Set the blocked signals to this set.
    #[inline]
    pub fn setmask(&self) -> Result<()> {
        self.rt_sigprocmask(sys::SIG_SETMASK)
    }

    #[inline]
    fn rt_sigprocmask(&self, how: i32) -> Result<()> {
        sys::call_rd!(sys_rt_sigprocmask, how, self, 0).e2()
    }
}

// ===== Error =====

/// The result of changing blocked signals.
pub type Result<T, E = ProcSignalError> = result::Result<T, E>;

/// An error that may occur when changing blocked signals.
#[derive(Clone, Copy)]
pub struct ProcSignalError(ErrCode);

error::impl_error_os_simple!(ProcSignalError, "change blocked signals");

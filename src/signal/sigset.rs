use core::mem::MaybeUninit;

use crate::error::{ErrCode, FromErrCode, os_error_simple};
use crate::signal::Signo;

// ===== Sigset =====

/// Signal set.
#[derive(Debug, Clone)]
#[repr(transparent)]
pub struct Sigset(libc::sigset_t);

impl Sigset {
    /// Creates new empty [`Sigset`].
    #[inline]
    pub fn new() -> Self {
        let mut set = MaybeUninit::<Self>::uninit();
        unsafe {
            libc::sigemptyset(set.as_mut_ptr().cast());
            set.assume_init()
        }
    }

    /// Creates new full [`Sigset`], including all signal.
    #[inline]
    pub fn fill() -> Self {
        let mut set = MaybeUninit::<Self>::uninit();
        unsafe {
            libc::sigfillset(set.as_mut_ptr().cast());
            set.assume_init()
        }
    }

    pub(crate) fn as_ref(&self) -> &libc::sigset_t {
        &self.0
    }

    fn as_mut(&mut self) -> &mut libc::sigset_t {
        &mut self.0
    }

    /// Add signal in the set.
    #[inline]
    pub fn add(&mut self, sig: Signo) {
        unsafe { libc::sigaddset(self.as_mut(), sig.into()) };
    }

    /// Delete signal in the set.
    #[inline]
    pub fn delete(&mut self, sig: Signo) {
        unsafe { libc::sigdelset(self.as_mut(), sig.into()) };
    }

    /// Returns `true` if given signo is a member of the set.
    #[inline]
    pub fn is_member(&self, sig: Signo) -> bool {
        unsafe { libc::sigismember(self.as_ref(), sig.into()) == 1 }
    }

    /// Set the blocked signals to the union of the current set and this set.
    #[inline]
    pub fn block(&self) -> Result<(), ProcSignalError> {
        unsafe { <_>::e(libc::sigprocmask(libc::SIG_BLOCK, self.as_ref(), 0 as _)) }
    }

    /// Remove the blocked signals that is in this set.
    #[inline]
    pub fn unblock(&self) -> Result<(), ProcSignalError> {
        unsafe { <_>::e(libc::sigprocmask(libc::SIG_UNBLOCK, self.as_ref(), 0 as _)) }
    }

    /// Set the blocked signals to this set.
    #[inline]
    pub fn setmask(&self) -> Result<(), ProcSignalError> {
        unsafe { <_>::e(libc::sigprocmask(libc::SIG_SETMASK, self.as_ref(), 0 as _)) }
    }
}

// ===== Error =====

/// An error that may occur when changing blocked signals.
#[derive(Clone, Copy)]
pub struct ProcSignalError(ErrCode);

os_error_simple!(ProcSignalError, "change blocked signals");

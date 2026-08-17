use core::num::NonZeroI32;

use crate::error::{ErrCode, FromErrCode, os_error_simple};
use crate::signal::Signo;

/// Process handle.
#[derive(Debug, Clone)]
pub struct Process(NonZeroI32);

impl Process {
    /// Create [`Process`] referencing this process.
    #[inline]
    pub fn this() -> Self {
        Self(unsafe { NonZeroI32::new_unchecked(libc::getpid()) })
    }

    /// Create [`Process`] referencing parent process.
    ///
    /// Returns `None` if parent is in a different PID namespace.
    #[inline]
    pub fn parent() -> Option<Self> {
        NonZeroI32::new(unsafe { libc::getppid() }).map(Self)
    }

    /// Create child process by duplicating the calling process.
    ///
    /// Returns `None` if this execution is in the child process.
    #[inline]
    pub fn fork() -> Option<Process> {
        NonZeroI32::new(unsafe { libc::fork() }).map(Self)
    }

    /// Returns the process id.
    #[inline]
    pub fn id(&self) -> i32 {
        self.0.get()
    }

    /// Send a signal to process this struct refers to.
    #[inline]
    pub fn kill(&self, sig: Signo) -> Result<(), KillError> {
        unsafe { <_>::e(libc::kill(self.0.get(), sig.into())) }
    }
}

// ===== errors =====

/// An error that may occur when sending signal to a process.
#[derive(Clone, Copy)]
pub struct KillError(ErrCode);

os_error_simple!(KillError, "send signal to process");

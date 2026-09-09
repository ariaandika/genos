use core::ffi::CStr;
use core::num::NonZeroI32;

use crate::error::{ErrCode, SysResExt};
use crate::ffi::Char;
use crate::signal::Signo;
use crate::{error, sys};

/// Terminate process with given status code.
#[inline]
pub fn exit(status: i32) -> ! {
    sys::call!(NORETURN, sys_exit, status)
}

/// Executes the program referred to by path.
#[inline]
pub fn execve(path: &CStr, argv: &Option<&Char>, envp: &Option<&Char>) -> ErrCode {
    ErrCode::new(-sys::call!(RD, sys_execve, path, argv, envp).into_inner() as _)
}

/// Process handle.
#[derive(Debug, Clone)]
pub struct Process(NonZeroI32);

impl Process {
    /// Create [`Process`] referencing caller process.
    #[inline]
    pub fn this() -> Self {
        let pid = sys::call!(RD, sys_getpid).into_inner();
        // SAFETY: have faith from the kernel
        Self(unsafe { NonZeroI32::new_unchecked(pid as _) })
    }

    /// Create [`Process`] referencing callers parent process.
    ///
    /// Returns `None` if parent is in a different PID namespace.
    #[inline]
    pub fn parent() -> Option<Self> {
        let ppid = sys::call!(RD, sys_getppid).into_inner();
        NonZeroI32::new(ppid as _).map(Self)
    }

    /// Create child process by duplicating the calling process.
    ///
    /// Returns `None` if this execution is in the child process.
    #[inline]
    pub fn fork() -> Result<Option<Process>, ForkError> {
        sys::call!(RD, sys_fork)
            .io2()
            .map(|pid| NonZeroI32::new(pid as _).map(Self))
    }

    /// Returns the process id.
    #[inline]
    pub fn id(&self) -> i32 {
        self.0.get()
    }

    /// Send a signal to process this struct refers to.
    #[inline]
    pub fn kill(&self, sig: Signo) -> Result<(), KillError> {
        sys::call!(RD, sys_kill, self.0.get(), i32::from(sig)).e2()
    }
}

// ===== errors =====

/// An error that may occur when `fork`-ing a process.
#[derive(Clone, Copy)]
pub struct ForkError(ErrCode);

error::impl_error_os_simple!(ForkError, "fork process");

/// An error that may occur when sending signal to a process.
#[derive(Clone, Copy)]
pub struct KillError(ErrCode);

error::impl_error_os_simple!(KillError, "send signal to process");

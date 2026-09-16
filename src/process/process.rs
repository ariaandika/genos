use core::ffi::CStr;

use crate::error::{ErrCode, SysResExt};
use crate::ffi::{Char, Pid};
use crate::signal::Signo;
use crate::{error, sys};

/// Returns process ID (PID) of the calling process (`getpid(2)`).
#[inline]
pub fn getpid() -> Pid {
    sys::call_rd!(sys_getpid).into_inner() as Pid
}

/// Returns process ID (PID) of the parent of the calling process (`getppid(2)`).
#[inline]
pub fn getppid() -> Pid {
    sys::call_rd!(sys_getppid).into_inner() as Pid
}

/// Create child process by duplicating the calling process (`fork(2)`).
#[inline]
pub fn fork() -> Result<Pid, ForkError> {
    sys::call_rd!(sys_fork).io2().map(|e| e as _)
}

/// Executes the program referred to by path (`execve(2)`).
#[inline]
pub fn execve(path: &CStr, argv: &Option<&Char>, envp: &Option<&Char>) -> ErrCode {
    ErrCode::sys(sys::call_rd!(sys_execve, path, argv, envp).into_inner() as _)
}

/// Send a signal to process this struct refers to (`kill(2)`).
#[inline]
pub fn kill(pid: Pid, sig: Signo) -> Result<(), KillError> {
    sys::call_rd!(sys_kill, pid, i32::from(sig)).e2()
}

/// Terminate process with given status code (`exit(2)`).
#[inline]
pub fn exit(status: i32) -> ! {
    unsafe { sys::call1_noret(sys::sys_exit, status as usize) }
}

// ===== errors =====

/// An error that may occur during [`fork`] operation.
#[derive(Clone, Copy)]
pub struct ForkError(ErrCode);

error::impl_error_os_simple!(ForkError, "fork process");

/// An error that may occur during [`kill`] operation.
#[derive(Clone, Copy)]
pub struct KillError(ErrCode);

error::impl_error_os_simple!(KillError, "send signal to process");

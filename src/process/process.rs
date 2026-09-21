use crate::ffi::{Char, Pid};
use crate::signal::Signo;
use crate::sys::{self, Error, arch};

/// Returns process ID (PID) of the calling process (`getpid(2)`).
#[inline]
pub fn getpid() -> Pid {
    sys::call_rd_raw!(sys_getpid) as Pid
}

/// Returns process ID (PID) of the parent of the calling process (`getppid(2)`).
#[inline]
pub fn getppid() -> Pid {
    sys::call_rd_raw!(sys_getppid) as Pid
}

/// Create child process by duplicating the calling process (`fork(2)`).
#[inline]
pub fn fork() -> Result<Pid, Error<arch::sys_fork>> {
    sys::call_rd!(sys_fork)
}

/// Executes the program referred to by path (`execve(2)`).
#[inline]
pub fn execve(path: &Char, argv: &Option<&Char>, envp: &Option<&Char>) -> Error<arch::sys_execve> {
    let res = sys::call_rd_raw!(sys_execve, path, argv, envp);
    // SAFETY: if `execve` returns, its an error
    unsafe { Error::from_infallible(res) }
}

/// Send a signal to process this struct refers to (`kill(2)`).
#[inline]
pub fn kill(pid: Pid, sig: Signo) -> Result<(), Error<arch::sys_kill>> {
    sys::call_rd!(sys_kill, pid, i32::from(sig))
}

/// Terminate process with given status code (`exit(2)`).
#[inline]
pub fn exit(status: i32) -> ! {
    unsafe { arch::call1_noret(<arch::sys_exit as sys::SysId>::NO, status) }
}

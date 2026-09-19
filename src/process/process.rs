use crate::error::ErrCode;
use crate::ffi::{Char, Pid};
use crate::signal::Signo;
use crate::sys::{self, SysRes, arch};

/// Returns process ID (PID) of the calling process (`getpid(2)`).
#[inline]
pub fn getpid() -> Pid {
    SysRes::<()>::into_raw(sys::call_rd!(sys_getpid)) as Pid
}

/// Returns process ID (PID) of the parent of the calling process (`getppid(2)`).
#[inline]
pub fn getppid() -> Pid {
    SysRes::<()>::into_raw(sys::call_rd!(sys_getppid)) as Pid
}

/// Create child process by duplicating the calling process (`fork(2)`).
#[inline]
pub fn fork() -> impl SysRes<Pid> {
    sys::call_rd!(sys_fork)
}

/// Executes the program referred to by path (`execve(2)`).
#[inline]
pub fn execve(path: &Char, argv: &Option<&Char>, envp: &Option<&Char>) -> ErrCode {
    ErrCode::sys(SysRes::<()>::into_raw(sys::call_rd!(sys_execve, path, argv, envp)) as _)
}

/// Send a signal to process this struct refers to (`kill(2)`).
#[inline]
pub fn kill(pid: Pid, sig: Signo) -> impl SysRes<()> {
    sys::call_rd!(sys_kill, pid, i32::from(sig))
}

/// Terminate process with given status code (`exit(2)`).
#[inline]
pub fn exit(status: i32) -> ! {
    unsafe { arch::call1_noret(arch::sys_exit::NO, status) }
}

//! Process management.
pub use process::{KillError, Process};

mod process;
mod main;

/// Terminate process with given status code.
#[inline]
pub fn _exit(status: i32) -> ! {
    crate::sys::call!(NORETURN, __NR_exit, status)
}

//! Process management.
pub use args::{Args, args, raw_args};
pub use process::{KillError, Process};

mod args;
mod process;
mod main;

/// Terminate process with given status code.
#[inline]
pub fn _exit(status: i32) -> ! {
    unsafe { libc::_exit(status) }
}

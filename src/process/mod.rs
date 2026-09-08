//! Process management.
pub use process::{_exit, KillError, Process, execve};

mod process;

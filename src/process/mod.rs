//! Process management.
pub use process::{exit, KillError, Process, execve};

mod process;

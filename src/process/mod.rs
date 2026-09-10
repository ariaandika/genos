//! Process management.
pub use process::{KillError, Process, execve, exit};
pub use clone::{CloneArgs, CloneError, CloneFlags, clone, clone3};

mod clone;
mod process;

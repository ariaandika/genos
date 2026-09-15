//! Process management.
pub use process::{KillError, Process, execve, exit};
#[doc(inline)]
pub use clone::{CloneArgs, clone};

mod process;
pub mod clone;

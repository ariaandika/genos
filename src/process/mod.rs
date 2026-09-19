//! Process management.
pub use process::{execve, exit, fork, getpid, getppid, kill};
#[doc(inline)]
pub use clone::{CloneArgs, clone};

mod process;
pub mod clone;

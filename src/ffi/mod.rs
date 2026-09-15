//! Foreign interface.
pub use nulchar::Char;
pub use posix::{ClockID, Gid, Mode, Off, Pid, Timer, Uid};

mod nulchar;
mod posix;

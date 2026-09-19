//! Foreign interface.
pub use nulchar::Char;
pub use posix::{ClockID, Gid, Mode, Off, Pid, Time, Timer, Uid};

mod nulchar;
mod posix;

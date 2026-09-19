use core::fmt;

use crate::fd::{AsFd, BorrowedFd};
use crate::io;
use crate::sys::SysRes;

/// Standard input stream (`stdin(3)`).
#[derive(Debug)]
pub struct Stdin;

/// Standard output stream (`stdout(3)`).
#[derive(Debug)]
pub struct Stdout;

/// Standard error stream (`stderr(3)`).
#[derive(Debug)]
pub struct Stderr;

macro_rules! impl_stream {
    ($me:ident, $fd:expr $(, $write:path)?) => {
        impl AsFd for $me {
            #[inline]
            fn as_fd(&self) -> BorrowedFd<'_> {
                unsafe { BorrowedFd::borrow_raw($fd) }
            }
        }
        $(
            impl $write for $me {
                #[inline]
                fn write_str(&mut self, s: &str) -> fmt::Result {
                    let res = io::write(self, s.as_bytes()).into_raw();
                    if res >= 0 { Ok(()) } else { Err(fmt::Error) }
                }
            }
        )?
    };
}
impl_stream!(Stdin, 0);
impl_stream!(Stdout, 1, fmt::Write);
impl_stream!(Stderr, 2, fmt::Write);

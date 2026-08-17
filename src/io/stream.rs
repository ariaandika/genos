use core::fmt;

use crate::fd::{AsFd, BorrowedFd};
use crate::io::{Read, ReadError, Write, WriteError};

/// Standard in stream.
#[derive(Debug)]
pub struct Stdin;

impl AsFd for Stdin {
    #[inline]
    fn as_fd(&self) -> BorrowedFd<'_> {
        unsafe { BorrowedFd::borrow_raw(0) }
    }
}

impl Read for Stdin {
    type Error = ReadError;
}

/// Standard out stream.
#[derive(Debug)]
pub struct Stdout;

impl AsFd for Stdout {
    #[inline]
    fn as_fd(&self) -> BorrowedFd<'_> {
        unsafe { BorrowedFd::borrow_raw(1) }
    }
}

impl Write for Stdout {
    type Error = WriteError;
}

impl fmt::Write for Stdout {
    #[inline]
    fn write_str(&mut self, s: &str) -> fmt::Result {
        match Write::write(self, s.as_bytes()) {
            Ok(_) => Ok(()),
            Err(_) => Err(fmt::Error),
        }
    }
}

/// Standard err stream.
#[derive(Debug)]
pub struct Stderr;

impl AsFd for Stderr {
    #[inline]
    fn as_fd(&self) -> BorrowedFd<'_> {
        unsafe { BorrowedFd::borrow_raw(2) }
    }
}

impl Write for Stderr {
    type Error = WriteError;
}

impl fmt::Write for Stderr {
    #[inline]
    fn write_str(&mut self, s: &str) -> fmt::Result {
        match Write::write(self, s.as_bytes()) {
            Ok(_) => Ok(()),
            Err(_) => Err(fmt::Error),
        }
    }
}

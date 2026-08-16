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

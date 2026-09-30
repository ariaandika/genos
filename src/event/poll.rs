//! [`poll`] associated types.
use core::{ffi, slice};

use crate::fd::{AsFd, BorrowedFd};
use crate::flags;
use crate::sys::{self, arch};

/// Wait for some event on a file descriptor (`poll(2)`).
#[inline]
pub fn poll(fds: &mut [Pollfd], timeout: i32) -> Result<usize, sys::Error<arch::sys_poll>> {
    sys::call!(sys_poll, fds.as_mut_ptr(), fds.len(), timeout)
}

// ===== Pollfd =====

/// File descriptor entry.
#[derive(Debug)]
#[repr(C)]
pub struct Pollfd<'a> {
    /// Target file descriptor.
    pub fd: BorrowedFd<'a>,
    /// Requested events.
    pub events: EventType,
    /// Returned events.
    pub revents: EventType,
}

impl AsFd for Pollfd<'_> {
    #[inline]
    fn as_fd(&self) -> BorrowedFd<'_> {
        self.fd.as_fd()
    }
}

impl<'a> Pollfd<'a> {
    /// Creates new [`Pollfd`] with given fd.
    #[inline]
    pub fn new<Fd: AsFd + ?Sized>(fd: &'a Fd, events: EventType) -> Self {
        Self { fd: fd.as_fd(), events, revents: <_>::default() }
    }

    /// Wait for some event on this fd.
    ///
    /// This is purely convinient method that calls [`poll`] with single element slice.
    #[inline]
    pub fn poll(&mut self, timeout: i32) -> Result<usize, sys::Error<arch::sys_poll>> {
        poll(slice::from_mut(self), timeout)
    }
}

// ===== EventType =====

/// [`Pollfd`] event flags.
#[derive(Debug, Default, Clone, Copy)]
#[repr(transparent)]
pub struct EventType(ffi::c_short);

flags::impl_bitops_simple!(EventType);

impl EventType {
    /// Returns `true` if event contains `other`.
    #[inline]
    pub const fn contains(self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}

impl Pollfd<'_> {
    /// `POLLIN`
    pub const IN: EventType = EventType(POLLIN);
    /// `POLLPRI`
    pub const PRI: EventType = EventType(POLLPRI);
    /// `POLLOUT`
    pub const OUT: EventType = EventType(POLLOUT);
    /// `POLLERR`
    pub const ERR: EventType = EventType(POLLERR);
    /// `POLLHUP`
    pub const HUP: EventType = EventType(POLLHUP);
    /// `POLLNVAL`
    pub const NVAL: EventType = EventType(POLLNVAL);
    /// `POLLRDHUP`
    pub const RDHUP: EventType = EventType(POLLRDHUP);
}

// ===== extern =====

// include/uapi/asm-generic/poll.h

const POLLIN: ffi::c_short = 0x0001;
const POLLPRI: ffi::c_short = 0x0002;
const POLLOUT: ffi::c_short = 0x0004;
const POLLERR: ffi::c_short = 0x0008;
const POLLHUP: ffi::c_short = 0x0010;
const POLLNVAL: ffi::c_short = 0x0020;
const POLLRDHUP: ffi::c_short = 0x2000;

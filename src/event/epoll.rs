//! [`Epoll`] associated types.
use core::mem::MaybeUninit;
use core::{fmt, result};

use crate::error::{ErrCode, SysResExt};
use crate::fd::{AsFd, OwnedFd};
use crate::{error, fd, flags, sys};

// ===== Epoll =====

/// I/O event notification facility.
///
/// See `epoll(7)`.
#[derive(Debug)]
pub struct Epoll(OwnedFd);

fd::impl_fd_simple!(Epoll);

impl Epoll {
    /// Creates new [`Epoll`].
    ///
    /// Reference: `epoll_create1(2)`.
    #[inline]
    pub fn create(flags: Flags) -> Result<Self> {
        sys::call!(RD, epoll_create1, flags.0).fd(Kind::Create)
    }

    /// Add an entry to the interest list.
    ///
    /// [`InputFlags`] can be added by `OR`-ing with [`EventType`].
    ///
    /// Reference: `epoll_ctl(2)`.
    #[inline]
    pub fn add<Fd: AsFd>(&self, fd: &Fd, events: EventType, data: u64) -> Result<()> {
        self.epoll_ctl(sys::EPOLL_CTL_ADD, fd, &Event { events, data }, Kind::Add)
    }

    /// Change the settings associated with fd in the interest list.
    ///
    /// [`InputFlags`] can be added by `OR`-ing with [`EventType`].
    ///
    /// Reference: `epoll_ctl(2)`.
    #[inline]
    pub fn modify<Fd: AsFd>(&self, fd: &Fd, events: EventType, data: u64) -> Result<()> {
        self.epoll_ctl(sys::EPOLL_CTL_MOD, fd, &Event { events, data }, Kind::Mod)
    }

    /// Remove (deregister) the target fd from the interest list.
    ///
    /// Reference: `epoll_ctl(2)`.
    #[inline]
    pub fn delete<Fd: AsFd>(&self, fd: &Fd) -> Result<()> {
        self.epoll_ctl(sys::EPOLL_CTL_DEL, fd, 0 as _, Kind::Del)
    }

    fn epoll_ctl<Fd: AsFd>(&self, op: i32, fd: &Fd, ev: *const Event, er: Kind) -> Result<()> {
        sys::call!(RD, epoll_ctl, self.as_fd(), op, fd.as_fd(), ev).e(er)
    }
}

impl Epoll {
    /// Waits for an I/O event and returns the number of events written to the given buffer.
    ///
    /// `buf` must not be empty.
    ///
    /// The `timeout` specifies the number of milliseconds that this call will block.
    ///
    /// This call will block until either a file descriptor deliver an event, the call is interupted
    /// by a signal handler, or `timeout` expires.
    ///
    /// Specifying a `timeout` of -1 causes this call to block indefinitely, while specifying a
    /// `timeout` equal to zero causes this call to return immediately, even if no events are
    /// available.
    ///
    /// The returned [`Event::data`] contains the same data as was specified in the most recent
    /// supplied data for the corresponding open fd.
    ///
    /// The returned [`Event::events`] is a bit mask of that indicates the events that have occurred
    /// for the corresponding open file description.
    ///
    /// Reference: `epoll_wait(2)`.
    #[inline]
    pub fn wait(&self, buf: &mut [MaybeUninit<Event>], timeout: i32) -> Result<usize> {
        sys::call!(epoll_wait, self.as_fd(), buf.as_mut_ptr(), buf.len(), timeout).io(Kind::Wait)
    }
}

// ===== EpollEvent =====

/// [`Epoll`] event.
///
/// Reference: `epoll_event(3type)`.
#[derive(Debug, Default, Clone)]
#[repr(C, packed)]
pub struct Event {
    /// Event types returned by [`Epoll::wait`], and input flags, which affect its behaviour, but
    /// not returned.
    pub events: EventType,
    /// Data that the kernel should save and then return when associated fd becomes ready.
    pub data: u64,
}

// ===== Flags =====

/// Epoll creation flags.
///
/// Refernce: `epoll_create1(2)`.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Flags(i32);

flags::impl_bitops_simple!(Flags);

impl Flags {
    /// Set the close-on-exec (FD_CLOEXEC) flag on the new fd.
    pub const CLOEXEC: Self = Self(sys::EPOLL_CLOEXEC);
}

impl flags::OpenFlag for Flags {
    const CLOEXEC: Self = Self::CLOEXEC;
    /// Epoll does not have non-blocking mode.
    const NONBLOCK: Self = Self(0);
}

// ===== EventType =====

/// Epoll event types.
///
/// Reference: `epoll_ctl(2)`.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct EventType(u32);

flags::impl_bitops_simple!(EventType);
flags::impl_bitops_simple!(EventType, InputFlags);

impl EventType {
    /// The associated file is available for `read` operations.
    pub const IN: Self = Self(sys::EPOLLIN);
    /// There is an exceptional condition on the file descriptor.
    pub const PRI: Self = Self(sys::EPOLLPRI);
    /// The associated file is available for `write` operations.
    pub const OUT: Self = Self(sys::EPOLLOUT);
    /// Error condition happened on the associated file descriptor.
    ///
    /// [`Epoll::wait`] will always report for this event; it is not necessary to set it in
    /// [`Event::events`].
    pub const ERR: Self = Self(sys::EPOLLERR);
    /// Hang up happened on the associated file descriptor.
    ///
    /// [`Epoll::wait`] will always report for this event; it is not necessary to set it in
    /// [`Event::events`].
    pub const HUP: Self = Self(sys::EPOLLHUP);
    /// Stream socket peer closed connection, or shut down writing half of connection.
    ///
    /// This flag is especially useful for writing simple code to detect peer shutdown when using
    /// edge-triggered monitoring.
    pub const RDHUP: Self = Self(sys::EPOLLRDHUP);

    /// Returns `true` if events contains [`EventType::IN`].
    #[inline]
    pub fn has_read(self) -> bool {
        self & Self::IN == Self::IN
    }

    /// Returns `true` if events contains [`EventType::OUT`].
    #[inline]
    pub fn has_write(self) -> bool {
        self & Self::OUT == Self::OUT
    }

    /// Returns `true` if events contains [`EventType::RDHUP`].
    #[inline]
    pub fn has_read_hang_up(self) -> bool {
        self & Self::RDHUP == Self::RDHUP
    }
}

// ===== InputFlags =====

/// Epoll input flags.
///
/// Reference: `epoll_ctl(2)`.
#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct InputFlags(u32);

impl InputFlags {
    /// Requests edge-triggered notification for the associated file descriptor.
    pub const ET: Self = Self(sys::EPOLLET);
    /// Requests one-shot notification for the associated file descriptor.
    pub const ONESHOT: Self = Self(sys::EPOLLONESHOT);
    /// Ensure that the system does not enter "suspend" or "hibernate" while this event is pending
    /// or being processed.
    pub const WAKEUP: Self = Self(sys::EPOLLWAKEUP);
    /// Sets an exclusive wakeup mode for the epoll fd that is being attached to the target fd.
    pub const EXCLUSIVE: Self = Self(sys::EPOLLEXCLUSIVE);
}

// ===== errors =====

/// Type alias for result of [`Epoll`] operations.
pub type Result<T> = result::Result<T, Error>;

/// An error that may occur during any [`Epoll`] operations.
#[derive(Debug, Clone)]
pub struct Error {
    kind: Kind,
    code: ErrCode,
}

#[derive(Debug, Clone)]
enum Kind {
    Create,
    Add,
    Mod,
    Del,
    Wait,
}

error::impl_error_with_kind!(Error, Kind);

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self { kind, code } = self;
        let msg = match kind {
            Kind::Create => "create epoll",
            Kind::Add => "add fd to epoll",
            Kind::Mod => "modify fd on the epoll",
            Kind::Del => "delete fd on the epoll",
            Kind::Wait => "wait for notification on the epoll",
        };
        write!(f, "failed to {msg}: {code}")
    }
}

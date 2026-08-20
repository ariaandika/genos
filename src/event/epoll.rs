//! [`Epoll`] associated types.
use core::mem::MaybeUninit;
use core::{fmt, result};

use crate::error::{ErrCode, SysResExt};
use crate::fd::{AsFd, OwnedFd};
use crate::{error, fd, flags, sys};

// ===== Epoll =====

/// I/O event notification facility.
#[derive(Debug)]
pub struct Epoll(OwnedFd);

fd::impl_fd_simple!(Epoll);

impl Epoll {
    /// Creates new [`Epoll`].
    #[inline]
    pub fn create(flags: Flags) -> Result<Self> {
        sys::call!(RD, __NR_epoll_create1, flags.0).fd(Kind::Create)
    }

    /// Add an entry to the interest list.
    ///
    /// [`InputFlags`] can be added by `OR`-ing with [`EventType`].
    #[inline]
    pub fn add<Fd: AsFd>(&self, fd: &Fd, events: EventType, data: u64) -> Result<()> {
        const OP: i32 = sys::EPOLL_CTL_ADD;
        let event = Event { events, data };
        sys::call!(RD, __NR_epoll_ctl, self.as_fd(), OP, fd.as_fd(), &event).e(Kind::Add)
    }

    /// Change the settings associated with fd in the interest list.
    ///
    /// [`InputFlags`] can be added by `OR`-ing with [`EventType`].
    #[inline]
    pub fn modify<Fd: AsFd>(&self, fd: &Fd, events: EventType, data: u64) -> Result<()> {
        const OP: i32 = sys::EPOLL_CTL_MOD;
        let event = Event { events, data };
        sys::call!(RD, __NR_epoll_ctl, self.as_fd(), OP, fd.as_fd(), &event).e(Kind::Mod)
    }

    /// Remove (deregister) the target fd from the interest list.
    #[inline]
    pub fn delete<Fd: AsFd>(&self, fd: &Fd) -> Result<()> {
        const OP: i32 = sys::EPOLL_CTL_DEL;
        sys::call!(RD, __NR_epoll_ctl, self.as_fd(), OP, fd.as_fd(), 0).e(Kind::Del)
    }
}

impl Epoll {
    /// Waits for an I/O event and returns the number of events written to the given buffer.
    ///
    /// The buffer must not be empty.
    ///
    /// The `timeout` specifies the number of milliseconds that this call will block. Time is
    /// measured against the `CLOCK_MONOTONIC` clock. Note that the timeout interval will be rounded
    /// up to the system clock granularity, and kernel scheduling delays mean that the blocking
    /// interval may overrun by a small amount.
    ///
    /// Specifying a `timeout` of -1 causes this call to block indefinitely, while specifying a
    /// `timeout` equal to zero causes this call to return immediately, even if no events are
    /// available.
    ///
    /// This call will block until either a file descriptor deliver an event, the call is interupted
    /// by a signal handler, or `timeout` expires.
    ///
    /// The `data` field of each returned [`Event`] structure contains the same data as was
    /// specified in the most recent supplied data for the corresponding open fd.
    ///
    /// The `events` field is a bit mask of [`EventType`] that indicates the events that have
    /// occurred for the corresponding open file description.
    #[inline]
    pub fn wait(&self, buf: &mut [MaybeUninit<Event>], timeout: i32) -> Result<usize> {
        sys::call!(__NR_epoll_wait, self.as_fd(), &mut *buf, buf.len(), timeout).io(Kind::Wait)
    }
}

// ===== EpollEvent =====

/// [`Epoll`] event.
// with repr equivalen to `epoll_event`
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

/// Socket creation flags.
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
#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct InputFlags(u32);

impl InputFlags {
    /// Sets an exclusive wakeup mode for the epoll file descriptor that is being attached to the
    /// target fd.
    ///
    /// When a wakeup event occurs and multiple epoll file descriptors are attached to the same
    /// target file using [`InputFlags::EXCLUSIVE`], one or more of the epoll file descriptors will
    /// receive an event with [`Epoll::wait`]. The default in this scenario (when
    /// [`InputFlags::EXCLUSIVE`] is not set) is for all epoll file descriptors to receive an event.
    /// [`InputFlags::EXCLUSIVE`] is thus useful for avoiding thundering herd problems in certain
    /// scenarios.
    ///
    /// If the same file descriptor is in multiple epoll instances, some with the
    /// [`InputFlags::EXCLUSIVE`] flag, and others without, then events will be provided to all
    /// epoll instances that did not specify [`InputFlags::EXCLUSIVE`], and at least one of the
    /// epoll instances that did specify [`InputFlags::EXCLUSIVE`].
    ///
    /// The following values may be specified in conjunction with [`InputFlags::EXCLUSIVE`]:
    /// [`EventType::IN`], [`EventType::OUT`], [`InputFlags::WAKEUP`], and [`InputFlags::ET`].
    /// [`EventType::HUP`] and [`EventType::ERR`] can also be specified, but this is not required:
    /// as usual, these events are always reported if they occur, regardless of whether they are
    /// specified in events. Attempts to specify other values in events yield the error `EINVAL`.
    ///
    /// [`InputFlags::EXCLUSIVE`] may be used only in an [`Epoll::add`] operation; attempts to
    /// employ it with [`Epoll::modify`] yield an error. If [`InputFlags::EXCLUSIVE`] has been set,
    /// then a subsequent [`Epoll::modify`] on the same epfd, fd pair yields an error. Specifying
    /// [`InputFlags::EXCLUSIVE`] in events and specifies the target fd as an epoll instance will
    /// likewise fail. The error in all of these cases is `EINVAL`.
    pub const EXCLUSIVE: Self = Self(sys::EPOLLEXCLUSIVE);

    /// If [`InputFlags::ONESHOT`] and [`InputFlags::ET`] are clear and the process has the
    /// CAP_BLOCK_SUSPEND capability, ensure that the system does not enter "suspend" or "hibernate"
    /// while this event is pending or being processed.
    ///
    /// The event is considered as being "processed" from the time when it is returned by a call to
    /// [`Epoll::wait`] until the next call to [`Epoll::wait`] on the same [`Epoll`] file
    /// descriptor, the closure of that file descriptor, the removal of the event file descriptor
    /// with [`Epoll::delete`], or the clearing of [`InputFlags::WAKEUP`] for the event file
    /// descriptor with [`Epoll::modify`]. See also BUGS.
    pub const WAKEUP: Self = Self(sys::EPOLLWAKEUP);

    /// Requests one-shot notification for the associated file descriptor.
    ///
    /// This means that after an event notified for the file descriptor by [`Epoll::wait`], the file
    /// descriptor is disabled in the interest list and no other events will be reported by the
    /// epoll interface. The user must call [`Epoll::modify`] to rearm the file descriptor with a
    /// new event mask.
    pub const ONESHOT: Self = Self(sys::EPOLLONESHOT);

    /// Requests edge-triggered notification for the associated file descriptor.
    ///
    /// The default behavior for epoll is level-triggered.
    pub const ET: Self = Self(sys::EPOLLET);
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
            Kind::Add => "add fd to the epoll",
            Kind::Mod => "modify fd on the epoll",
            Kind::Del => "delete fd on the epoll",
            Kind::Wait => "wait for notification on the epoll",
        };
        write!(f, "failed to {msg}: {code}")
    }
}

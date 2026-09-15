//! [`Epoll`] associated types.
use core::mem::MaybeUninit;
use core::{fmt, result};

use crate::error::{ErrCode, SysResExt};
use crate::fd::{AsFd, Open, OwnedFd};
use crate::{error, fd, flags, sys};

// ===== Epoll =====

/// I/O event notification facility (`epoll(7)`).
#[derive(Debug)]
pub struct Epoll(OwnedFd);

fd::impl_fd_simple!(Epoll);

impl Epoll {
    /// Creates new [`Epoll`] (`epoll_create1(2)`).
    #[inline]
    pub fn create(flags: Flags) -> Result<Self> {
        sys::call_rd!(sys_epoll_create1, flags.0).fd(Kind::Create)
    }

    /// Add an entry to the interest list (`epoll_ctl(2)`).
    ///
    /// [`InputFlags`] can be added by `OR`-ing with [`EventType`].
    #[inline]
    pub fn add<Fd: AsFd + ?Sized>(&self, fd: &Fd, events: EventType, data: u64) -> Result<()> {
        self.epoll_ctl(EPOLL_CTL_ADD, fd, &Event { events, data }, Kind::Add)
    }

    /// Change the settings associated with fd in the interest list (`epoll_ctl(2)`).
    ///
    /// [`InputFlags`] can be added by `OR`-ing with [`EventType`].
    #[inline]
    pub fn modify<Fd: AsFd + ?Sized>(&self, fd: &Fd, events: EventType, data: u64) -> Result<()> {
        self.epoll_ctl(EPOLL_CTL_MOD, fd, &Event { events, data }, Kind::Mod)
    }

    /// Remove (deregister) the target fd from the interest list (`epoll_ctl(2)`).
    #[inline]
    pub fn delete<Fd: AsFd + ?Sized>(&self, fd: &Fd) -> Result<()> {
        self.epoll_ctl(EPOLL_CTL_DEL, fd, 0 as _, Kind::Del)
    }

    #[inline]
    fn epoll_ctl<Fd>(&self, op: i32, fd: &Fd, ev: *const Event, er: Kind) -> Result<()>
    where
        Fd: AsFd + ?Sized,
    {
        sys::call_rd!(sys_epoll_ctl, self.as_fd(), op, fd.as_fd(), ev).e(er)
    }

    /// Waits for events `epoll_wait(2)`.
    #[inline]
    pub fn wait(&self, buf: &mut [MaybeUninit<Event>], timeout: i32) -> Result<usize> {
        sys::call!(sys_epoll_wait, self.as_fd(), buf.as_mut_ptr(), buf.len(), timeout)
            .io(Kind::Wait)
    }
}

// ===== Event =====

/// [`Epoll`] event (`epoll_event(3type)`).
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

/// [`Epoll::create`] flags.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Flags(i32);

flags::impl_bitops_simple!(Flags);

impl Epoll {
    /// `EPOLL_CLOEXEC`
    pub const CLOEXEC: Flags = Flags(EPOLL_CLOEXEC);
}

// ===== EventType =====

/// [`Event`] types (`epoll_ctl(2)`).
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct EventType(u32);

flags::impl_bitops_simple!(EventType);
flags::impl_bitops_simple!(EventType, InputFlags);

impl Epoll {
    /// `EPOLLIN`
    pub const IN: EventType = EventType(EPOLLIN);
    /// `EPOLLPRI`
    pub const PRI: EventType = EventType(EPOLLPRI);
    /// `EPOLLOUT`
    pub const OUT: EventType = EventType(EPOLLOUT);
    /// `EPOLLERR`
    pub const ERR: EventType = EventType(EPOLLERR);
    /// `EPOLLHUP`
    pub const HUP: EventType = EventType(EPOLLHUP);
    /// `EPOLLRDHUP`
    pub const RDHUP: EventType = EventType(EPOLLRDHUP);
}

impl EventType {
    /// Returns `true` if events contains `EPOLLIN`.
    #[inline]
    pub const fn has_read(self) -> bool {
        self.0 & Epoll::IN.0 != 0
    }

    /// Returns `true` if events contains `EPOLLOUT`.
    #[inline]
    pub const fn has_write(self) -> bool {
        self.0 & Epoll::OUT.0 != 0
    }

    /// Returns `true` if events contains `EPOLLRDHUP`.
    #[inline]
    pub const fn has_rdhup(self) -> bool {
        self.0 & Epoll::RDHUP.0 != 0
    }
}

// ===== InputFlags =====

/// [`Event`] input flags (`epoll_ctl(2)`).
#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct InputFlags(u32);

impl Epoll {
    /// `EPOLLET`
    pub const ET: InputFlags = InputFlags(EPOLLET);
    /// `EPOLLONESHOT`
    pub const ONESHOT: InputFlags = InputFlags(EPOLLONESHOT);
    /// `EPOLLWAKEUP`
    pub const WAKEUP: InputFlags = InputFlags(EPOLLWAKEUP);
    /// `EPOLLEXCLUSIVE`
    pub const EXCLUSIVE: InputFlags = InputFlags(EPOLLEXCLUSIVE);
}

// ===== Error =====

/// The result of [`Epoll`] operations.
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
            Kind::Wait => "wait for epoll event",
        };
        write!(f, "failed to {msg}: {code}")
    }
}

// ===== extern =====

// include/uapi/linux/eventpoll.h

const EPOLL_CLOEXEC: i32 = Open::CLOEXEC.raw();

const EPOLL_CTL_ADD: i32 = 1;
const EPOLL_CTL_DEL: i32 = 2;
const EPOLL_CTL_MOD: i32 = 3;

const EPOLLIN: u32 = 0x00000001;
const EPOLLPRI: u32 = 0x00000002;
const EPOLLOUT: u32 = 0x00000004;
const EPOLLERR: u32 = 0x00000008;
const EPOLLHUP: u32 = 0x00000010;
const EPOLLRDHUP: u32 = 0x00002000;
const EPOLLEXCLUSIVE: u32 = 1 << 28;
const EPOLLWAKEUP: u32 = 1 << 29;
const EPOLLONESHOT: u32 = 1 << 30;
const EPOLLET: u32 = 1 << 31;

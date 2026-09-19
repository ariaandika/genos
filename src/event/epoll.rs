//! [`Epoll`] associated types.
use core::mem::MaybeUninit;

use crate::fd::{AsFd, OwnedFd};
use crate::sys::SysRes;
use crate::{fd, flags, sys};

// ===== Epoll =====

/// I/O event notification facility (`epoll(7)`).
#[derive(Debug)]
pub struct Epoll(OwnedFd);

fd::impl_fd_simple!(Epoll);

impl Epoll {
    /// Creates new [`Epoll`] (`epoll_create1(2)`).
    #[inline]
    pub fn create(flags: Flags) -> impl SysRes<Self> {
        sys::call_rd!(sys_epoll_create1, flags.0)
    }

    #[inline]
    fn epoll_ctl<Fd: AsFd + ?Sized>(&self, op: i32, fd: &Fd, ev: *const Event) -> impl SysRes<()> {
        sys::call_rd!(sys_epoll_ctl, self.as_raw_fd(), op, fd.as_raw_fd(), ev)
    }

    /// Waits for events `epoll_wait(2)`.
    #[inline]
    pub fn wait(&self, buf: &mut [MaybeUninit<Event>], timeout: i32) -> impl SysRes<usize> {
        sys::call!(sys_epoll_wait, self.as_raw_fd(), buf.as_mut_ptr(), buf.len(), timeout)
    }
}

impl Epoll {
    /// Add an entry to the interest list (`epoll_ctl(2)`).
    ///
    /// [`InputFlags`] can be added by `OR`-ing with [`EventType`].
    #[inline]
    pub fn add<Fd: AsFd + ?Sized>(&self, fd: &Fd, events: EventType, data: u64) -> impl SysRes<()> {
        self.epoll_ctl(EPOLL_CTL_ADD, fd, &Event { events, data })
    }

    /// Change the settings associated with fd in the interest list (`epoll_ctl(2)`).
    ///
    /// [`InputFlags`] can be added by `OR`-ing with [`EventType`].
    #[inline]
    pub fn modify<Fd>(&self, fd: &Fd, events: EventType, data: u64) -> impl SysRes<()>
    where
        Fd: AsFd + ?Sized,
    {
        self.epoll_ctl(EPOLL_CTL_MOD, fd, &Event { events, data })
    }

    /// Remove (deregister) the target fd from the interest list (`epoll_ctl(2)`).
    #[inline]
    pub fn delete<Fd: AsFd + ?Sized>(&self, fd: &Fd) -> impl SysRes<()> {
        self.epoll_ctl(EPOLL_CTL_DEL, fd, 0 as _)
    }
}

// ===== Event =====

/// [`Epoll`] event (`epoll_event(3type)`).
#[derive(Debug, Default, Clone)]
#[repr(C, packed)]
pub struct Event {
    /// `epoll_event.events`
    pub events: EventType,
    /// `epoll_event.data`
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

// ===== InputFlags =====

/// [`Event`] input flags (`epoll_ctl(2)`).
#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct InputFlags(u32);

flags::impl_bitops_simple!(InputFlags);
flags::impl_bitops_simple!(InputFlags, EventType, Output = EventType);

impl Epoll {
    /// `EPOLLEXCLUSIVE`
    pub const EXCLUSIVE: InputFlags = InputFlags(EPOLLEXCLUSIVE);
    /// `EPOLLWAKEUP`
    pub const WAKEUP: InputFlags = InputFlags(EPOLLWAKEUP);
    /// `EPOLLONESHOT`
    pub const ONESHOT: InputFlags = InputFlags(EPOLLONESHOT);
    /// `EPOLLET`
    pub const ET: InputFlags = InputFlags(EPOLLET);
}

// ===== extern =====

// include/uapi/linux/eventpoll.h

const EPOLL_CLOEXEC: i32 = fd::O_CLOEXEC;

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

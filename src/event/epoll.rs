//! [`Epoll`] associated types.
use core::mem::MaybeUninit;

use crate::fd::{AsFd, OwnedFd};
use crate::sys::{Error, arch};
use crate::{fd, flags, sys};

// ===== Epoll =====

/// I/O event notification facility (`epoll(7)`).
#[derive(Debug)]
pub struct Epoll(OwnedFd);

fd::impl_fd_simple!(Epoll);

impl Epoll {
    /// Creates new [`Epoll`] (`epoll_create1(2)`).
    #[inline]
    pub fn create(flags: Flags) -> Result<Self, Error<arch::sys_epoll_create1>> {
        sys::call_rd!(sys_epoll_create1, flags.0)
    }

    /// Add, modify, or remove entries in the interest list (`epoll_ctl(2)`).
    #[inline]
    pub fn ctl<Fd: AsFd + ?Sized>(
        &self,
        op: Ctl,
        fd: &Fd,
        ev: Option<&Event>,
    ) -> Result<(), Error<arch::sys_epoll_ctl>> {
        sys::call_rd!(sys_epoll_ctl, self.as_raw_fd(), op.0, fd.as_raw_fd(), optref(ev))
    }

    /// Waits for events `epoll_wait(2)`.
    #[inline]
    pub fn wait(
        &self,
        buf: &mut [MaybeUninit<Event>],
        timeout: i32,
    ) -> Result<usize, Error<arch::sys_epoll_wait>> {
        sys::call!(sys_epoll_wait, self.as_raw_fd(), buf.as_mut_ptr(), buf.len(), timeout)
    }
}

impl Epoll {
    /// Add an entry to the interest list (`epoll_ctl(2)`).
    ///
    /// [`InputFlags`] can be added by `OR`-ing with [`EventType`].
    #[inline]
    pub fn add<Fd: AsFd + ?Sized>(
        &self,
        fd: &Fd,
        events: EventType,
        data: u64,
    ) -> Result<(), Error<arch::sys_epoll_ctl>> {
        self.ctl(Self::CTL_ADD, fd, Some(&Event { events, data }))
    }

    /// Change the settings associated with fd in the interest list (`epoll_ctl(2)`).
    ///
    /// [`InputFlags`] can be added by `OR`-ing with [`EventType`].
    #[inline]
    pub fn modify<Fd>(
        &self,
        fd: &Fd,
        events: EventType,
        data: u64,
    ) -> Result<(), Error<arch::sys_epoll_ctl>>
    where
        Fd: AsFd + ?Sized,
    {
        self.ctl(Self::CTL_MOD, fd, Some(&Event { events, data }))
    }

    /// Remove (deregister) the target fd from the interest list (`epoll_ctl(2)`).
    #[inline]
    pub fn delete<Fd: AsFd + ?Sized>(&self, fd: &Fd) -> Result<(), Error<arch::sys_epoll_ctl>> {
        self.ctl(Self::CTL_DEL, fd, None)
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

// ===== Ctl =====

/// [`Epoll::ctl`] operation.
#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct Ctl(i32);

impl Epoll {
    /// `EPOLL_CTL_ADD`
    pub const CTL_ADD: Ctl = Ctl(EPOLL_CTL_ADD);
    /// `EPOLL_CTL_DEL`
    pub const CTL_DEL: Ctl = Ctl(EPOLL_CTL_DEL);
    /// `EPOLL_CTL_MOD`
    pub const CTL_MOD: Ctl = Ctl(EPOLL_CTL_MOD);
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

fn optref(opt: Option<&Event>) -> *const Event {
    // this will generate to just a `mov`
    opt.map_or(core::ptr::null(), |e|e as *const _)
}

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

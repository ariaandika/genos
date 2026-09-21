//! [`Timerfd`] associated types.
use crate::fd::{AsFd, OwnedFd};
use crate::sys::{Error, arch, optmut};
use crate::time::{Clock, ITimerspec};
use crate::{fd, flags, sys};

// ===== Timerfd =====

/// Timer notifications.
#[derive(Debug)]
pub struct Timerfd(OwnedFd);

fd::impl_fd_simple!(Timerfd);

impl Timerfd {
    /// Creates new [`Timerfd`] (`timerfd_create(2)`).
    #[inline]
    pub fn create(clock: Clock, flags: Flags) -> Result<Self, Error<arch::sys_timerfd_create>> {
        sys::call_rd!(sys_timerfd_create, i32::from(clock), flags.0)
    }

    /// Arms (starts) or disarms (stops) the timer (`timerfd_settime(2)`).
    #[inline]
    pub fn set_time(
        &self,
        new: &ITimerspec,
        old: Option<&mut ITimerspec>,
        flags: TimerFlags,
    ) -> Result<(), Error<arch::sys_timerfd_settime>> {
        sys::call!(sys_timerfd_settime, self.as_raw_fd(), flags.0, new, optmut(old))
    }

    /// Returns the current timer (`timerfd_gettime(2)`).
    #[inline]
    pub fn get_time(
        &self,
        curr_value: &mut ITimerspec,
    ) -> Result<(), Error<arch::sys_timerfd_gettime>> {
        sys::call!(sys_timerfd_gettime, self.as_raw_fd(), curr_value)
    }
}

// ===== Flags =====

/// [`Timerfd`] creation flags.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Flags(i32);

flags::impl_bitops_simple!(Flags);

impl Timerfd {
    /// `TFD_CLOEXEC`
    pub const CLOEXEC: Flags = Flags(TFD_CLOEXEC);
    /// `TFD_NONBLOCK`
    pub const NONBLOCK: Flags = Flags(TFD_NONBLOCK);
}

// ===== TimerFlags =====

/// [`Timerfd::set_time`] flags.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct TimerFlags(i32);

flags::impl_bitops_simple!(TimerFlags);

impl Timerfd {
    /// `TFD_TIMER_ABSTIME`
    pub const ABSTIME: TimerFlags = TimerFlags(TFD_TIMER_ABSTIME);
    /// `TFD_TIMER_CANCEL_ON_SET`
    pub const CANCEL_ON_SET: TimerFlags = TimerFlags(TFD_TIMER_CANCEL_ON_SET);
}

// ===== extern =====

// include/uapi/linux/timerfd.h

const TFD_TIMER_ABSTIME: i32 = 1 << 0;
const TFD_TIMER_CANCEL_ON_SET: i32 = 1 << 1;
const TFD_CLOEXEC: i32 = fd::O_CLOEXEC;
const TFD_NONBLOCK: i32 = fd::O_NONBLOCK;

//! [`Timerfd`] associated types.
use core::{fmt, mem, result};

use crate::error::{ErrCode, SysResExt};
use crate::fd::{AsFd, Open, OwnedFd};
use crate::time::{Clock, ITimerspec};
use crate::{error, fd, flags, sys};

// ===== Timerfd =====

/// Timer notifications.
#[derive(Debug)]
pub struct Timerfd(OwnedFd);

fd::impl_fd_simple!(Timerfd);

impl Timerfd {
    /// Creates new [`Timerfd`] (`timerfd_create(2)`).
    #[inline]
    pub fn create(clock: Clock, flags: Flags) -> Result<Self> {
        sys::call_rd!(sys_timerfd_create, i32::from(clock), flags.0).fd(Kind::Create)
    }

    /// Arms (starts) or disarms (stops) the timer (`timerfd_settime(2)`).
    #[inline]
    pub fn set_time(&self, time: &ITimerspec, flags: TimerFlags) -> Result<()> {
        sys::call_rd!(sys_timerfd_settime, self.as_raw_fd(), flags.0, time, 0).e(Kind::Set)
    }

    /// Returns the current timer (`timerfd_gettime(2)`).
    ///
    /// The underlying syscalls can only returns error if given fd or pointer is invalid, which this
    /// struct guarantee to be valid.
    ///
    /// The underlying syscall may return error if caller creates `Timerfd` with invalid fd via
    /// [`FromRawFd::from_raw_fd`][1]. In that case, the returned `ITimerspec` is zeroed.
    ///
    /// If the error is a concern, caller may use [`Timerfd::try_time`].
    ///
    /// [1]: crate::fd::FromRawFd::from_raw_fd
    #[inline]
    pub fn time(&self) -> ITimerspec {
        let mut time = unsafe { mem::zeroed::<ITimerspec>() };
        timerfd_gettime(self, &raw mut time);
        time
    }

    /// Returns the current timer (`timerfd_gettime(2)`).
    #[inline]
    pub fn try_time(&self) -> Result<ITimerspec> {
        let mut time = unsafe { mem::zeroed::<ITimerspec>() };
        timerfd_gettime(self, &raw mut time)
            .e(Kind::Get)
            .map(|()| time)
    }
}

fn timerfd_gettime(fd: &Timerfd, time: *mut ITimerspec) -> impl SysResExt {
    sys::call!(sys_timerfd_gettime, fd.as_raw_fd(), time)
}

// ===== Flags =====

/// [`Timerfd`] creation flags.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Flags(i32);

impl Flags {
    /// `TFD_CLOEXEC`
    pub const CLOEXEC: Self = Self(TFD_CLOEXEC);
    /// `TFD_NONBLOCK`
    pub const NONBLOCK: Self = Self(TFD_NONBLOCK);
}

flags::impl_bitops_simple!(Flags);

// ===== TimerFlags =====

/// [`Timerfd::set_time`] flags.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct TimerFlags(i32);

impl TimerFlags {
    /// `TFD_TIMER_ABSTIME`
    pub const ABSTIME: Self = Self(TFD_TIMER_ABSTIME);
    /// `TFD_TIMER_CANCEL_ON_SET`
    pub const CANCEL_ON_SET: Self = Self(TFD_TIMER_CANCEL_ON_SET);
}

flags::impl_bitops_simple!(TimerFlags);

// ===== Error =====

/// Type alias for result of [`Timerfd`] operations.
pub type Result<T, E = Error> = result::Result<T, E>;

/// An error that may occur during any [`Timerfd`] operations.
#[derive(Debug, Clone)]
pub struct Error {
    kind: Kind,
    code: ErrCode,
}

#[derive(Debug, Clone, Copy)]
enum Kind {
    Create,
    Get,
    Set,
}

error::impl_error_with_kind!(Error, Kind);

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self { kind, code } = self;
        let msg = match kind {
            Kind::Create => "create timerfd",
            Kind::Get => "get timerfd time",
            Kind::Set => "set timerfd time",
        };
        write!(f, "failed to {msg}: {code}")
    }
}

// ===== extern =====

// include/uapi/linux/timerfd.h

const TFD_TIMER_ABSTIME: i32 = 1 << 0;
const TFD_TIMER_CANCEL_ON_SET: i32 = 1 << 1;
const TFD_CLOEXEC: i32 = Open::CLOEXEC.raw();
const TFD_NONBLOCK: i32 = Open::NONBLOCK.raw();

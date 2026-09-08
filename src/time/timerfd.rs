//! [`Timerfd`] associated types.
use core::mem::MaybeUninit;
use core::time::Duration;
use core::{fmt, mem, result};

use crate::error::{ErrCode, SysResExt};
use crate::fd::{AsFd, OwnedFd};
use crate::time::Clock;
use crate::{error, fd, flags, io, sys};

// ===== Timerfd =====

/// Timer notifications.
#[derive(Debug)]
pub struct Timerfd(OwnedFd);

fd::impl_fd_simple!(Timerfd);

impl Timerfd {
    /// Creates new [`Timerfd`].
    ///
    /// The `clock` argument must be one of the following:
    ///
    /// - [`Clock::REALTIME`]
    /// - [`Clock::MONOTONIC`]
    /// - [`Clock::BOOTTIME`]
    /// - [`Clock::REALTIME_ALARM`]
    /// - [`Clock::BOOTTIME_ALARM`]
    #[inline]
    pub fn create(clock: Clock, flags: Flags) -> Result<Self> {
        sys::call!(RD, sys_timerfd_create, i32::from(clock), flags.0).fd(Kind::Create)
    }

    /// Arms (starts) or disarms (stops) the timer.
    ///
    /// `initial` is an initial expiration
    ///
    /// Setting the duration to zero, will disarm the timer.
    ///
    /// By default, the initial expiration time specified is interpreted relative to the current
    /// time on the timer's clock at the time of the call. An absolute timeout can be selected via
    /// the flags argument.
    #[inline]
    pub fn set_time(&self, initial: Duration, interval: Duration, flags: TimerFlags) -> Result<()> {
        let time = sys::itimerspec {
            // interval timer, after the initial timer
            it_interval: sys::timespec {
                tv_sec: interval.as_secs() as _,
                tv_nsec: interval.subsec_nanos() as _,
            },
            // initial timer
            it_value: sys::timespec {
                tv_sec: initial.as_secs() as _,
                tv_nsec: initial.subsec_nanos() as _,
            },
        };
        sys::call!(RD, sys_timerfd_settime, self.as_fd(), flags.0, &time, 0).e(Kind::Set)
    }

    /// Returns the current `(initial, interval)` timer.
    #[inline]
    pub fn time(&self) -> (Duration, Duration) {
        let mut time = unsafe { mem::zeroed::<sys::itimerspec>() };
        sys::call!(sys_timerfd_gettime, self.as_fd(), &mut time);
        let init = Duration::new(time.it_value.tv_sec as _, time.it_value.tv_nsec as _);
        let ival = Duration::new(time.it_interval.tv_sec as _, time.it_interval.tv_nsec as _);
        (init, ival)
    }

    /// Checks the expiration status.
    #[inline]
    pub fn read(&self) -> Result<u64> {
        let mut n = [const { MaybeUninit::uninit() }; size_of::<u64>()];
        io::read(self, &mut n)?;
        Ok(unsafe { mem::transmute::<[MaybeUninit<u8>; _], u64>(n) })
    }
}

// ===== Flags =====

/// [`Timerfd`] creation flags.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Flags(i32);

impl Flags {
    /// Set the close-on-exec (FD_CLOEXEC) flag on the new fd.
    pub const CLOEXEC: Self = Self(sys::TFD_CLOEXEC);
    /// Set the `O_NONBLOCK` file status flag on the new fd.
    pub const NONBLOCK: Self = Self(sys::TFD_NONBLOCK);
}

impl flags::OpenFlag for Flags {
    const CLOEXEC: Self = Self::CLOEXEC;
    const NONBLOCK: Self = Self::NONBLOCK;
}

flags::impl_bitops_simple!(Flags);

// ===== TimerFlags =====

/// [`Timerfd::set_time`] flags.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct TimerFlags(i32);

impl TimerFlags {
    /// Interpret `new_value.it_value` as an absolute value on the timer's clock.
    pub const ABSTIME: Self = Self(sys::TFD_TIMER_ABSTIME);
    /// Mark this timer as cancelable if the real-time clock undergoes a discontinuous change.
    ///
    /// When such changes occur, a current or future `read(2)` will fail with the error `ECANCELED`.
    pub const CANCEL_ON_SET: Self = Self(sys::TFD_TIMER_CANCEL_ON_SET);
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
    Set,
    Read,
}

error::impl_error_with_kind!(Error, Kind);

impl From<io::ReadError> for Error {
    #[inline]
    fn from(value: io::ReadError) -> Self {
        Error { kind: Kind::Read, code: value.into() }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self { kind, code } = self;
        let msg = match kind {
            Kind::Create => "create timerfd",
            Kind::Set => "configure timerfd",
            Kind::Read => "check timerfd status",
        };
        write!(f, "failed to {msg}: {code}")
    }
}

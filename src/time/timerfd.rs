//! Timer notifications.
use core::time::Duration;
use core::{error, fmt, mem, result};

use crate::error::{AsErrCode, ErrCode};
use crate::fd::{AsRawFd, FromRawFd, OwnedFd, impl_fd_simple};
use crate::net::OpenFlag;

/// Timer notifications.
#[derive(Debug)]
pub struct Timerfd(OwnedFd);

impl_fd_simple!(Timerfd);

impl Timerfd {
    /// Creates new [`Timerfd`].
    #[inline]
    pub fn create(kind: ClockKind) -> Result<Self> {
        unsafe { fd(libc::timerfd_create(kind.0, 0), Kind::Create) }
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
        let time = libc::itimerspec {
            // interval timer, after the initial timer
            it_interval: libc::timespec {
                tv_sec: interval.as_secs() as _,
                tv_nsec: interval.subsec_nanos() as _,
            },
            // initial timer
            it_value: libc::timespec {
                tv_sec: initial.as_secs() as _,
                tv_nsec: initial.subsec_nanos() as _,
            },
        };
        unsafe { e(libc::timerfd_settime(self.as_raw_fd(), flags.0, &time, 0 as _), Kind::Set) }
    }

    /// Returns the current `(initial, interval)` timer.
    #[inline]
    pub fn get_time(&self) -> (Duration, Duration) {
        let mut time = unsafe { mem::zeroed::<libc::itimerspec>() };
        let _ = unsafe { libc::timerfd_gettime(self.as_raw_fd(), &mut time) };
        let init = Duration::new(time.it_value.tv_sec as _, time.it_value.tv_nsec as _);
        let ival = Duration::new(time.it_interval.tv_sec as _, time.it_interval.tv_nsec as _);
        (init, ival)
    }

    /// Checks the expiration status.
    #[inline]
    pub fn read(&self) -> Result<u64> {
        let mut n = [0u8; _];
        let res = unsafe { libc::read(self.as_raw_fd(), n.as_mut_ptr().cast(), n.len()) };
        if res != -1 { Ok(u64::from_ne_bytes(n)) } else { Err(Error::errno(Kind::Read)) }
    }
}

// ===== ClockKind =====

/// The clock that is used to mark the progress of the [`Timerfd`].
#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct ClockKind(i32);

impl ClockKind {
    /// A settable system-wide real-time clock.
    pub const REALTIME: Self = Self(libc::CLOCK_REALTIME);
    /// A nonsettable monotonically increasing clock that measures time from some unspecified point
    /// in the past that does not change after system startup.
    pub const MONOTONIC: Self = Self(libc::CLOCK_MONOTONIC);
    /// Like CLOCK_MONOTONIC, this is a monotonically increasing clock.
    ///
    /// However, whereas the [`ClockKind::MONOTONIC`] clock does not measure the time while a system
    /// is suspended, the [`ClockKind::BOOTTIME`] clock does include the time during which the
    /// system is suspended. This is useful for applications that need to be suspend-aware.
    /// [`ClockKind::REALTIME`] is not suitable for such applications, since that clock is affected
    /// by discontinuous changes to the system clock.
    pub const BOOTTIME: Self = Self(libc::CLOCK_BOOTTIME);
    /// This clock is like [`ClockKind::REALTIME`], but will wake the system if it is suspended.
    ///
    /// The caller must have the CAP_WAKE_ALARM capability in order to set a timer against this
    /// clock.
    pub const REALTIME_ALARM: Self = Self(libc::CLOCK_REALTIME_ALARM);
    /// This clock is like [`ClockKind::BOOTTIME`], but will wake the system if it is suspended.
    ///
    /// The caller must have the CAP_WAKE_ALARM capability in order to set a timer against this
    /// clock.
    pub const BOOTTIME_ALARM: Self = Self(libc::CLOCK_BOOTTIME_ALARM);
}

// ===== Flags =====

/// [`Timerfd`] creation flags.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Flags(i32);

impl Flags {
    /// Set the close-on-exec (FD_CLOEXEC) flag on the new fd.
    pub const CLOEXEC: Self = Self(libc::TFD_CLOEXEC);
    /// Set the `O_NONBLOCK` file status flag on the new fd.
    pub const NONBLOCK: Self = Self(libc::TFD_NONBLOCK);
}

impl OpenFlag for Flags {
    const CLOEXEC: Self = Self::CLOEXEC;
    const NONBLOCK: Self = Self::NONBLOCK;
}

// ===== TimerFlags =====

/// [`Timerfd::set_time`] flags.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct TimerFlags(i32);

impl TimerFlags {
    /// Interpret new_value.it_value as an absolute value on the timer's clock.
    ///
    /// The timer will expire when the value of the timer's clock reaches the value specified in
    /// new_value.it_value.
    pub const ABSTIME: Self = Self(libc::TFD_TIMER_ABSTIME);
    /// If this flag is specified along with [`TimerFlags::ABSTIME`] and the clock for this timer is
    /// [`ClockKind::REALTIME`] or [`ClockKind::REALTIME_ALARM`], then mark this timer as cancelable
    /// if the real-time clock undergoes a discontinuous change (`settimeofday(2)`,
    /// `clock_settime(2)`, or similar).
    ///
    /// When such changes occur, a current or future `read(2)` will fail with the error ECANCELED.
    pub const CANCEL_ON_SET: Self = Self(libc::TFD_TIMER_CANCEL_ON_SET);
}

// ===== Error =====

fn e(res: i32, kind: Kind) -> Result<()> {
    if res == -1 {
        return Err(Error::errno(kind));
    }
    Ok(())
}

fn fd<T: FromRawFd>(res: i32, kind: Kind) -> Result<T> {
    if res == -1 {
        return Err(Error::errno(kind));
    }
    Ok(unsafe { T::from_raw_fd(res) })
}

/// Type alias for result of [`Timerfd`] operations.
pub type Result<T, E = Error> = result::Result<T, E>;

/// An error that may occur during any [`Timerfd`] operations.
#[derive(Debug, Clone)]
pub struct Error {
    kind: Kind,
    code: ErrCode,
}

impl Error {
    fn errno(kind: Kind) -> Error {
        Self { kind, code: ErrCode::errno() }
    }
}

#[derive(Debug, Clone, Copy)]
enum Kind {
    Create,
    Set,
    Read,
}

impl AsErrCode for Error {
    #[inline]
    fn as_err_code(&self) -> ErrCode {
        self.code
    }
}

impl error::Error for Error {}

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

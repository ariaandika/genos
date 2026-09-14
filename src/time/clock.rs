use core::mem::MaybeUninit;
use core::{fmt, result};

use crate::error::{self, ErrCode, SysResExt};
use crate::ffi::Char;
use crate::sys;
use crate::time::Timespec;

// ===== Clock =====

/// Clock Identofier.
///
/// Reference: `clock_getres(2)`.
#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct Clock(i32);

impl Clock {
    /// A settable system-wide real-time clock.
    pub const REALTIME: Self = Self(sys::CLOCK_REALTIME);
    /// A nonsettable monotonically increasing clock that measures time from some unspecified point
    /// in the past that does not change after system startup.
    pub const MONOTONIC: Self = Self(sys::CLOCK_MONOTONIC);
    /// Like CLOCK_MONOTONIC, this is a monotonically increasing clock.
    pub const BOOTTIME: Self = Self(sys::CLOCK_BOOTTIME);
    /// This clock is like [`Clock::REALTIME`], but will wake the system if it is suspended.
    pub const REALTIME_ALARM: Self = Self(sys::CLOCK_REALTIME_ALARM);
    /// This clock is like [`Clock::BOOTTIME`], but will wake the system if it is suspended.
    pub const BOOTTIME_ALARM: Self = Self(sys::CLOCK_BOOTTIME_ALARM);
    /// Similar to [`Clock::MONOTONIC`], but provides access to a raw hardware-based time that is
    /// not sub‐ ject to frequency adjustments.
    pub const MONOTONIC_RAW: Self = Self(sys::CLOCK_MONOTONIC_RAW);
    /// A faster but less precise version of [`Clock::REALTIME`].
    pub const REALTIME_COARSE: Self = Self(sys::CLOCK_REALTIME_COARSE);
    /// A faster but less precise version of [`Clock::MONOTONIC`].
    pub const MONOTONIC_COARSE: Self = Self(sys::CLOCK_MONOTONIC_COARSE);
    /// A system-wide clock derived from wall-clock time but counting leap seconds.
    pub const TAI: Self = Self(sys::CLOCK_TAI);
}

impl Clock {
    /// Symbol name for [`Clock::time`] exported by vDSO.
    pub const GETTIME_VDSO_SYM: &Char = Char::new(c"__vdso_clock_gettime");

    /// Finds the resolution (precision) of this clock.
    #[inline]
    pub fn res(self) -> Result<Timespec> {
        let mut ts = MaybeUninit::uninit();
        sys::call!(sys_clock_getres, self.0, &mut ts).e(Kind::Res)?;
        Ok(unsafe { ts.assume_init() })
    }

    /// Retrieve the time of this clock.
    #[inline]
    pub fn time(self) -> Result<Timespec> {
        let mut ts = MaybeUninit::uninit();
        sys::call!(sys_clock_gettime, self.0, &mut ts).e(Kind::Get)?;
        Ok(unsafe { ts.assume_init() })
    }

    /// Set the time of this clock.
    #[inline]
    pub fn set_time(self, time: &Timespec) -> Result<()> {
        sys::call_rd!(sys_clock_settime, self.0, time).e(Kind::Set)
    }
}

impl From<Clock> for i32 {
    #[inline]
    fn from(value: Clock) -> Self {
        value.0
    }
}

// ===== Error =====

/// Type alias for result of [`Clock`] operations.
pub type Result<T, E = Error> = result::Result<T, E>;

/// An error that may occur during any [`Clock`] operations.
#[derive(Debug, Clone)]
pub struct Error {
    kind: Kind,
    code: ErrCode,
}

#[derive(Debug, Clone, Copy)]
enum Kind {
    Res,
    Get,
    Set,
}

error::impl_error_with_kind!(Error, Kind);

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self { kind, code } = self;
        let msg = match kind {
            Kind::Res => "get clock resolution",
            Kind::Get => "get clock time",
            Kind::Set => "set clock time",
        };
        write!(f, "failed to {msg}: {code}")
    }
}

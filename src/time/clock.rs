//! [`Clock`] associated types.
use core::mem::MaybeUninit;
use core::{fmt, result};

use crate::error::{self, ErrCode, SysResExt};
use crate::ffi::Char;
use crate::sys;
use crate::time::Timespec;

// ===== Clock =====

/// Clock Identifier.
#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct Clock(i32);

impl Clock {
    /// `CLOCK_REALTIME`
    pub const REALTIME: Self = Self(CLOCK_REALTIME);
    /// `CLOCK_MONOTONIC`
    pub const MONOTONIC: Self = Self(CLOCK_MONOTONIC);
    /// `CLOCK_BOOTTIME`
    pub const BOOTTIME: Self = Self(CLOCK_BOOTTIME);
    /// `CLOCK_REALTIME_ALARM`
    pub const REALTIME_ALARM: Self = Self(CLOCK_REALTIME_ALARM);
    /// `CLOCK_BOOTTIME_ALARM`
    pub const BOOTTIME_ALARM: Self = Self(CLOCK_BOOTTIME_ALARM);
    /// `CLOCK_MONOTONIC_RAW`
    pub const MONOTONIC_RAW: Self = Self(CLOCK_MONOTONIC_RAW);
    /// `CLOCK_REALTIME_COARSE`
    pub const REALTIME_COARSE: Self = Self(CLOCK_REALTIME_COARSE);
    /// `CLOCK_MONOTONIC_COARSE`
    pub const MONOTONIC_COARSE: Self = Self(CLOCK_MONOTONIC_COARSE);
    /// `CLOCK_TAI`
    pub const TAI: Self = Self(CLOCK_TAI);
}

impl Clock {
    /// Symbol name for [`Clock::time`] exported by vDSO.
    pub const GETTIME_VDSO_SYM: &Char = Char::new(c"__vdso_clock_gettime");

    /// Finds the resolution (precision) of this clock (`clock_getres(2)`).
    #[inline]
    pub fn res(self) -> Result<Timespec> {
        let mut ts = MaybeUninit::uninit();
        sys::call!(sys_clock_getres, self.0, &mut ts).e(Kind::Res)?;
        Ok(unsafe { ts.assume_init() })
    }

    /// Retrieve the time of this clock (`clock_gettime(2)`).
    #[inline]
    pub fn time(self) -> Result<Timespec> {
        let mut ts = MaybeUninit::uninit();
        sys::call!(sys_clock_gettime, self.0, &mut ts).e(Kind::Get)?;
        Ok(unsafe { ts.assume_init() })
    }

    /// Set the time of this clock (`clock_settime(2)`).
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

// ===== extern =====

// include/uapi/linux/time.h

const CLOCK_REALTIME: i32 = 0;
const CLOCK_MONOTONIC: i32 = 1;
// const CLOCK_PROCESS_CPUTIME_ID: i32 = 2;
// const CLOCK_THREAD_CPUTIME_ID: i32 = 3;
const CLOCK_MONOTONIC_RAW: i32 = 4;
const CLOCK_REALTIME_COARSE: i32 = 5;
const CLOCK_MONOTONIC_COARSE: i32 = 6;
const CLOCK_BOOTTIME: i32 = 7;
const CLOCK_REALTIME_ALARM: i32 = 8;
const CLOCK_BOOTTIME_ALARM: i32 = 9;
const CLOCK_TAI: i32 = 11;

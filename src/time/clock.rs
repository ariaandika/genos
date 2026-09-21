//! [`Clock`] associated types.
use core::mem::MaybeUninit;

use crate::ffi::Char;
use crate::flags;
use crate::sys::{self, Error, arch};
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
    /// Symbol name for [`Clock::get_time`] exported by vDSO.
    pub const GETTIME_VDSO_SYM: &Char = Char::new(c"__vdso_clock_gettime");

    /// Finds the resolution (precision) of this clock (`clock_getres(2)`).
    #[inline]
    pub fn get_res(
        self,
        res: &mut MaybeUninit<Timespec>,
    ) -> Result<(), Error<arch::sys_clock_getres>> {
        sys::call!(sys_clock_getres, self.0, res)
    }

    /// Retrieve the time of this clock (`clock_gettime(2)`).
    #[inline]
    pub fn get_time(
        self,
        tp: &mut MaybeUninit<Timespec>,
    ) -> Result<(), Error<arch::sys_clock_gettime>> {
        sys::call!(sys_clock_gettime, self.0, tp)
    }

    /// Set the time of this clock (`clock_settime(2)`).
    #[inline]
    pub fn set_time(self, tp: &Timespec) -> Result<(), Error<arch::sys_clock_settime>> {
        sys::call_rd!(sys_clock_settime, self.0, tp)
    }

    /// High-resolution sleep with this clock.
    #[inline]
    pub fn nanosleep(
        self,
        flags: Flags,
        t: &Timespec,
        remain: Option<&mut Timespec>,
    ) -> Result<(), Error<arch::sys_clock_nanosleep>> {
        sys::call!(sys_clock_nanosleep, flags.0, t, sys::optmut(remain))
    }
}

impl From<Clock> for i32 {
    #[inline]
    fn from(value: Clock) -> Self {
        value.0
    }
}

// ===== Flags =====

/// `clock_nanosleep(2)` flags.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Flags(i32);

flags::impl_bitops_simple!(Flags);

impl Flags {
    /// `TIMER_ABSTIME`
    pub const ABSTIME: Flags = Flags(TIMER_ABSTIME);
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

const TIMER_ABSTIME: i32 = 0x01;

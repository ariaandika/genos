use core::ffi;

use crate::ffi::{Char, Time};
use crate::sys::{self, SysRes};

/// Symbol name for [`time`] exported by vDSO.
pub const TIME_VDSO_SYM: &Char = Char::new(c"__vdso_time");

/// Get time in seconds since Epoch, `1970-01-01 00:00:00 +0000 (UTC)` (`time(2)`).
#[inline]
pub fn time(tloc: Option<&mut Time>) -> impl SysRes<Time> {
    sys::call!(sys_time, sys::optmut(tloc))
}

/// High-resolution sleep (`nanosleep(2)`).
#[inline]
pub fn nanosleep(duration: &Timespec, rem: Option<&mut Timespec>) -> impl SysRes<()> {
    sys::call!(sys_nanosleep, duration, sys::optmut(rem))
}

// ===== Timespec =====

/// Time in seconds and nanoseconds.
#[derive(Debug)]
#[repr(C)]
pub struct Timespec {
    /// Seconds.
    pub tv_sec: ffi::c_long,
    /// Nanoseconds.
    pub tv_nsec: ffi::c_long,
}

// ===== ITimerspec =====

/// Interval for a timer with nanosecond precision.
#[derive(Debug)]
#[repr(C)]
pub struct ITimerspec {
    /// Interval for periodic timer.
    pub it_interval: Timespec,
    /// Initial expiration.
    pub it_value: Timespec,
}

// ===== extern =====

// include/uapi/linux/time.h

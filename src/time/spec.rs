use core::ffi;

// source: include/uapi/linux/time.h

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

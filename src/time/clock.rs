use crate::sys;

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

impl From<Clock> for i32 {
    #[inline]
    fn from(value: Clock) -> Self {
        value.0
    }
}

//! Time management.
pub use time::{ITimerspec, TIME_VDSO_SYM, Timespec, nanosleep, time};
pub use clock::Clock;
#[doc(inline)]
pub use timerfd::Timerfd;

mod time;
pub mod clock;
pub mod timerfd;

//! Time management.
pub use time::{ITimerspec, TIME_VDSO_SYM, Timespec, time};
pub use clock::Clock;
#[doc(inline)]
pub use timerfd::Timerfd;

mod time;
mod clock;
pub mod timerfd;

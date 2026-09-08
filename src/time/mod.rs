//! Time management.
pub use spec::{Timespec, ITimerspec};
pub use clock::Clock;
pub use time::{TIME_VDSO_SYM, time};
pub use timerfd::Timerfd;

mod spec;
mod clock;
mod time;
pub mod timerfd;

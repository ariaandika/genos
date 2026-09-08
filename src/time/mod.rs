//! Time management.
pub use spec::{Timespec, ITimerspec};
pub use clock::Clock;
pub use timerfd::Timerfd;

mod spec;
mod clock;
pub mod timerfd;


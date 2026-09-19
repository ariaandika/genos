//! Signal management.
pub use signo::Signo;
#[doc(inline)]
pub use sigset::{Sigset, rt_sigprocmask};
#[doc(inline)]
pub use signalfd::Signalfd;

mod signo;
mod sigset;
pub mod signalfd;

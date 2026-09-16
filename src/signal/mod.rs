//! Signal management.
pub use signo::Signo;
#[doc(inline)]
pub use sigset::Sigset;
#[doc(inline)]
pub use signalfd::Signalfd;

mod signo;
pub mod sigset;
pub mod signalfd;

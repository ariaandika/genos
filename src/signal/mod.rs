//! Signal management.
pub use signo::Signo;
pub use sigset::Sigset;
pub use signalfd::Signalfd;

mod signo;
pub mod sigset;
pub mod signalfd;

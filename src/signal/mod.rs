//! Signal management.
pub use signalfd::Signalfd;
pub use signo::Signo;
pub use sigset::Sigset;

pub mod signalfd;
mod signo;
mod sigset;

/// Signal operation error types.
pub mod error {
    pub use super::sigset::ProcSignalError;
}

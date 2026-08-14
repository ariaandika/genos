//! Signal management.
pub use signo::Signo;
pub use sigset::Sigset;
pub use signalfd::Signalfd;

mod signo;
mod sigset;
pub mod signalfd;

/// Signal operation error types.
pub mod error {
    pub use super::sigset::ProcSignalError;
}

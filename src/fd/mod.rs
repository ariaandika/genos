//! Owned and borrowed linux file descriptors.
pub use fd::{BorrowedFd, OwnedFd, RawFd};
pub use traits::{AsFd, FromRawFd, IntoRawFd};
pub use fcntl::Open;

pub(crate) use traits::impl_fd_simple;
pub(crate) use fcntl::{O_CLOEXEC, O_NONBLOCK};

mod fd;
mod traits;
mod fcntl;

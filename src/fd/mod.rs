//! Owned and borrowed linux file descriptors.
pub use fd::{BorrowedFd, OwnedFd, RawFd};
pub use traits::{AsFd, FromRawFd, IntoRawFd};

pub(crate) use traits::impl_fd_simple;

mod fd;
mod traits;

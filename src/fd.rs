//! Owned and borrowed linux file descriptors.
pub use std::os::fd::*;

macro_rules! impl_fd_simple {
    ($me:ident) => {
        impl crate::fd::FromRawFd for $me {
            #[inline]
            unsafe fn from_raw_fd(fd: crate::fd::RawFd) -> Self {
                Self(unsafe { <_>::from_raw_fd(fd) })
            }
        }
        impl crate::fd::AsFd for $me {
            #[inline]
            fn as_fd(&self) -> crate::fd::BorrowedFd<'_> {
                self.0.as_fd()
            }
        }
        impl crate::fd::AsRawFd for $me {
            #[inline]
            fn as_raw_fd(&self) -> crate::fd::RawFd {
                self.0.as_raw_fd()
            }
        }
        impl crate::fd::IntoRawFd for $me {
            #[inline]
            fn into_raw_fd(self) -> crate::fd::RawFd {
                self.0.into_raw_fd()
            }
        }
    };
}
pub(crate) use impl_fd_simple;

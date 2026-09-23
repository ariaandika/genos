use core::mem;

use crate::fd::{BorrowedFd, OwnedFd, RawFd};

// ===== traits =====

/// A trait to express the ability to construct an object from a raw file descriptor.
pub trait FromRawFd {
    /// Creates this type from given file descriptor.
    ///
    /// # Safety
    ///
    /// The `fd` passed in must be an owned file descriptor; in particular, it must be open.
    unsafe fn from_raw_fd(fd: RawFd) -> Self;
}

/// A trait to borrow the file descriptor from an underlying object.
pub trait AsFd {
    /// Borrows the file descriptor.
    fn as_fd(&self) -> BorrowedFd<'_>;

    /// Extracts the raw file descriptor.
    #[inline]
    fn as_raw_fd(&self) -> RawFd {
        self.as_fd().raw()
    }
}

/// A trait to express the ability to consume an object and acquire ownership of its raw file
/// descriptor.
pub trait IntoRawFd {
    /// Consumes this object, returning the raw underlying file descriptor.
    ///
    /// This function is typically used to **transfer ownership** of the underlying file descriptor
    /// to the caller. When used in this way, callers are then the unique owners of the file
    /// descriptor and must close it once it's no longer needed.
    #[must_use = "losing the raw file descriptor may leak resources"]
    fn into_raw_fd(self) -> RawFd;
}

// ===== impls =====

impl FromRawFd for RawFd {
    #[inline]
    unsafe fn from_raw_fd(fd: RawFd) -> Self {
        fd
    }
}

impl FromRawFd for OwnedFd {
    #[inline]
    unsafe fn from_raw_fd(fd: RawFd) -> Self {
        Self::from_raw(fd)
    }
}

impl AsFd for BorrowedFd<'_> {
    #[inline]
    fn as_fd(&self) -> BorrowedFd<'_> {
        *self
    }
}

impl AsFd for OwnedFd {
    #[inline]
    fn as_fd(&self) -> BorrowedFd<'_> {
        unsafe { BorrowedFd::borrow_raw(self.raw()) }
    }
}

impl IntoRawFd for RawFd {
    #[inline]
    fn into_raw_fd(self) -> RawFd {
        self
    }
}

impl IntoRawFd for OwnedFd {
    #[inline]
    fn into_raw_fd(self) -> RawFd {
        mem::ManuallyDrop::new(self).raw()
    }
}

impl<T: AsFd + ?Sized> AsFd for &T {
    #[inline]
    fn as_fd(&self) -> BorrowedFd<'_> {
        T::as_fd(self)
    }
}

impl<T: AsFd + ?Sized> AsFd for &mut T {
    #[inline]
    fn as_fd(&self) -> BorrowedFd<'_> {
        T::as_fd(self)
    }
}

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
        impl crate::fd::IntoRawFd for $me {
            #[inline]
            fn into_raw_fd(self) -> crate::fd::RawFd {
                self.0.into_raw_fd()
            }
        }
        impl From<$me> for crate::fd::OwnedFd {
            #[inline]
            fn from(value: $me) -> Self {
                value.0
            }
        }
    };
}
pub(crate) use impl_fd_simple;

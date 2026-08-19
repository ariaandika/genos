//! Owned and borrowed linux file descriptors.
use core::{ffi, fmt, marker, mem};

// ===== types =====

/// Raw file descriptors.
pub type RawFd = ffi::c_int;

// core::num::niche_types::NotAllOnes<RawFd>;
type ValidRawFd = RawFd;

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
}

/// A trait to extract the raw file descriptor from an underlying object.
pub trait AsRawFd {
    /// Extracts the raw file descriptor.
    fn as_raw_fd(&self) -> RawFd;
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

// ===== BorrowedFd =====

/// A borrowed file descriptor.
///
/// This has a lifetime parameter to tie it to the lifetime of something that owns the file
/// descriptor. For the duration of that lifetime, it is guaranteed that nobody will close the file
/// descriptor.
///
/// This uses `repr(transparent)` and has the representation of a host file descriptor, so it can be
/// used in FFI in places where a file descriptor is passed as an argument, it is not captured or
/// consumed.
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct BorrowedFd<'fd> {
    fd: ValidRawFd,
    _p: marker::PhantomData<&'fd OwnedFd>,
}

impl BorrowedFd<'_> {
    /// Returns a `BorrowedFd` holding the given raw file descriptor.
    ///
    /// # Safety
    ///
    /// The resource pointed to by `fd` must remain open for the duration of the returned
    /// `BorrowedFd`.
    #[inline]
    pub const unsafe fn borrow_raw(fd: RawFd) -> Self {
        Self { fd, _p: marker::PhantomData }
    }
}

// ===== OwnedFd =====

/// An owned file descriptor.
///
/// This closes the file descriptor on drop. It is guaranteed that nobody else will close the file
/// descriptor.
///
/// This uses `repr(transparent)` and has the representation of a host file descriptor, so it can be
/// used in FFI in places where a file descriptor is passed as a consumed argument or returned as an
/// owned value.
#[derive(Debug)]
#[repr(transparent)]
pub struct OwnedFd(ValidRawFd);

impl Drop for OwnedFd {
    #[inline]
    fn drop(&mut self) {
        crate::sys::call!(RD, __NR_close, self.0);
    }
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
        Self(fd)
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
        unsafe { BorrowedFd::borrow_raw(self.0) }
    }
}

impl AsRawFd for RawFd {
    #[inline]
    fn as_raw_fd(&self) -> RawFd {
        *self
    }
}

impl AsRawFd for BorrowedFd<'_> {
    #[inline]
    fn as_raw_fd(&self) -> RawFd {
        self.fd
    }
}

impl AsRawFd for OwnedFd {
    #[inline]
    fn as_raw_fd(&self) -> RawFd {
        self.0
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
        mem::ManuallyDrop::new(self).0
    }
}

impl fmt::Debug for BorrowedFd<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("BorrowedFd").field(&self.fd).finish()
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

impl<T: AsRawFd + ?Sized> AsRawFd for &T {
    #[inline]
    fn as_raw_fd(&self) -> RawFd {
        T::as_raw_fd(self)
    }
}

impl<T: AsRawFd + ?Sized> AsRawFd for &mut T {
    #[inline]
    fn as_raw_fd(&self) -> RawFd {
        T::as_raw_fd(self)
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
        impl From<$me> for crate::fd::OwnedFd {
            #[inline]
            fn from(value: $me) -> Self {
                value.0
            }
        }
    };
}
pub(crate) use impl_fd_simple;

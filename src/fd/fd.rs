use core::{ffi, fmt, marker};

use crate::sys;

// ===== types =====

/// Raw file descriptors.
pub type RawFd = ffi::c_int;

// core::num::niche_types::NotAllOnes<RawFd>;
type ValidRawFd = RawFd;

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

    /// Returns the raw fd integer.
    #[inline]
    pub const fn raw(&self) -> RawFd {
        self.fd
    }
}

impl fmt::Debug for BorrowedFd<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("BorrowedFd").field(&self.fd).finish()
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

impl OwnedFd {
    pub(super) const fn from_raw(raw: i32) -> Self {
        Self(raw)
    }

    /// Returns the raw fd integer.
    #[inline]
    pub const fn raw(&self) -> RawFd {
        self.0
    }
}

impl Drop for OwnedFd {
    #[inline]
    fn drop(&mut self) {
        sys::call_rd!(sys_close, self.0);
    }
}

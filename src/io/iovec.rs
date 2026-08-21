//! I/O vector buffer.
use core::{fmt, marker, mem, slice};

use crate::sys;

// ===== IoVec =====

/// Vector I/O data structure.
///
/// See `iovec(3type)`.
#[repr(C)]
pub struct IoVec<'a> {
    iov: sys::iovec,
    _p: marker::PhantomData<&'a [u8]>,
}

impl<'a> IoVec<'a> {
    /// Creates new [`IoVec`].
    #[inline]
    pub const fn new(iov: &'a [u8]) -> Self {
        Self {
            iov: sys::iovec { iov_base: iov.as_ptr().cast_mut().cast(), iov_len: iov.len() },
            _p: marker::PhantomData,
        }
    }

    /// Returns the bytes slice.
    #[inline]
    pub const fn as_slice(&self) -> &'a [u8] {
        unsafe { slice::from_raw_parts(self.iov.iov_base.cast(), self.iov.iov_len) }
    }
}

impl<'a> From<&'a [u8]> for IoVec<'a> {
    #[inline]
    fn from(value: &'a [u8]) -> Self {
        Self::new(value)
    }
}

impl<'a> fmt::Debug for IoVec<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("IoVec").finish_non_exhaustive()
    }
}

// ===== IoVecMut =====

/// Vector I/O data structure.
///
/// See `iovec(3type)`.
#[repr(C)]
pub struct IoVecMut<'a> {
    iov: sys::iovec,
    _p: marker::PhantomData<&'a mut [mem::MaybeUninit<u8>]>,
}

impl<'a> IoVecMut<'a> {
    /// Creates new [`IoVecMut`].
    #[inline]
    pub const fn new(iov: &'a mut [u8]) -> Self {
        Self {
            iov: sys::iovec { iov_base: iov.as_mut_ptr().cast(), iov_len: iov.len() },
            _p: marker::PhantomData,
        }
    }

    /// Creates new [`IoVecMut`] from uninitialized bytes.
    #[inline]
    pub const fn from_uninit(iov: &'a mut [mem::MaybeUninit<u8>]) -> Self {
        Self {
            iov: sys::iovec { iov_base: iov.as_mut_ptr().cast(), iov_len: iov.len() },
            _p: marker::PhantomData,
        }
    }
}

impl<'a> From<&'a mut [u8]> for IoVecMut<'a> {
    #[inline]
    fn from(value: &'a mut [u8]) -> Self {
        Self::new(value)
    }
}

impl<'a> From<&'a mut [mem::MaybeUninit<u8>]> for IoVecMut<'a> {
    #[inline]
    fn from(value: &'a mut [mem::MaybeUninit<u8>]) -> Self {
        Self::from_uninit(value)
    }
}

impl<'a> fmt::Debug for IoVecMut<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("IoVecMut").finish_non_exhaustive()
    }
}

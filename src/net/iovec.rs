//! I/O vector buffer.
use core::{marker, slice};

// ===== IoVec =====

/// Vector I/O data structure.
#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct IoVec<'a> {
    iov: libc::iovec,
    _p: marker::PhantomData<&'a ()>,
}

impl<'a> IoVec<'a> {
    /// Creates new [`IoVec`].
    #[inline]
    pub const fn new(iov: &'a [u8]) -> Self {
        let iov = libc::iovec { iov_base: iov.as_ptr().cast_mut().cast(), iov_len: iov.len() };
        Self { _p: marker::PhantomData, iov }
    }

    /// Returns the contained bytes.
    #[inline]
    pub const fn into_inner(self) -> &'a [u8] {
        unsafe { slice::from_raw_parts(self.iov.iov_base.cast(), self.iov.iov_len) }
    }
}

impl<'a> From<IoVec<'a>> for &'a [u8] {
    #[inline]
    fn from(value: IoVec<'a>) -> Self {
        value.into_inner()
    }
}

// ===== IoVecMut =====

/// Vector I/O data structure.
#[derive(Debug, Clone)]
#[repr(transparent)]
pub struct IoVecMut<'a> {
    iov: libc::iovec,
    _p: marker::PhantomData<&'a ()>,
}

impl<'a> IoVecMut<'a> {
    /// Creates new [`IoVecMut`].
    #[inline]
    pub const fn new(iov: &'a mut [u8]) -> Self {
        let iov = libc::iovec { iov_base: iov.as_mut_ptr().cast(), iov_len: iov.len() };
        Self { _p: marker::PhantomData, iov }
    }

    /// Returns the contained bytes.
    #[inline]
    pub const fn into_inner(self) -> &'a mut [u8] {
        unsafe { slice::from_raw_parts_mut(self.iov.iov_base.cast(), self.iov.iov_len) }
    }
}

impl<'a> From<IoVecMut<'a>> for &'a mut [u8] {
    #[inline]
    fn from(value: IoVecMut<'a>) -> Self {
        value.into_inner()
    }
}

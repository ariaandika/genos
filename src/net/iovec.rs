//! I/O vector buffer.
use core::{fmt, marker, slice};

// ===== IoVec =====

// source: include/uapi/linux/uio.h

/// Vector I/O data structure.
#[repr(C)]
pub struct IoVec<'a> {
    iov_base: *const u8,
    iov_len: usize,
    _p: marker::PhantomData<&'a [u8]>,
}

impl<'a> IoVec<'a> {
    /// Creates new [`IoVec`].
    #[inline]
    pub const fn new(iov: &'a [u8]) -> Self {
        Self {
            _p: marker::PhantomData,
            iov_base: iov.as_ptr().cast_mut().cast(),
            iov_len: iov.len(),
        }
    }

    /// Returns the shared bytes slice.
    #[inline]
    pub const fn as_slice(&self) -> &'a [u8] {
        unsafe { slice::from_raw_parts(self.iov_base, self.iov_len) }
    }
}

impl<'a> From<IoVec<'a>> for &'a [u8] {
    #[inline]
    fn from(value: IoVec<'a>) -> Self {
        value.as_slice()
    }
}

impl<'a> fmt::Debug for IoVec<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_slice().fmt(f)
    }
}

// ===== IoVecMut =====

/// Vector I/O data structure.
#[repr(C)]
pub struct IoVecMut<'a> {
    iov_base: *mut u8,
    iov_len: usize,
    _p: marker::PhantomData<&'a mut ()>,
}

impl<'a> IoVecMut<'a> {
    /// Creates new [`IoVecMut`].
    #[inline]
    pub const fn new(iov: &'a mut [u8]) -> Self {
        Self { iov_base: iov.as_mut_ptr().cast(), iov_len: iov.len(), _p: marker::PhantomData }
    }

    fn as_slice(&self) -> &'a [u8] {
        unsafe { slice::from_raw_parts(self.iov_base, self.iov_len) }
    }

    /// Returns the contained bytes.
    #[inline]
    pub const fn into_inner(self) -> &'a mut [u8] {
        unsafe { slice::from_raw_parts_mut(self.iov_base.cast(), self.iov_len) }
    }
}

impl<'a> From<IoVecMut<'a>> for &'a mut [u8] {
    #[inline]
    fn from(value: IoVecMut<'a>) -> Self {
        value.into_inner()
    }
}

impl<'a> fmt::Debug for IoVecMut<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_slice().fmt(f)
    }
}

use core::{fmt, marker, mem};

// ===== IoVec =====

/// Vector I/O data structure (`iovec(3type)`).
#[repr(C)]
pub struct IoVec<'a> {
    iov_base: *const u8,
    iov_len: usize,
    _p: marker::PhantomData<&'a [u8]>,
}

impl<'a> IoVec<'a> {
    /// `UIO_FASTIOV`
    pub const FASTIOV: usize = UIO_FASTIOV;

    /// `UIO_MAXIOV`
    pub const MAXIOV: usize = UIO_MAXIOV;

    /// Creates new [`IoVec`].
    #[inline]
    pub(crate) const fn new(base: *const u8, len: usize) -> Self {
        Self { iov_base: base, iov_len: len, _p: marker::PhantomData }
    }

    /// Creates new [`IoVec`] from slice.
    #[inline]
    pub const fn from_slice(iov: &'a [u8]) -> Self {
        Self::new(iov.as_ptr(), iov.len())
    }
}

impl<'a> From<&'a [u8]> for IoVec<'a> {
    #[inline]
    fn from(value: &'a [u8]) -> Self {
        Self::from_slice(value)
    }
}

impl<'a> fmt::Debug for IoVec<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("IoVec").finish_non_exhaustive()
    }
}

// ===== IoVecMut =====

/// Vector I/O data structure (`iovec(3type)`).
#[repr(C)]
pub struct IoVecMut<'a> {
    iov_base: *mut mem::MaybeUninit<u8>,
    iov_len: usize,
    _p: marker::PhantomData<&'a mut [mem::MaybeUninit<u8>]>,
}

impl<'a> IoVecMut<'a> {
    /// `UIO_FASTIOV`
    pub const FASTIOV: usize = UIO_FASTIOV;

    /// `UIO_MAXIOV`
    pub const MAXIOV: usize = UIO_MAXIOV;

    /// Creates new [`IoVecMut`].
    #[inline]
    pub(crate) const fn new(base: *mut mem::MaybeUninit<u8>, len: usize) -> Self {
        Self { iov_base: base, iov_len: len, _p: marker::PhantomData }
    }

    /// Creates new [`IoVecMut`] from slice.
    #[inline]
    pub const fn from_slice(iov: &'a mut [mem::MaybeUninit<u8>]) -> Self {
        Self::new(iov.as_mut_ptr(), iov.len())
    }
}

impl<'a> From<&'a mut [mem::MaybeUninit<u8>]> for IoVecMut<'a> {
    #[inline]
    fn from(value: &'a mut [mem::MaybeUninit<u8>]) -> Self {
        Self::from_slice(value)
    }
}

impl<'a> fmt::Debug for IoVecMut<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("IoVecMut").finish_non_exhaustive()
    }
}

// ===== extern =====

// include/uapi/linux/uio.h

const UIO_FASTIOV: usize = 8;
const UIO_MAXIOV: usize = 1024;

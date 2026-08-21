//! Socket control message.
use core::{ffi, marker, mem};

use crate::net::msg::{AncillaryData, sealed};
use crate::sys;

// ===== CMsgKind =====

/// Control message kind.
pub trait CMsgKind {
    /// Control message data type.
    type Data;

    /// Control message type.
    const TYPE: CMsgType;
}

// ===== CMsgType =====

/// Control message type.
#[derive(Debug, Clone, Copy)]
pub struct CMsgType(i32);

impl CMsgType {
    /// Send or receive a set of open file descriptors from another process.
    pub const RIGHTS: Self = Self(sys::SCM_RIGHTS);
}

impl From<CMsgType> for i32 {
    #[inline]
    fn from(value: CMsgType) -> Self {
        value.0
    }
}

// ===== cmsg =====

// control message are packed header with arbitrary data:
//
// ```
// struct cmsghdr {
//    size_t cmsg_len;
//    int    cmsg_level;
//    int    cmsg_type;
//    /* followed by unsigned char cmsg_data[]; */
// };
// ```
//
// - size: size_of<cmsghdr>() + size_of<data>() + padding
// - align: align_of<cmsghdr>().max(align_of<data>())

// ===== CMsgArray =====

/// Control message array.
#[derive(Debug)]
#[repr(C)]
pub struct CMsgArray<T: CMsgKind + ?Sized, const N: usize> {
    hdr: sys::cmsghdr,
    data: [mem::MaybeUninit<T::Data>; N],
    _kind: marker::PhantomData<fn() -> T>,
}

impl<T: CMsgKind + ?Sized, const N: usize> CMsgArray<T, N> {
    /// Creates [`CMsgArray`] with uninitialized data.
    #[inline]
    pub const fn uninit() -> Self {
        let data = [const { mem::MaybeUninit::uninit() }; N];
        Self::new_inner(data, size_of::<[T::Data; 0]>())
    }

    /// Creates [`CMsgArray`] with given type and data.
    #[inline]
    pub fn new(data: [T::Data; N]) -> Self {
        // [`MaybeUninit::transpose`]: https://github.com/rust-lang/rust/issues/96097
        let data = mem::MaybeUninit::new(data).into();
        Self::new_inner(data, size_of::<[T::Data; N]>())
    }

    const fn new_inner(data: [mem::MaybeUninit<T::Data>; N], data_len: usize) -> Self {
        const { assert!(sys::CMSG_SPACE(size_of::<[T::Data; N]>()) == size_of::<Self>()) };
        let hdr = sys::cmsghdr {
            cmsg_len: sys::CMSG_LEN(data_len),
            cmsg_level: sys::SOL_SOCKET,
            cmsg_type: T::TYPE.0,
        };
        Self { hdr, data, _kind: marker::PhantomData }
    }

    /// Returns length of the initialized data.
    #[inline]
    pub const fn len(&self) -> usize {
        self.hdr.data_len() / size_of::<T::Data>()
    }

    /// Returns the initialized data as slice.
    #[inline]
    pub fn data(&self) -> &[T::Data] {
        // SAFETY: the length is set since the constructor or by the kernel
        unsafe { self.data.get_unchecked(..self.len()).assume_init_ref() }
    }
}

impl<T: CMsgKind + ?Sized, const N: usize> AncillaryData for CMsgArray<T, N> {}
impl<T: CMsgKind + ?Sized, const N: usize> sealed::Sealed for CMsgArray<T, N> {
    #[inline]
    fn as_ptr(&self) -> *const ffi::c_void {
        self as *const _ as _
    }

    #[inline]
    fn as_mut_ptr(&mut self) -> *mut ffi::c_void {
        self as *mut _ as _
    }

    #[inline]
    fn space(&self) -> usize {
        // same result using `CMSG_SPACE`
        size_of::<Self>()
    }
}

//! Socket control message.
use core::{marker, mem, ops};

use crate::net::raw;

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
    /// `SCM_RIGHTS`
    pub const RIGHTS: Self = Self(raw::SCM_RIGHTS);
    /// `SCM_CREDENTIALS`
    pub const CREDENTIALS: Self = Self(raw::SCM_CREDENTIALS);
    /// `SCM_SECURITY`
    pub const SECURITY: Self = Self(raw::SCM_SECURITY);
    /// `SCM_PIDFD`
    pub const PIDFD: Self = Self(raw::SCM_PIDFD);
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

/// Control message.
#[derive(Debug)]
#[repr(C)]
pub struct CMsgHdr {
    hdr: raw::cmsghdr,
}

impl CMsgHdr {
    /// Returns initialized data size in bytes.
    #[inline]
    pub const fn data_size(&self) -> usize {
        self.hdr.cmsg_len - raw::cmsg_align(size_of::<raw::cmsghdr>())
    }

    /// Returns control message type.
    #[inline]
    pub const fn ty(&self) -> CMsgType {
        CMsgType(self.hdr.cmsg_type)
    }
}

// ===== CMsgArray =====

/// Control message array.
#[derive(Debug)]
#[repr(C)]
pub struct CMsgArray<T: CMsgKind + ?Sized, const N: usize> {
    hdr: raw::cmsghdr,
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
        const { assert!(raw::cmsg_space(size_of::<[T::Data; N]>()) == size_of::<Self>()) };
        let hdr = raw::cmsghdr {
            cmsg_len: raw::cmsg_len(data_len),
            cmsg_level: raw::SOL_SOCKET,
            cmsg_type: T::TYPE.0,
        };
        Self { hdr, data, _kind: marker::PhantomData }
    }

    /// Returns count of the initialized data.
    #[inline]
    pub const fn len(&self) -> usize {
        self.as_hdr().data_size() / size_of::<T::Data>()
    }

    /// Cast to [`CMsgHdr`].
    #[inline]
    pub const fn as_hdr(&self) -> &CMsgHdr {
        unsafe { &*(self as *const _ as *const _) }
    }

    /// Cast to [`CMsgHdr`].
    #[inline]
    pub const fn as_mut_hdr(&mut self) -> &mut CMsgHdr {
        unsafe { &mut *(self as *mut _ as *mut _) }
    }

    /// Returns the initialized data as slice.
    #[inline]
    pub fn data(&self) -> &[T::Data] {
        // SAFETY: the length is set since the constructor or by the kernel
        unsafe { self.data.get_unchecked(..self.len()).assume_init_ref() }
    }

    /// Returns the initialized data as slice.
    #[inline]
    pub fn data_mut(&mut self) -> &mut [T::Data] {
        // SAFETY: the length is set since the constructor or by the kernel
        let len = self.len();
        unsafe { self.data.get_unchecked_mut(..len).assume_init_mut() }
    }
}

impl<T: CMsgKind + ?Sized, const N: usize> ops::Deref for CMsgArray<T, N> {
    type Target = CMsgHdr;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_hdr()
    }
}

impl<T: CMsgKind + ?Sized, const N: usize> ops::DerefMut for CMsgArray<T, N> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut_hdr()
    }
}

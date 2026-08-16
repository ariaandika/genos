//! Socket control message.
use core::{ffi, marker, mem};

use crate::net::msg::{AncillaryData, sealed};

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

impl From<CMsgType> for i32 {
    #[inline]
    fn from(value: CMsgType) -> Self {
        value.0
    }
}

impl CMsgType {
    /// Send or receive a set of open file descriptors from another process.
    pub const RIGHTS: Self = Self(libc::SCM_RIGHTS);
}

// ===== CMsgArray =====

const fn cmsg_space<T, const N: usize>() -> usize {
    unsafe { libc::CMSG_SPACE(size_of::<[T; N]>() as _) as usize }
}

const fn cmsg_len<T, const N: usize>() -> usize {
    unsafe { libc::CMSG_LEN(size_of::<[T; N]>() as _) as usize }
}

/// Control message array.
#[derive(Debug)]
#[repr(C)]
pub struct CMsgArray<T: CMsgKind + ?Sized, const N: usize> {
    hdr: libc::cmsghdr,
    data: [mem::MaybeUninit<T::Data>; N],
    _kind: marker::PhantomData<fn() -> T>,
}

impl<T: CMsgKind + ?Sized, const N: usize> CMsgArray<T, N> {
    /// Creates [`CMsgArray`] with uninitialized data.
    #[inline]
    pub fn uninit() -> Self {
        const { assert!(cmsg_space::<T::Data, N>() == size_of::<Self>()) };
        Self {
            hdr: libc::cmsghdr {
                cmsg_len: cmsg_len::<T::Data, 0>(),
                cmsg_level: libc::SOL_SOCKET,
                cmsg_type: T::TYPE.0,
            },
            data: [const { mem::MaybeUninit::uninit() }; N],
            _kind: marker::PhantomData,
        }
    }

    /// Creates [`CMsgArray`] with given type and data.
    #[inline]
    pub fn new(data: [T::Data; N]) -> Self {
        const { assert!(cmsg_space::<T::Data, N>() == size_of::<Self>()) };
        Self {
            hdr: libc::cmsghdr {
                cmsg_len: cmsg_len::<T::Data, N>(),
                cmsg_level: libc::SOL_SOCKET,
                cmsg_type: T::TYPE.0,
            },
            data: mem::MaybeUninit::new(data).into(),
            _kind: marker::PhantomData,
        }
    }

    /// Returns length of the initialized data.
    #[inline]
    pub fn len(&self) -> usize {
        self.hdr.cmsg_len / size_of::<T::Data>()
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
    fn as_mut_ptr(&mut self) -> *mut ffi::c_void {
        self as *mut _ as _
    }

    #[inline]
    fn space(&self) -> usize {
        size_of::<Self>()
    }
}

// ===== CMsgBuf =====

/// Control message to send or receive a set of open fd from another process.
#[derive(Debug)]
pub struct SCMRights(marker::PhantomData<()>);

impl CMsgKind for SCMRights {
    type Data = i32;

    const TYPE: CMsgType = CMsgType::RIGHTS;
}

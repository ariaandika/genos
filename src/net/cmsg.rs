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

impl CMsgType {
    /// Send or receive a set of open file descriptors from another process.
    pub const RIGHTS: Self = Self(SCM_RIGHTS);
}

impl From<CMsgType> for i32 {
    #[inline]
    fn from(value: CMsgType) -> Self {
        value.0
    }
}

// ===== CMsgArray =====

/// Control message array.
#[derive(Debug)]
#[repr(C)]
pub struct CMsgArray<T: CMsgKind + ?Sized, const N: usize> {
    hdr: cmsghdr,
    data: [mem::MaybeUninit<T::Data>; N],
    _kind: marker::PhantomData<fn() -> T>,
}

impl<T: CMsgKind + ?Sized, const N: usize> CMsgArray<T, N> {
    /// Creates [`CMsgArray`] with uninitialized data.
    #[inline]
    pub fn uninit() -> Self {
        let data = [const { mem::MaybeUninit::uninit() }; N];
        Self::new_inner(data, CMSG_LEN(size_of::<[T::Data; 0]>()))
    }

    /// Creates [`CMsgArray`] with given type and data.
    #[inline]
    pub fn new(data: [T::Data; N]) -> Self {
        let data = mem::MaybeUninit::new(data).into();
        Self::new_inner(data, CMSG_LEN(size_of::<[T::Data; N]>()))
    }

    fn new_inner(data: [mem::MaybeUninit<T::Data>; N], cmsg_len: usize) -> Self {
        const { assert!(CMSG_SPACE(size_of::<[T::Data; N]>()) == size_of::<Self>()) };
        Self {
            hdr: cmsghdr { cmsg_len, cmsg_level: SOL_SOCKET, cmsg_type: T::TYPE.0 },
            data,
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

// ===== extern =====

// source: include/uapi/asm-generic/socket.h
const SOL_SOCKET: i32 = 1;

// source: include/linux/socket.h

/// rw: access rights (array of int)
const SCM_RIGHTS: i32 = 0x01;
// /// rw: struct ucred
// const SCM_CREDENTIALS: i32 = 0x02;
// /// rw: security label
// const SCM_SECURITY: i32 = 0x03;
// /// ro: pidfd (int)
// const SCM_PIDFD: i32 = 0x04;

#[allow(non_snake_case)]
const fn CMSG_ALIGN(len: usize) -> usize {
    (len + size_of::<usize>() - 1) & !(size_of::<usize>() - 1)
}

#[allow(non_snake_case)]
const fn CMSG_SPACE(length: usize) -> usize {
    CMSG_ALIGN(length) + CMSG_ALIGN(size_of::<cmsghdr>())
}

#[allow(non_snake_case)]
const fn CMSG_LEN(length: usize) -> usize {
    CMSG_ALIGN(size_of::<cmsghdr>()) + length
}

// #[allow(non_snake_case)]
// fn CMSG_FIRSTHDR(mhdr: &msghdr) -> *mut cmsghdr {
//     if mhdr.msg_controllen as usize >= size_of::<cmsghdr>() {
//         mhdr.msg_control.cast()
//     } else {
//         0 as _
//     }
// }
//
// #[allow(non_snake_case)]
// fn CMSG_DATA(cmsg: *const cmsghdr) -> *mut u8 {
//     unsafe { cmsg.offset(1) as *mut u8 }
// }

#[derive(Debug)]
#[repr(C)]
struct cmsghdr {
    cmsg_len: usize,
    cmsg_level: i32,
    cmsg_type: i32,
}

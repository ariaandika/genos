//! Socket message.
use core::{ffi, fmt, marker};

use crate::net::iovec::{IoVec, IoVecMut};
use crate::sys;

// ===== AncillaryData =====

/// Ancillary data.
pub trait AncillaryData: sealed::Sealed {}

pub(super) mod sealed {
    pub trait Sealed {
        /// Returns the pointer to the buffer.
        fn as_ptr(&self) -> *const super::ffi::c_void;

        /// Returns the pointer to the buffer.
        fn as_mut_ptr(&mut self) -> *mut super::ffi::c_void;

        /// Returns the buffer length with the padding.
        fn space(&self) -> usize;
    }
}

// ===== MsgHdr =====

/// Message header.
///
/// See `sendmsg(2)`.
#[repr(C)]
pub struct MsgHdr<'io, 'ct> {
    hdr: sys::msghdr,
    _p: marker::PhantomData<&'io [IoVec<'io>]>,
    _q: marker::PhantomData<&'ct ()>,
}

impl<'io> MsgHdr<'io, 'static> {
    /// Creates new [`MsgHdr`].
    #[inline]
    pub const fn new(iov: &'io [IoVec<'io>]) -> Self {
        Self {
            hdr: sys::msghdr {
                msg_name: 0 as _,
                msg_namelen: 0,
                msg_iov: iov.as_ptr().cast_mut().cast(),
                msg_iovlen: iov.len(),
                msg_control: 0 as _,
                msg_controllen: 0,
                msg_flags: 0,
            },
            _p: marker::PhantomData,
            _q: marker::PhantomData,
        }
    }
}

impl<'io, 'ct> MsgHdr<'io, 'ct> {
    /// Creates new [`MsgHdr`].
    #[inline]
    pub fn with_cmsg<C: AncillaryData>(iov: &'io [IoVec<'io>], cmsg: &'ct C) -> Self {
        Self {
            hdr: sys::msghdr {
                msg_name: 0 as _,
                msg_namelen: 0,
                msg_iov: iov.as_ptr().cast_mut().cast(),
                msg_iovlen: iov.len(),
                msg_control: cmsg.as_ptr().cast_mut().cast(),
                msg_controllen: cmsg.space(),
                msg_flags: 0,
            },
            _p: marker::PhantomData,
            _q: marker::PhantomData,
        }
    }
}

impl<'io, 'ct> MsgHdr<'io, 'ct> {
    /// Returns the iovecs length.
    #[inline]
    pub const fn iov_len(&self) -> usize {
        self.hdr.msg_iovlen
    }

    /// Returns the control message length.
    #[inline]
    pub const fn cmsg_len(&self) -> usize {
        self.hdr.msg_controllen
    }
}

impl<'io, 'ct> fmt::Debug for MsgHdr<'io, 'ct> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MsgHdr").finish_non_exhaustive()
    }
}

// ===== MsgHdrMut =====

/// Message header.
///
/// See `recvmsg(2)`.
#[repr(C)]
pub struct MsgHdrMut<'io, 'ct> {
    hdr: sys::msghdr,
    _p: marker::PhantomData<&'io mut [IoVec<'io>]>,
    _q: marker::PhantomData<&'ct mut ()>,
}

impl<'io> MsgHdrMut<'io, 'static> {
    /// Creates new [`MsgHdrMut`].
    #[inline]
    pub const fn new(iov: &'io mut [IoVecMut<'io>]) -> Self {
        Self {
            hdr: sys::msghdr {
                msg_name: 0 as _,
                msg_namelen: 0,
                msg_iov: iov.as_mut_ptr().cast(),
                msg_iovlen: iov.len(),
                msg_control: 0 as _,
                msg_controllen: 0,
                msg_flags: 0,
            },
            _p: marker::PhantomData,
            _q: marker::PhantomData,
        }
    }
}

impl<'io, 'ct> MsgHdrMut<'io, 'ct> {
    /// Creates new [`MsgHdrMut`].
    #[inline]
    pub fn with_cmsg<C: AncillaryData>(iov: &'io mut [IoVecMut<'io>], cmsg: &'ct mut C) -> Self {
        Self {
            hdr: sys::msghdr {
                msg_name: 0 as _,
                msg_namelen: 0,
                msg_iov: iov.as_mut_ptr().cast(),
                msg_iovlen: iov.len(),
                msg_control: cmsg.as_mut_ptr().cast(),
                msg_controllen: cmsg.space(),
                msg_flags: 0,
            },
            _p: marker::PhantomData,
            _q: marker::PhantomData,
        }
    }
}

impl<'io, 'ct> MsgHdrMut<'io, 'ct> {
    /// Returns the iovecs length.
    #[inline]
    pub const fn iov_len(&self) -> usize {
        self.hdr.msg_iovlen
    }

    /// Returns the control message length.
    #[inline]
    pub const fn cmsg_len(&self) -> usize {
        self.hdr.msg_controllen
    }

    /// Returns the message flags.
    #[inline]
    pub const fn flags(&self) -> MsgFlags {
        MsgFlags(self.hdr.msg_flags)
    }
}

impl<'io, 'ct> fmt::Debug for MsgHdrMut<'io, 'ct> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MsgHdrMut").finish_non_exhaustive()
    }
}

// ===== MsgFlags =====

/// [`MsgHdr`] flags.
#[derive(Debug, Clone, Copy)]
pub struct MsgFlags(i32);

macro_rules! def {
    ($($(#[$doc:meta])* fn $f:ident(), $val:ident;)*) => {
        impl MsgFlags {$(
            $(#[$doc])*
            pub const fn $f(&self) -> bool {
                self.0 & sys::$val != 0
            }
        )*}
    };
}
def! {
    /// Indicates end-of-record; the data returned completed a record (generally, used with sockets
    /// of type `SOCK_SEQPACKET`).
    fn has_eor(), MSG_EOR;
    /// Indicates that the trailing portion of a datagram was discarded because the datagram was
    /// larger than the buffer supplied.
    fn has_trunc(), MSG_TRUNC;
    /// Indicates that some control data was discarded due to lack of space in the buffer for
    /// ancillary data.
    fn has_ctrunc(), MSG_CTRUNC;
    /// Is returned to indicate that expedited or out-of-band data was received.
    fn has_oob(), MSG_OOB;
    /// Indicates that no data was received but an extended error from the socket error queue.
    fn has_errqueue(), MSG_ERRQUEUE;
    /// Indicates that `MSG_CMSG_CLOEXEC` was specified in the flags argument of `recvmsg()`.
    fn has_cmsg_cloexec(), MSG_CMSG_CLOEXEC;
}

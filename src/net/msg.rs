//! Socket message.
use core::{ffi, marker};

use crate::net::iovec::IoVecMut;

// ===== AncillaryData =====

/// Ancillary data.
pub trait AncillaryData: sealed::Sealed {}

pub(crate) mod sealed {
    pub trait Sealed {
        fn as_mut_ptr(&mut self) -> *mut super::ffi::c_void;
        fn space(&self) -> usize;
    }
}

// ===== MsgHdr =====

/// Message header.
#[derive(Debug, Clone)]
#[repr(transparent)]
pub struct MsgHdr<'io, 'ct> {
    hdr: libc::msghdr,
    _p: marker::PhantomData<&'io ()>,
    _q: marker::PhantomData<&'ct ()>,
}

impl<'io, 'ct> MsgHdr<'io, 'ct> {
    /// Creates new [`MsgHdr`].
    #[inline]
    pub fn new(iov: &'io mut [IoVecMut<'io>], flags: i32) -> Self {
        Self {
            hdr: libc::msghdr {
                msg_name: 0 as _,
                msg_namelen: 0,
                msg_iov: iov.as_mut_ptr().cast(),
                msg_iovlen: iov.len(),
                msg_control: 0 as _,
                msg_controllen: 0,
                msg_flags: flags,
            },
            _p: marker::PhantomData,
            _q: marker::PhantomData,
        }
    }

    /// Set control message data.
    #[inline]
    pub fn set_control_buf<C: AncillaryData>(&mut self, cmsg: &mut C) {
        self.hdr.msg_control = cmsg.as_mut_ptr();
        self.hdr.msg_controllen = cmsg.space();
    }
}

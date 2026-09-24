//! Socket message.
use core::{fmt, marker, ptr};

use crate::flags;
use crate::io::{IoVec, IoVecMut};
use crate::net::addr::SockAddr;
use crate::net::raw::{self, Socklen};

macro_rules! new_hdr {
    ($n:expr, $nl:expr, $v:expr, $vl:expr, $c:expr, $cl:expr) => {
        Self {
            msg_name: $n,
            msg_namelen: $nl,
            msg_iov: $v,
            msg_iovlen: $vl,
            msg_control: $c,
            msg_controllen: $cl,
            msg_flags: 0,
            _n: marker::PhantomData,
            _o: marker::PhantomData,
            _c: marker::PhantomData,
        }
    };
}

// ===== MsgHdr =====

/// Message header.
///
/// See `sendmsg(2)`.
#[repr(C)]
pub struct MsgHdr<'nm, 'io, 'ct> {
    msg_name: *const SockAddr,
    msg_namelen: Socklen,
    msg_iov: *const IoVec<'io>,
    msg_iovlen: usize,
    msg_control: *const MsgControl,
    msg_controllen: usize,
    msg_flags: i32,
    _n: marker::PhantomData<&'nm Socklen>,
    _o: marker::PhantomData<&'io [IoVec<'io>]>,
    _c: marker::PhantomData<&'ct MsgControl>,
}

impl<'nm, 'io, 'ct> MsgHdr<'nm, 'io, 'ct> {
    /// Creates new [`MsgHdr`].
    #[inline]
    pub const fn new(
        name: &'nm SockAddr,
        namelen: Socklen,
        iov: &'io [IoVec<'io>],
        control: &'ct MsgControl,
        controllen: usize,
    ) -> Self {
        new_hdr!(name, namelen, iov.as_ptr(), iov.len(), control, controllen)
    }
}

impl<'io> MsgHdr<'static, 'io, 'static> {
    /// Creates new [`MsgHdr`] with only iovecs.
    #[inline]
    pub const fn new_iov(iov: &'io [IoVec<'io>]) -> Self {
        new_hdr!(ptr::null_mut(), 0, iov.as_ptr(), iov.len(), ptr::null_mut(), 0)
    }
}

impl<'io, 'ct> MsgHdr<'static, 'io, 'ct> {
    /// Creates new [`MsgHdr`] with iovecs as message control.
    #[inline]
    pub const fn new_cmsg(
        iov: &'io [IoVec<'io>],
        control: &'ct MsgControl,
        controllen: usize,
    ) -> Self {
        new_hdr!(ptr::null_mut(), 0, iov.as_ptr(), iov.len(), control, controllen)
    }
}

impl MsgHdr<'_, '_, '_> {
    /// Returns the message iovecs count.
    #[inline]
    pub const fn iov_len(&self) -> usize {
        self.msg_iovlen
    }

    /// Returns the message control length.
    #[inline]
    pub const fn control_len(&self) -> usize {
        self.msg_controllen
    }
}

impl fmt::Debug for MsgHdr<'_, '_, '_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MsgHdr").finish_non_exhaustive()
    }
}

// ===== MsgHdrMut =====

/// Message header.
///
/// See `recvmsg(2)`.
#[repr(C)]
pub struct MsgHdrMut<'nm, 'io, 'ct> {
    msg_name: *mut SockAddr,
    msg_namelen: Socklen,
    msg_iov: *mut IoVecMut<'io>,
    msg_iovlen: usize,
    msg_control: *mut MsgControl,
    msg_controllen: usize,
    msg_flags: i32,
    _n: marker::PhantomData<&'nm mut Socklen>,
    _o: marker::PhantomData<&'io mut [IoVecMut<'io>]>,
    _c: marker::PhantomData<&'ct mut MsgControl>,
}

impl<'nm, 'io, 'ct> MsgHdrMut<'nm, 'io, 'ct> {
    /// Creates new [`MsgHdrMut`].
    #[inline]
    pub const fn new(
        name: &'nm mut SockAddr,
        namelen: Socklen,
        iov: &'io mut [IoVecMut<'io>],
        control: &'ct mut MsgControl,
        controllen: usize,
    ) -> Self {
        new_hdr!(name, namelen, iov.as_mut_ptr(), iov.len(), control, controllen)
    }
}

impl<'io> MsgHdrMut<'static, 'io, 'static> {
    /// Creates new [`MsgHdrMut`] with only iovecs.
    #[inline]
    pub const fn new_iov(iov: &'io mut [IoVecMut<'io>]) -> Self {
        new_hdr!(ptr::null_mut(), 0, iov.as_mut_ptr(), iov.len(), ptr::null_mut(), 0)
    }
}

impl<'io, 'ct> MsgHdrMut<'static, 'io, 'ct> {
    /// Creates new [`MsgHdrMut`] with iovecs as message control.
    #[inline]
    pub const fn new_cmsg(
        iov: &'io mut [IoVecMut<'io>],
        control: &'ct mut MsgControl,
        controllen: usize,
    ) -> Self {
        new_hdr!(ptr::null_mut(), 0, iov.as_mut_ptr(), iov.len(), control, controllen)
    }
}

impl MsgHdrMut<'_, '_, '_> {
    /// Returns the message iovecs count.
    #[inline]
    pub const fn iov_len(&self) -> usize {
        self.msg_iovlen
    }

    /// Returns the message control length.
    #[inline]
    pub const fn cmsg_len(&self) -> usize {
        self.msg_controllen
    }

    /// Returns the message flags.
    #[inline]
    pub const fn flags(&self) -> MsgFlags {
        MsgFlags(self.msg_flags)
    }
}

impl fmt::Debug for MsgHdrMut<'_, '_, '_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MsgHdrMut").finish_non_exhaustive()
    }
}

// ===== Control =====

/// Control message.
#[derive(Debug)]
pub struct MsgControl {
    _p: [u8; 0],
}

// ===== MsgFlags =====

/// [`MsgHdr`] flags.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MsgFlags(i32);

flags::impl_bitops_simple!(MsgFlags);

impl MsgFlags {
    /// `MSG_EOR`
    pub const EOR: MsgFlags = MsgFlags(raw::MSG_EOR);
    /// `MSG_TRUNC`
    pub const TRUNC: MsgFlags = MsgFlags(raw::MSG_TRUNC);
    /// `MSG_CTRUNC`
    pub const CTRUNC: MsgFlags = MsgFlags(raw::MSG_CTRUNC);
    /// `MSG_OOB`
    pub const OOB: MsgFlags = MsgFlags(raw::MSG_OOB);
    /// `MSG_ERRQUEUE`
    pub const ERRQUEUE: MsgFlags = MsgFlags(raw::MSG_ERRQUEUE);
    /// `MSG_CMSG_CLOEXEC`
    pub const CMSG_CLOEXEC: MsgFlags = MsgFlags(raw::MSG_CMSG_CLOEXEC);
}

impl From<MsgFlags> for i32 {
    #[inline]
    fn from(value: MsgFlags) -> Self {
        value.0
    }
}

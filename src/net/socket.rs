//! [`Socket`] associated types.
use core::mem::MaybeUninit;

use crate::fd::{AsFd, OwnedFd};
use crate::net::addr::{Family, SockAddr};
use crate::net::msg::{MsgHdr, MsgHdrMut};
use crate::net::raw::{self, Socklen};
use crate::sys::SysRes;
use crate::{fd, flags, sys};

// ===== Socket =====

/// Communication endpoint.
#[derive(Debug)]
pub struct Socket(OwnedFd);

fd::impl_fd_simple!(Socket);

impl Socket {
    /// Creates new [`Socket`] (`socket(2)`).
    #[inline]
    pub fn create(domain: Family, ty: Type, flags: Flags) -> impl SysRes<Self> {
        sys::call_rd!(sys_socket, i32::from(domain), ty.0 | flags.0, 0)
    }

    /// Creates new UNIX domain [`Socket`] stream.
    ///
    /// This is a helper method to create socket with [`Family::UNIX`] and [`Socket::STREAM`].
    #[inline]
    pub fn unix_stream(flags: Flags) -> impl SysRes<Self> {
        Self::create(Family::UNIX, Self::STREAM, flags)
    }

    /// Bind address to this socket (`bind(2)`).
    #[inline]
    pub fn bind(&self, addr: &SockAddr, addrlen: Socklen) -> impl SysRes<()> {
        sys::call_rd!(sys_bind, self.as_raw_fd(), addr, addrlen)
    }

    /// Initiate a connection on this socket (`connect(2)`).
    #[inline]
    pub fn connect(&self, addr: &SockAddr, addrlen: Socklen) -> impl SysRes<()> {
        sys::call_rd!(sys_connect, self.as_raw_fd(), addr, addrlen)
    }

    /// Returns this socket address (`getsockname(2)`).
    #[inline]
    pub fn addr(&self, addr: &mut SockAddr, addrlen: &mut Socklen) -> impl SysRes<()> {
        sys::call!(sys_getsockname, self.as_raw_fd(), addr, addrlen)
    }

    /// Returns the peer socket address (`getpeername(2)`).
    #[inline]
    pub fn peer_addr(&self, addr: &mut SockAddr, addrlen: &mut Socklen) -> impl SysRes<()> {
        sys::call!(sys_getpeername, self.as_raw_fd(), addr, addrlen)
    }

    /// Listen for connections on this socket (`listen(2)`).
    #[inline]
    pub fn listen(&self, backlog: i32) -> impl SysRes<()> {
        sys::call_rd!(sys_listen, self.as_raw_fd(), backlog)
    }

    /// Shut down part of a full-duplex connection (`shutdown(2)`).
    #[inline]
    pub fn shutdown(&self, how: Shutdown) -> impl SysRes<()> {
        sys::call_rd!(sys_shutdown, self.as_raw_fd(), how.0)
    }
}

impl Socket {
    /// Send message on this fd (`sendto(2)`).
    #[inline]
    pub fn send(&self, buf: &[u8], flags: MsgFlags) -> impl SysRes<usize> {
        sys::call_rd!(sys_sendto, self.as_raw_fd(), buf.as_ptr(), buf.len(), flags.0, 0, 0)
    }

    /// Send message on this fd (`sendmsg(2)`).
    #[inline]
    pub fn sendmsg(&self, msg: &MsgHdr, flags: MsgFlags) -> impl SysRes<usize> {
        sys::call_rd!(sys_sendmsg, self.as_raw_fd(), msg, flags.0)
    }

    /// Receive message from this fd (`recvfrom(2)`).
    #[inline]
    pub fn recv(&self, buf: &mut [MaybeUninit<u8>], flags: MsgFlags) -> impl SysRes<usize> {
        sys::call!(sys_recvfrom, self.as_raw_fd(), buf.as_mut_ptr(), buf.len(), flags.0, 0, 0)
    }

    /// Receive message from this fd (`recvmsg(2)`).
    #[inline]
    pub fn recvmsg(&self, msg: &mut MsgHdrMut, flags: MsgFlags) -> impl SysRes<usize> {
        sys::call!(sys_recvmsg, self.as_raw_fd(), msg, flags.0)
    }

    /// Accept a connection on this socket (`accept4(2)`).
    #[inline]
    pub fn accept(
        &self,
        addr: Option<&mut SockAddr>,
        addrlen: Option<&mut Socklen>,
        flags: Flags,
    ) -> impl SysRes<Self> {
        let addr = sys::optmut(addr);
        let addrlen = sys::optmut(addrlen);
        sys::call_rd!(sys_accept4, self.as_raw_fd(), addr, addrlen, flags.0)
    }
}

// ===== Type =====

/// Socket type.
#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct Type(i32);

impl Socket {
    /// `SOCK_STREAM`
    pub const STREAM: Type = Type(raw::SOCK_STREAM);
    /// `SOCK_DGRAM`
    pub const DGRAM: Type = Type(raw::SOCK_DGRAM);
    /// `SOCK_RAW`
    pub const RAW: Type = Type(raw::SOCK_RAW);
}

// ===== Flags =====

/// Socket creation flags.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Flags(i32);

flags::impl_bitops_simple!(Flags);

impl Socket {
    /// `SOCK_CLOEXEC`
    pub const CLOEXEC: Flags = Flags(raw::SOCK_CLOEXEC);
    /// `SOCK_NONBLOCK`
    pub const NONBLOCK: Flags = Flags(raw::SOCK_NONBLOCK);
}

// ===== MsgFlags =====

/// Message operation flags.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct MsgFlags(i32);

flags::impl_bitops_simple!(MsgFlags);

impl MsgFlags {
    /// `MSG_CMSG_CLOEXEC`
    pub const CMSG_CLOEXEC: Self = Self(raw::MSG_CMSG_CLOEXEC);
    /// `MSG_DONTWAIT`
    pub const DONTWAIT: Self = Self(raw::MSG_DONTWAIT);
    /// `MSG_PEEK`
    pub const PEEK: Self = Self(raw::MSG_PEEK);
}

// ===== ShutdownFlags =====

/// Shutdown types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Shutdown(i32);

impl Shutdown {
    /// `SHUT_RD`
    pub const READ: Self = Self(raw::SHUT_RD);
    /// `SHUT_WR`
    pub const WRITE: Self = Self(raw::SHUT_WR);
    /// `SHUT_RDWR`
    pub const BOTH: Self = Self(raw::SHUT_RDWR);
}

//! [`Socket`] associated types.
use core::mem::MaybeUninit;
use core::{fmt, result};

use crate::error::{ErrCode, SysResExt};
use crate::fd::{AsFd, OwnedFd};
use crate::net::addr::{AddrError, Family, SockAddr};
use crate::net::msg::{MsgHdr, MsgHdrMut};
use crate::net::raw::{self, Socklen};
use crate::{error, fd, flags, sys};

// ===== Socket =====

/// Communication endpoint.
#[derive(Debug)]
pub struct Socket(OwnedFd);

fd::impl_fd_simple!(Socket);

impl Socket {
    /// Creates new [`Socket`] (`socket(2)`).
    #[inline]
    pub fn create(domain: Family, ty: Type, flags: Flags) -> Result<Self> {
        sys::call_rd!(sys_socket, i32::from(domain), ty.0 | flags.0, 0).fd(Kind::Create)
    }

    /// Creates new UNIX domain [`Socket`] stream.
    ///
    /// This is a helper method to create socket with [`Family::UNIX`] and [`Type::STREAM`].
    #[inline]
    pub fn unix_stream(flags: Flags) -> Result<Self> {
        Self::create(Family::UNIX, Type::STREAM, flags)
    }

    /// Bind address to this socket (`bind(2)`).
    #[inline]
    pub fn bind(&self, addr: &SockAddr, addrlen: Socklen) -> Result<()> {
        sys::call_rd!(sys_bind, self.as_raw_fd(), addr, addrlen).e(Kind::Bind)
    }

    /// Initiate a connection on this socket (`connect(2)`).
    #[inline]
    pub fn connect(&self, addr: &SockAddr, addrlen: Socklen) -> Result<()> {
        sys::call_rd!(sys_connect, self.as_raw_fd(), addr, addrlen).e(Kind::Connect)
    }

    /// Returns this socket address (`getsockname(2)`).
    #[inline]
    pub fn addr(&self, addr: &mut SockAddr, addrlen: &mut Socklen) -> Result<()> {
        sys::call!(sys_getsockname, self.as_raw_fd(), addr, addrlen).e(Kind::GetAddr)
    }

    /// Returns the peer socket address (`getpeername(2)`).
    #[inline]
    pub fn peer_addr(&self, addr: &mut SockAddr, addrlen: &mut Socklen) -> Result<()> {
        sys::call!(sys_getpeername, self.as_raw_fd(), addr, addrlen).e(Kind::GetAddr)
    }

    /// Listen for connections on this socket (`listen(2)`).
    #[inline]
    pub fn listen(&self, backlog: i32) -> Result<()> {
        sys::call_rd!(sys_listen, self.as_raw_fd(), backlog).e(Kind::Listen)
    }

    /// Shut down part of a full-duplex connection (`shutdown(2)`).
    #[inline]
    pub fn shutdown(&self, how: Shutdown) -> Result<()> {
        sys::call_rd!(sys_shutdown, self.as_raw_fd(), how.0).e(Kind::Shutdown)
    }
}

impl Socket {
    /// Send message on this fd (`sendto(2)`).
    #[inline]
    pub fn send(&self, buf: &[u8], flags: SendFlags) -> Result<usize> {
        sys::call_rd!(sys_sendto, self.as_raw_fd(), buf.as_ptr(), buf.len(), flags.0, 0, 0)
            .io(Kind::Write)
    }

    /// Send message on this fd (`sendmsg(2)`).
    #[inline]
    pub fn sendmsg(&self, msg: &MsgHdr, flags: SendFlags) -> Result<usize> {
        sys::call_rd!(sys_sendmsg, self.as_raw_fd(), msg, flags.0).io(Kind::Write)
    }

    /// Receive message from this fd (`recvfrom(2)`).
    #[inline]
    pub fn recv(&self, buf: &mut [MaybeUninit<u8>], flags: RecvFlags) -> Result<usize> {
        sys::call!(sys_recvfrom, self.as_raw_fd(), buf.as_mut_ptr(), buf.len(), flags.0, 0, 0)
            .io(Kind::Read)
    }

    /// Receive message from this fd (`recvmsg(2)`).
    #[inline]
    pub fn recvmsg(&self, msg: &mut MsgHdrMut, flags: RecvFlags) -> Result<usize> {
        sys::call!(sys_recvmsg, self.as_raw_fd(), msg, flags.0).io(Kind::Read)
    }

    /// Accept a connection on this socket (`accept4(2)`).
    #[inline]
    pub fn accept(&self, flags: Flags) -> Result<Self> {
        sys::call_rd!(sys_accept4, self.as_raw_fd(), 0, 0, flags.0).fd(Kind::Accept)
    }
}

// ===== Type =====

/// Socket type.
#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct Type(i32);

impl Type {
    /// `SOCK_STREAM`
    pub const STREAM: Self = Self(raw::SOCK_STREAM);
    /// `SOCK_DGRAM`
    pub const DGRAM: Self = Self(raw::SOCK_DGRAM);
    /// `SOCK_RAW`
    pub const RAW: Self = Self(raw::SOCK_RAW);
}

// ===== Flags =====

/// Socket creation flags.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Flags(i32);

flags::impl_bitops_simple!(Flags);

impl Flags {
    /// `SOCK_CLOEXEC`
    pub const CLOEXEC: Self = Self(raw::SOCK_CLOEXEC);
    /// `SOCK_NONBLOCK`
    pub const NONBLOCK: Self = Self(raw::SOCK_NONBLOCK);
}

// ===== SendFlags =====

/// Message sending operation flags.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct SendFlags(i32);

flags::impl_bitops_simple!(SendFlags);

impl SendFlags {
    /// `MSG_DONTWAIT`
    pub const DONTWAIT: Self = Self(raw::MSG_DONTWAIT);
}

// ===== RecvFlags =====

/// Message receiving operation flags.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct RecvFlags(i32);

flags::impl_bitops_simple!(RecvFlags);

impl RecvFlags {
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

// ===== Error =====

/// Type alias for result of [`Socket`] operations.
pub type Result<T, E = Error> = result::Result<T, E>;

/// An error that may occur during any [`Socket`] operations.
#[derive(Debug, Clone)]
pub struct Error {
    kind: Kind,
    code: ErrCode,
}

#[derive(Debug, Clone)]
enum Kind {
    Create,
    Bind,
    Connect,
    Listen,
    Read,
    Write,
    Accept,
    GetAddr,
    Shutdown,
    Addr(AddrError),
}

error::impl_error_with_kind!(Error, Kind);

impl From<AddrError> for Error {
    #[inline]
    fn from(v: AddrError) -> Self {
        Self { kind: Kind::Addr(v), code: ErrCode::EINVAL }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self { kind, code } = self;
        let cause = match kind {
            Kind::Addr(err) => format_args!("{}", { err }),
            _ => format_args!("{code}"),
        };
        let msg = match kind {
            Kind::Create => "create socket",
            Kind::Bind => "bind socket address",
            Kind::Connect => "connect peer socket",
            Kind::Listen => "listen socket connection",
            Kind::Read => "read socket",
            Kind::Write => "write socket",
            Kind::Accept => "accept socket connection",
            Kind::GetAddr => "get socket address",
            Kind::Shutdown => "shutdown socket",
            Kind::Addr(_) => "create socket address",
        };
        write!(f, "failed to {msg}: {cause}")
    }
}

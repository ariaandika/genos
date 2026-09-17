//! [`Socket`] associated types.
use core::mem::MaybeUninit;
use core::{fmt, result};

use crate::error::{ErrCode, SysResExt};
use crate::fd::{AsFd, OwnedFd};
use crate::net::addr::{AddrError, Family, SockAddr};
use crate::net::msg::{MsgHdr, MsgHdrMut};
use crate::{error, fd, flags, sys};

// ===== Socket =====

/// Communication endpoint.
#[derive(Debug)]
pub struct Socket(OwnedFd);

fd::impl_fd_simple!(Socket);

impl Socket {
    /// Creates new [`Socket`].
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

    /// Bind address to this socket.
    #[inline]
    pub fn bind<A: SockAddr>(&self, addr: &A) -> Result<()> {
        let len = size_of::<A>() as sys::socklen_t;
        sys::call_rd!(sys_bind, self.as_raw_fd(), addr, len).e(Kind::Bind)
    }

    /// Initiate a connection on this socket.
    #[inline]
    pub fn connect<A: SockAddr>(&self, addr: &A) -> Result<()> {
        let len = size_of::<A>() as sys::socklen_t;
        sys::call_rd!(sys_connect, self.as_raw_fd(), addr, len).e(Kind::Connect)
    }

    /// Returns this socket address.
    #[inline]
    pub fn addr<A: SockAddr>(&self) -> Result<A> {
        let (mut addr, mut len) = (A::zeroed(), A::socklen_t());
        sys::call!(sys_getsockname, self.as_raw_fd(), &mut addr, &mut len).e(Kind::GetAddr)?;
        Self::validate_addr(addr, len)
    }

    /// Returns the peer socket address.
    #[inline]
    pub fn peer_addr<A: SockAddr>(&self) -> Result<A> {
        let (mut addr, mut len) = (A::zeroed(), A::socklen_t());
        sys::call!(sys_getpeername, self.as_raw_fd(), &mut addr, &mut len).e(Kind::GetAddr)?;
        Self::validate_addr(addr, len)
    }

    #[inline]
    fn validate_addr<A: SockAddr>(addr: A, len: sys::socklen_t) -> Result<A> {
        if addr.sa_family() != A::FAMILY.sa_family() {
            return Err(AddrError::MissmatchFamily.into());
        }
        if len > A::socklen_t() {
            return Err(AddrError::MissmatchFamily.into());
        }
        Ok(addr)
    }

    /// Listen for connections on this socket.
    #[inline]
    pub fn listen(&self) -> Result<()> {
        sys::call_rd!(sys_listen, self.as_raw_fd(), -1).e(Kind::Listen)
    }

    /// Shut down part of a full-duplex connection.
    #[inline]
    pub fn shutdown(&self) -> Result<()> {
        sys::call_rd!(sys_shutdown, self.as_raw_fd(), -1).e(Kind::Shutdown)
    }
}

impl Socket {
    /// Send message on this fd.
    #[inline]
    pub fn send(&self, buf: &[u8], flags: SendFlags) -> Result<usize> {
        sys::call_rd!(sys_sendto, self.as_raw_fd(), buf.as_ptr(), buf.len(), flags.0, 0, 0)
            .io(Kind::Write)
    }

    /// Send message on this fd.
    #[inline]
    pub fn sendmsg(&self, msg: &MsgHdr, flags: SendFlags) -> Result<usize> {
        sys::call_rd!(sys_sendmsg, self.as_raw_fd(), msg, flags.0).io(Kind::Write)
    }

    /// Receive message from this fd.
    #[inline]
    pub fn recv(&self, buf: &mut [MaybeUninit<u8>], flags: RecvFlags) -> Result<usize> {
        sys::call!(sys_recvfrom, self.as_raw_fd(), buf.as_mut_ptr(), buf.len(), flags.0, 0, 0)
            .io(Kind::Read)
    }

    /// Receive message from this fd.
    #[inline]
    pub fn recvmsg(&self, msg: &mut MsgHdrMut, flags: RecvFlags) -> Result<usize> {
        sys::call!(sys_recvmsg, self.as_raw_fd(), msg, flags.0).io(Kind::Read)
    }

    /// Accept a connection on this socket.
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
    /// Provides sequenced, reliable, two-way, connection-based byte streams.
    pub const STREAM: Self = Self(sys::SOCK_STREAM);
    /// Supports datagrams (connectionless, unreliable messages of a fixed maximum length).
    pub const DGRAM: Self = Self(sys::SOCK_DGRAM);
    /// Provides raw network protocol access.
    pub const RAW: Self = Self(sys::SOCK_RAW);
}

// ===== Flags =====

/// Socket creation flags.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Flags(i32);

impl Flags {
    /// Set the close-on-exec (FD_CLOEXEC) flag on the new fd.
    pub const CLOEXEC: Self = Self(sys::SOCK_CLOEXEC);
    /// Set the `O_NONBLOCK` file status flag on the new fd.
    pub const NONBLOCK: Self = Self(sys::SOCK_NONBLOCK);
}

impl flags::OpenFlag for Flags {
    const CLOEXEC: Self = Self::CLOEXEC;
    const NONBLOCK: Self = Self::NONBLOCK;
}

flags::impl_bitops_simple!(Flags);

// ===== SendFlags =====

/// Message sending operation flags.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct SendFlags(i32);

impl SendFlags {
    /// Enables nonblocking operation; if the operation would block, the call fails with EAGAIN or
    /// EWOULDBLOCK.
    pub const DONTWAIT: Self = Self(sys::MSG_DONTWAIT);
}

flags::impl_bitops_simple!(SendFlags);

// ===== RecvFlags =====

/// Message receiving operation flags.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct RecvFlags(i32);

impl RecvFlags {
    /// Set the close-on-exec flag for the fd received via a UNIX domain fd using the `SCM_RIGHTS`
    /// operation.
    pub const CMSG_CLOEXEC: Self = Self(sys::MSG_CMSG_CLOEXEC);
    /// Enables nonblocking operation; if the operation would block, the call fails with EAGAIN or
    /// EWOULDBLOCK.
    pub const DONTWAIT: Self = Self(sys::MSG_DONTWAIT);
    /// Receive message without removing that data from the queue.
    pub const PEEK: Self = Self(sys::MSG_PEEK);
}

flags::impl_bitops_simple!(RecvFlags);

// ===== ShutdownFlags =====

/// Shutdown types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Shutdown(i32);

impl Shutdown {
    /// Disable further receptions.
    pub const READ: Self = Self(sys::SHUT_RD);
    /// Disable further transmission.
    pub const WRITE: Self = Self(sys::SHUT_WR);
    /// Disable further receptions and transmission.
    pub const BOTH: Self = Self(sys::SHUT_RDWR);
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

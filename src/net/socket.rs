//! Linux socket.
use core::mem::MaybeUninit;
use core::{error, fmt, mem, result};

use crate::error::{AsErrCode, ErrCode};
use crate::fd::{AsFd, FromRawFd, OwnedFd, impl_fd_simple};
use crate::flags::{OpenFlag, impl_bitops_simple};
use crate::io::{Read, ReadError, Write, WriteError};
use crate::net::addr::{AddrError, Family, SockAddr};
use crate::net::msg::MsgHdr;
use crate::sys;

// ===== Socket =====

/// Linux socket.
#[derive(Debug)]
pub struct Socket(OwnedFd);

impl_fd_simple!(Socket);

impl Socket {
    /// Creates new [`Socket`].
    #[inline]
    pub fn create(family: Family, ty: Type, flags: Flags) -> Result<Self> {
        fd(sys::call!(RD, __NR_socket, i32::from(family), ty.0 | flags.0, 0), Kind::Create)
    }

    /// Creates new UNIX domain [`Socket`] stream.
    #[inline]
    pub fn unix_stream(flags: Flags) -> Result<Self> {
        Self::create(Family::UNIX, Type::STREAM, flags)
    }

    /// Bind address to this socket.
    #[inline]
    pub fn bind<A: SockAddr>(&self, addr: &A) -> Result<()> {
        let (raw, len) = addr.as_raw();
        e(sys::call!(RD, __NR_bind, self.as_fd(), raw, len), Kind::Bind)
    }

    /// Initiate a connection on this socket.
    #[inline]
    pub fn connect<A: SockAddr>(&self, addr: &A) -> Result<()> {
        let (raw, len) = addr.as_raw();
        e(sys::call!(RD, __NR_connect, self.as_fd(), raw, len), Kind::Connect)
    }

    /// Returns this socket address.
    #[inline]
    pub fn addr<A: SockAddr>(&self) -> Result<A> {
        let mut addr = unsafe { mem::zeroed::<A::Raw>() };
        let mut len = size_of::<A::Raw>() as _;
        let res = sys::call!(__NR_getsockname, self.as_fd(), &mut addr, &mut len);
        match e(res, Kind::GetAddr) {
            Ok(()) => A::from_raw(addr, len).map_err(<_>::into),
            Err(err) => Err(err),
        }
    }

    /// Returns the peer socket address.
    #[inline]
    pub fn peer_addr<A: SockAddr>(&self) -> Result<A> {
        let mut addr = unsafe { mem::zeroed::<A::Raw>() };
        let mut len = size_of::<A::Raw>() as _;
        let res = sys::call!(__NR_getpeername, self.as_fd(), &mut addr, &mut len);
        match e(res, Kind::GetAddr) {
            Ok(()) => A::from_raw(addr, len).map_err(<_>::into),
            Err(err) => Err(err),
        }
    }

    /// Listen for connections on this socket.
    #[inline]
    pub fn listen(&self) -> Result<()> {
        e(sys::call!(RD, __NR_listen, self.as_fd(), -1), Kind::Listen)
    }
}

impl Read for Socket {
    type Error = Error;
}

impl Write for Socket {
    type Error = Error;
}

// would block
impl Socket {
    /// Send message on this fd.
    #[inline]
    pub fn send(&self, buf: &[u8], flags: SendFlags) -> Result<usize> {
        io(sys::call!(RD, __NR_sendto, self.as_fd(), buf, buf.len(), flags.0, 0, 0), Kind::Write)
    }

    /// Send message on this fd.
    #[inline]
    pub fn sendmsg(&self, msg: &MsgHdr, flags: SendFlags) -> Result<usize> {
        io(sys::call!(RD, __NR_sendmsg, self.as_fd(), msg, flags.0), Kind::Write)
    }

    /// Receive message from this fd.
    #[inline]
    pub fn recv(&self, buf: &mut [MaybeUninit<u8>], flags: RecvFlags) -> Result<usize> {
        io(sys::call!(__NR_recvfrom, self.as_fd(), &mut *buf, buf.len(), flags.0, 0, 0), Kind::Read)
    }

    /// Receive message from this fd.
    #[inline]
    pub fn recvmsg(&self, msg: &mut MsgHdr, flags: RecvFlags) -> Result<usize> {
        io(sys::call!(__NR_recvmsg, self.as_fd(), msg, flags.0), Kind::Read)
    }

    /// Accept a connection on this socket.
    #[inline]
    pub fn accept(&self, flags: Flags) -> Result<Self> {
        fd(sys::call!(RD, __NR_accept4, self.as_fd(), 0, 0, flags.0), Kind::Accept)
    }
}

// ===== Type =====

/// Socket type.
#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct Type(i32);

impl Type {
    /// Provides sequenced, reliable, two-way, connection-based byte streams.
    ///
    /// An out-of-band data transmission mechanism may be supported.
    pub const STREAM: Self = Self(SOCK_STREAM);
    /// Supports datagrams (connectionless, unreliable messages of a fixed maximum length).
    pub const DGRAM: Self = Self(SOCK_DGRAM);
    /// Provides raw network protocol access.
    pub const RAW: Self = Self(SOCK_RAW);
}

// ===== Flags =====

/// Socket creation flags.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Flags(i32);

impl Flags {
    /// Set the close-on-exec (FD_CLOEXEC) flag on the new fd.
    pub const CLOEXEC: Self = Self(SOCK_CLOEXEC);
    /// Set the `O_NONBLOCK` file status flag on the new fd.
    pub const NONBLOCK: Self = Self(SOCK_NONBLOCK);
}

impl OpenFlag for Flags {
    const CLOEXEC: Self = Self::CLOEXEC;
    const NONBLOCK: Self = Self::NONBLOCK;
}

impl_bitops_simple!(Flags);

// ===== SendFlags =====

/// Message sending operation flags.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct SendFlags(i32);

impl SendFlags {
    /// Enables nonblocking operation; if the operation would block, the call fails with EAGAIN or
    /// EWOULDBLOCK.
    pub const DONTWAIT: Self = Self(MSG_DONTWAIT);
}

impl_bitops_simple!(SendFlags);

// ===== RecvFlags =====

/// Message receiving operation flags.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct RecvFlags(i32);

impl RecvFlags {
    /// Set the close-on-exec flag for the fd received via a UNIX domain fd using the `SCM_RIGHTS`
    /// operation.
    pub const CMSG_CLOEXEC: Self = Self(MSG_CMSG_CLOEXEC);
    /// Enables nonblocking operation; if the operation would block, the call fails with EAGAIN or
    /// EWOULDBLOCK.
    pub const DONTWAIT: Self = Self(MSG_DONTWAIT);
    /// Receive message without removing that data from the queue.
    pub const PEEK: Self = Self(MSG_PEEK);
}

impl_bitops_simple!(RecvFlags);

// ===== Error =====

fn fd<T: FromRawFd>(res: isize, kind: Kind) -> Result<T> {
    if res.is_negative() {
        return Err(Error::new(kind, res.wrapping_neg()));
    }
    Ok(unsafe { T::from_raw_fd(res as _) })
}

fn e(res: isize, kind: Kind) -> Result<()> {
    if res.is_negative() {
        return Err(Error::new(kind, res.wrapping_neg()));
    }
    Ok(())
}

fn io(res: isize, kind: Kind) -> Result<usize> {
    match usize::try_from(res) {
        Ok(ok) => Ok(ok),
        Err(_) => Err(Error::new(kind, res.wrapping_neg())),
    }
}

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
    Addr(AddrError),
}

impl Error {
    fn new(kind: Kind, code: isize) -> Self {
        Self { kind, code: ErrCode::new(code as _) }
    }
}

impl From<AddrError> for Error {
    #[inline]
    fn from(v: AddrError) -> Self {
        Self { kind: Kind::Addr(v), code: ErrCode::new(libc::EINVAL) }
    }
}

impl From<ReadError> for Error {
    #[inline]
    fn from(v: ReadError) -> Self {
        Self { kind: Kind::Read, code: v.into() }
    }
}

impl From<WriteError> for Error {
    #[inline]
    fn from(v: WriteError) -> Self {
        Self { kind: Kind::Write, code: v.into() }
    }
}

impl AsErrCode for Error {
    #[inline]
    fn as_err_code(&self) -> ErrCode {
        self.code
    }
}

impl error::Error for Error {}

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
            Kind::Addr(_) => "create socket address",
        };
        write!(f, "failed to {msg}: {cause}")
    }
}

// ===== extern =====

// source: include/linux/socket.h

// #define MSG_OOB		1
const MSG_PEEK: i32 = 2;
// #define MSG_DONTROUTE	4
// #define MSG_TRYHARD     4
// #define MSG_CTRUNC	8
// #define MSG_PROBE	0
// #define MSG_TRUNC	0x20
const MSG_DONTWAIT: i32 = 0x40;
// #define MSG_EOR         0x80
// #define MSG_WAITALL	0x100
// #define MSG_FIN         0x200
// #define MSG_SYN		0x400
// #define MSG_CONFIRM	0x800
// #define MSG_RST		0x1000
// #define MSG_ERRQUEUE	0x2000
// #define MSG_NOSIGNAL	0x4000
// #define MSG_MORE	0x8000
// #define MSG_WAITFORONE	0x10000
// #define MSG_SENDPAGE_NOPOLICY 0x10000
// #define MSG_BATCH	0x40000
// #define MSG_EOF         MSG_FIN
// #define MSG_NO_SHARED_FRAGS 0x80000
// #define MSG_SENDPAGE_DECRYPTED	0x100000
// #define MSG_SOCK_DEVMEM 0x2000000
// #define MSG_ZEROCOPY	0x4000000
// #define MSG_SPLICE_PAGES 0x8000000
// #define MSG_FASTOPEN	0x20000000
const MSG_CMSG_CLOEXEC: i32 = 0x40000000;

// source: include/linux/net.h

const SOCK_STREAM: i32 = 1;
const SOCK_DGRAM: i32 = 2;
const SOCK_RAW: i32 = 3;
// const SOCK_RDM: i32 = 4;
// const SOCK_SEQPACKET: i32 = 5;
// const SOCK_DCCP: i32 = 6;
// const SOCK_PACKET: i32 = 10;

const SOCK_CLOEXEC: i32 = sys::O_CLOEXEC;
const SOCK_NONBLOCK: i32 = sys::O_NONBLOCK;

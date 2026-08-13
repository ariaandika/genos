//! Linux socket.
use core::ffi::{CStr, c_char};
use core::{error, fmt, mem};
use std::task::Poll;

use crate::error::{ErrCode, FromErrCode, os_error_simple};
use crate::fd::{AsRawFd, OwnedFd, impl_fd_simple};
use crate::flags::{OpenFlag, impl_bitops_simple};

// ===== Socket =====

/// Linux socket.
#[derive(Debug)]
pub struct Socket(OwnedFd);

impl_fd_simple!(Socket);

impl Socket {
    /// Creates new [`Socket`].
    #[inline]
    pub fn create(domain: Domain, ty: Type, flags: Flags) -> Result<Self, CreateError> {
        unsafe { <_>::fd(libc::socket(domain.0, ty.0 | flags.0, 0)) }
    }

    /// Creates new UNIX domain [`Socket`] stream.
    #[inline]
    pub fn unix_stream(flags: Flags) -> Result<Self, CreateError> {
        Self::create(Domain::UNIX, Type::STREAM, flags)
    }

    /// Bind address to this socket.
    #[inline]
    pub fn bind<A: SockAddr>(&self, addr: &A) -> Result<(), BindError> {
        let (raw, len) = addr.as_raw();
        <_>::e(unsafe { libc::bind(self.as_raw_fd(), raw as *const _ as _, len) })
    }

    /// Initiate a connection on this socket.
    #[inline]
    pub fn connect<A: SockAddr>(&self, addr: &A) -> Result<(), ConnectError> {
        let (raw, len) = addr.as_raw();
        <_>::e(unsafe { libc::connect(self.as_raw_fd(), raw as *const _ as _, len) })
    }

    /// Returns this socket address.
    #[inline]
    pub fn addr<A: SockAddr>(&self) -> Result<A, AddrError> {
        let mut addr = unsafe { mem::zeroed::<A::Raw>() };
        let mut len = size_of::<A::Raw>() as _;
        match <_>::e(unsafe { libc::getsockname(self.as_raw_fd(), &raw mut addr as _, &mut len) }) {
            Ok(()) => A::from_raw(addr, len),
            Err(err) => Err(err),
        }
    }

    /// Returns the peer socket address.
    #[inline]
    pub fn peer_addr<A: SockAddr>(&self) -> Result<A, AddrError> {
        let mut addr = unsafe { mem::zeroed::<A::Raw>() };
        let mut len = size_of::<A::Raw>() as _;
        let _: () = AddrError::e(unsafe {
            libc::getpeername(self.as_raw_fd(), &raw mut addr as _, &mut len)
        })?;
        A::from_raw(addr, len)
    }

    /// Listen for connections on this socket.
    #[inline]
    pub fn listen(&self) -> Result<(), ListenError> {
        <_>::e(unsafe { libc::listen(self.as_raw_fd(), -1) })
    }
}

// would block
impl Socket {
    /// Read from this socket.
    #[inline]
    pub fn read(&self, buf: &mut [u8]) -> Result<usize, ReadError> {
        let ptr = buf.as_mut_ptr().cast();
        unsafe { <_>::io(libc::read(self.as_raw_fd(), ptr, buf.len())) }
    }

    /// Write to this socket.
    #[inline]
    pub fn write(&self, buf: &[u8]) -> Result<usize, WriteError> {
        let ptr = buf.as_ptr().cast();
        unsafe { <_>::io(libc::write(self.as_raw_fd(), ptr, buf.len())) }
    }

    /// Poll read from this socket.
    #[inline]
    pub fn poll_read(&self, buf: &mut [u8]) -> Poll<Result<usize, ReadError>> {
        <_>::ep(Self::read(self, buf))
    }

    /// Poll write to this socket.
    #[inline]
    pub fn poll_write(&self, buf: &[u8]) -> Poll<Result<usize, WriteError>> {
        <_>::ep(Self::write(self, buf))
    }

    /// Accept a connection on this socket.
    #[inline]
    pub fn accept(&self) -> Result<Self, AcceptError> {
        unsafe { <_>::fd(libc::accept(self.as_raw_fd(), 0 as _, 0 as _)) }
    }

    /// Accept a connection on this socket and apply given flags.
    #[inline]
    pub fn accept4(&self, flags: Flags) -> Result<Self, AcceptError> {
        unsafe { <_>::fd(libc::accept4(self.as_raw_fd(), 0 as _, 0 as _, flags.0)) }
    }

    /// Poll accept a connection on this socket.
    #[inline]
    pub fn poll_accept(&self) -> Poll<Result<Self, AcceptError>> {
        <_>::ep(Self::accept(self))
    }

    /// Poll accept a connection on this socket and apply given flags.
    #[inline]
    pub fn poll_accept4(&self, flags: Flags) -> Poll<Result<Self, AcceptError>> {
        <_>::ep(Self::accept4(self, flags))
    }
}

// ===== SockAddr =====

/// A socket address.
pub trait SockAddr: sealed::Sealed {}
mod sealed {
    pub trait Sealed: Sized {
        type Raw;
        fn as_raw(&self) -> (&Self::Raw, libc::socklen_t);
        fn from_raw(raw: Self::Raw, len: u32) -> Result<Self, super::AddrError>;
    }
}

// ===== SockaddrUn =====

const SUN_PATH_OFFSET: usize = mem::offset_of!(libc::sockaddr_un, sun_path);

/// UNIX domain socket address.
#[derive(Debug)]
pub struct SockaddrUn {
    addr: libc::sockaddr_un,
    len: u32,
}

impl SockaddrUn {
    /// Creates [`SockaddrUn`] with given path.
    #[inline]
    pub const fn from_path(path: &CStr) -> Result<Self, AddrTooLong> {
        let mut addr = unsafe { mem::zeroed::<libc::sockaddr_un>() };
        addr.sun_family = libc::AF_UNIX as _;
        let path = path.to_bytes_with_nul();
        if path.len() > addr.sun_path.len() {
            return Err(AddrTooLong);
        }
        unsafe {
            addr.sun_path
                .as_mut_ptr()
                .copy_from_nonoverlapping(path.as_ptr().cast(), path.len());
        };
        // `unix(7)`
        let len = (SUN_PATH_OFFSET + path.len()) as _;
        Ok(Self { addr, len })
    }

    /// Returns the address pathname.
    #[inline]
    pub fn as_pathname(&self) -> Option<&CStr> {
        // `unix(7)`
        let addr_len = self.len as usize - SUN_PATH_OFFSET;
        if addr_len == 0 {
            // unnamed
            return None;
        } else if self.addr.sun_path[0] == 0 {
            // abstract
            return None;
        }
        unsafe {
            let path = mem::transmute::<&[c_char], &[u8]>(&self.addr.sun_path[..]);
            let path = path.get_unchecked(..addr_len);
            Some(CStr::from_bytes_with_nul_unchecked(path))
        }
    }
}

impl SockAddr for SockaddrUn {}
impl sealed::Sealed for SockaddrUn {
    type Raw = libc::sockaddr_un;

    #[inline]
    fn as_raw(&self) -> (&Self::Raw, libc::socklen_t) {
        (&self.addr, self.len)
    }

    #[inline]
    fn from_raw(addr: Self::Raw, mut len: u32) -> Result<Self, AddrError> {
        if len == 0 {
            len = SUN_PATH_OFFSET as _;
        } else if addr.sun_family != libc::AF_UNIX as _ {
            return Err(ErrCode::new(libc::EINVAL).into());
        }
        Ok(Self { addr, len })
    }
}

// ===== Domain =====

/// Socket address family.
#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct Domain(i32);

// `address_families(7)`
impl Domain {
    /// Local communication.
    ///
    /// See `unix(7)`.
    pub const UNIX: Self = Self(libc::AF_UNIX);
    /// Synonym for [`Domain::LOCAL`].
    pub const LOCAL: Self = Self(libc::AF_LOCAL);
    /// IPv4 Internet protocols.
    ///
    /// See `ip(7)`.
    pub const INET: Self = Self(libc::AF_INET);
}

// ===== Type =====

/// Socket type.
#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct Type(i32);

// `/usr/include/bits/socket.h`
impl Type {
    /// Provides sequenced, reliable, two-way, connection-based byte streams.
    ///
    /// An out-of-band data transmission mechanism may be supported.
    pub const STREAM: Self = Self(libc::SOCK_STREAM);
    /// Supports datagrams (connectionless, unreliable messages of a fixed maximum length).
    pub const DGRAM: Self = Self(libc::SOCK_DGRAM);
    /// Provides raw network protocol access.
    pub const RAW: Self = Self(libc::SOCK_RAW);
}

// ===== Flags =====

/// Socket creation flags.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Flags(i32);

// `/usr/include/bits/socket.h`
impl Flags {
    /// Set the close-on-exec (FD_CLOEXEC) flag on the new fd.
    pub const CLOEXEC: Self = Self(libc::SOCK_CLOEXEC);
    /// Set the `O_NONBLOCK` file status flag on the new fd.
    pub const NONBLOCK: Self = Self(libc::SOCK_NONBLOCK);
}

impl OpenFlag for Flags {
    const CLOEXEC: Self = Self::CLOEXEC;
    const NONBLOCK: Self = Self::NONBLOCK;
}

impl_bitops_simple!(Flags);

// ===== AddTooLong =====

/// An error that occur when supplying socket address that is too long.
#[derive(Debug, Clone, Copy)]
pub struct AddrTooLong;

impl error::Error for AddrTooLong { }

impl fmt::Display for AddrTooLong {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "socket address path is too long")
    }
}

// ===== CreateError =====

/// An error that may occur when creating socket.
#[derive(Clone, Copy)]
pub struct CreateError(ErrCode);

os_error_simple!(CreateError, "create socket");

// ===== ListenError =====

/// An error that may occur when listening on socket.
#[derive(Clone, Copy)]
pub struct ListenError(ErrCode);

os_error_simple!(ListenError, "listen for connections on a socket");

// ===== AddrError =====

/// An error that may occur when querying address on a socket.
#[derive(Clone, Copy)]
pub struct AddrError(ErrCode);

os_error_simple!(AddrError, "get an address on a socket");

// ===== AcceptError =====

/// An error that may occur when accepting connection on a socket.
#[derive(Clone, Copy)]
pub struct AcceptError(ErrCode);

os_error_simple!(AcceptError, "accept connection on a socket");

// ===== ReadError =====

/// An error that may occur when reading from a socket.
#[derive(Clone, Copy)]
pub struct ReadError(ErrCode);

os_error_simple!(ReadError, "read from a socket");

// ===== WriteError =====

/// An error that may occur when writing to a socket.
#[derive(Clone, Copy)]
pub struct WriteError(ErrCode);

os_error_simple!(WriteError, "write to a socket");

// ===== ConnectError =====

/// An error that can occur during socket connecting.
#[derive(Debug)]
pub enum ConnectError {
    /// Address creating failed.
    Addr(AddrTooLong),
    /// Connect call failed.
    Connect(ErrCode),
}

// ===== BindError =====

/// An error that can occur during socket name binding.
#[derive(Debug)]
pub enum BindError {
    /// Address creating failed.
    Addr(AddrTooLong),
    /// Bind call failed.
    Bind(ErrCode),
}

macro_rules! error_with_bind {
    ($me:ident::$vr:ident, $cx:literal) => {
        impl error::Error for $me {}
        impl fmt::Display for $me {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                match self {
                    Self::Addr(err) => err.fmt(f),
                    Self::$vr(err) => write!(f, "failed to {}: {}", $cx, err),
                }
            }
        }
        impl From<ErrCode> for $me {
            #[inline]
            fn from(v: ErrCode) -> Self { Self::$vr(v) }
        }
        impl From<AddrTooLong> for $me {
            #[inline]
            fn from(v: AddrTooLong) -> Self { Self::Addr(v) }
        }
        impl FromErrCode for $me {
            fn from_err_code(code: ErrCode) -> Self {
                Self::$vr(code)
            }
        }
    };
}
error_with_bind!(ConnectError::Connect, "initiate connection on a socket");
error_with_bind!(BindError::Bind, "bind address to socket");

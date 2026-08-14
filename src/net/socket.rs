//! Linux socket.
use core::ffi::{CStr, c_char};
use core::task::Poll;
use core::{error, fmt, mem, result};

use crate::error::ErrCode;
use crate::fd::{AsRawFd, FromRawFd, OwnedFd, impl_fd_simple};
use crate::flags::{OpenFlag, impl_bitops_simple};

// ===== Socket =====

/// Linux socket.
#[derive(Debug)]
pub struct Socket(OwnedFd);

impl_fd_simple!(Socket);

impl Socket {
    /// Creates new [`Socket`].
    #[inline]
    pub fn create(domain: Domain, ty: Type, flags: Flags) -> Result<Self> {
        unsafe { fd(libc::socket(domain.0, ty.0 | flags.0, 0), Kind::Create) }
    }

    /// Creates new UNIX domain [`Socket`] stream.
    #[inline]
    pub fn unix_stream(flags: Flags) -> Result<Self> {
        Self::create(Domain::UNIX, Type::STREAM, flags)
    }

    /// Bind address to this socket.
    #[inline]
    pub fn bind<A: SockAddr>(&self, addr: &A) -> Result<()> {
        let (raw, len) = addr.as_raw();
        unsafe { e(libc::bind(self.as_raw_fd(), raw as *const _ as _, len), Kind::Bind) }
    }

    /// Initiate a connection on this socket.
    #[inline]
    pub fn connect<A: SockAddr>(&self, addr: &A) -> Result<()> {
        let (raw, len) = addr.as_raw();
        unsafe { e(libc::connect(self.as_raw_fd(), raw as *const _ as _, len), Kind::Connect) }
    }

    /// Returns this socket address.
    #[inline]
    pub fn addr<A: SockAddr>(&self) -> Result<A> {
        let mut addr = unsafe { mem::zeroed::<A::Raw>() };
        let mut len = size_of::<A::Raw>() as _;
        let res = unsafe { libc::getsockname(self.as_raw_fd(), &raw mut addr as _, &mut len) };
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
        let res = unsafe { libc::getpeername(self.as_raw_fd(), &raw mut addr as _, &mut len) };
        match e(res, Kind::GetAddr) {
            Ok(()) => A::from_raw(addr, len).map_err(<_>::into),
            Err(err) => Err(err),
        }
    }

    /// Listen for connections on this socket.
    #[inline]
    pub fn listen(&self) -> Result<()> {
        unsafe { e(libc::listen(self.as_raw_fd(), -1), Kind::Listen) }
    }
}

// would block
impl Socket {
    /// Read from this socket.
    #[inline]
    pub fn read(&self, buf: &mut [u8]) -> Result<usize> {
        let ptr = buf.as_mut_ptr().cast();
        unsafe { io(libc::read(self.as_raw_fd(), ptr, buf.len()), Kind::Read) }
    }

    /// Write to this socket.
    #[inline]
    pub fn write(&self, buf: &[u8]) -> Result<usize> {
        let ptr = buf.as_ptr().cast();
        unsafe { io(libc::write(self.as_raw_fd(), ptr, buf.len()), Kind::Write) }
    }

    /// Accept a connection on this socket.
    #[inline]
    pub fn accept(&self) -> Result<Self> {
        unsafe { fd(libc::accept(self.as_raw_fd(), 0 as _, 0 as _), Kind::Accept) }
    }

    /// Accept a connection on this socket and apply given flags.
    #[inline]
    pub fn accept4(&self, flags: Flags) -> Result<Self> {
        unsafe { fd(libc::accept4(self.as_raw_fd(), 0 as _, 0 as _, flags.0), Kind::Accept) }
    }

    /// Poll read from this socket.
    #[inline]
    pub fn poll_read(&self, buf: &mut [u8]) -> Poll<Result<usize>> {
        ep(Self::read(self, buf))
    }

    /// Poll write to this socket.
    #[inline]
    pub fn poll_write(&self, buf: &[u8]) -> Poll<Result<usize>> {
        ep(Self::write(self, buf))
    }

    /// Poll accept a connection on this socket.
    #[inline]
    pub fn poll_accept(&self) -> Poll<Result<Self>> {
        ep(Self::accept(self))
    }

    /// Poll accept a connection on this socket and apply given flags.
    #[inline]
    pub fn poll_accept4(&self, flags: Flags) -> Poll<Result<Self>> {
        ep(Self::accept4(self, flags))
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
    pub const fn from_path(path: &CStr) -> Result<Self, AddrError> {
        let mut addr = unsafe { mem::zeroed::<libc::sockaddr_un>() };
        addr.sun_family = libc::AF_UNIX as _;
        let path = path.to_bytes_with_nul();
        if path.len() > addr.sun_path.len() {
            return Err(AddrError::ExcessivePath);
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
            return Err(AddrError::MissmatchDomain);
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

// ===== Error =====

fn fd<T: FromRawFd>(res: i32, kind: Kind) -> Result<T> {
    if res == -1 {
        return Err(Error::errno(kind));
    }
    Ok(unsafe { T::from_raw_fd(res) })
}

fn e(res: i32, kind: Kind) -> Result<()> {
    if res == -1 {
        return Err(Error::errno(kind));
    }
    Ok(())
}

fn io(res: isize, kind: Kind) -> Result<usize> {
    match usize::try_from(res) {
        Ok(ok) => Ok(ok),
        Err(_) => Err(Error::errno(kind)),
    }
}

fn ep<T>(res: Result<T>) -> Poll<Result<T>> {
    match res {
        Ok(ok) => Poll::Ready(Ok(ok)),
        Err(err) => {
            if err.code.would_block() {
                Poll::Pending
            } else {
                Poll::Ready(Err(err))
            }
        }
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
    fn errno(kind: Kind) -> Self {
        Self { kind, code: ErrCode::errno() }
    }
}

impl From<AddrError> for Error {
    #[inline]
    fn from(v: AddrError) -> Self {
        Self { kind: Kind::Addr(v), code: ErrCode::new(libc::EINVAL) }
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

// ===== SockAddrError =====

/// An error that occur when validating socket address.
#[derive(Debug, Clone, Copy)]
pub enum AddrError {
    /// Address path length exceeds maximum capacity.
    ExcessivePath,
    /// Domain in generic socket address does not match.
    MissmatchDomain,
}

impl error::Error for AddrError {}

impl fmt::Display for AddrError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match self {
            Self::ExcessivePath => "excessive path length",
            Self::MissmatchDomain => "missmatch domain name",
        };
        msg.fmt(f)
    }
}

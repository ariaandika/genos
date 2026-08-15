//! Socket address.
use core::ffi::{CStr, c_char};
use core::{error, fmt, mem};

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

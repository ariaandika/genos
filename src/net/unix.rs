//! UNIX socket address.
use core::{ffi, fmt, marker, mem};

use crate::ffi::Char;
use crate::net::addr::{AddrError, Family, SockAddr};
use crate::net::cmsg::{CMsgKind, CMsgType};
use crate::net::{SaFamily, raw};

// ===== SockaddrUn =====

/// UNIX domain socket address.
///
/// See `sockaddr_un(3type)`.
#[repr(C)]
pub struct SockAddrUn {
    sun_family: SaFamily,
    sun_path: [ffi::c_char; raw::UNIX_PATH_MAX],
}

impl SockAddrUn {
    /// Creates [`SockAddrUn`] with given path.
    ///
    /// # Errors
    ///
    /// Returns error if path is too long. UNIX domain address path musst be less than 108 including
    /// the null termination byte.
    #[inline]
    pub const fn from_path(path: &Char) -> Result<Self, AddrError> {
        let mut addr = unsafe { mem::zeroed::<Self>() };
        addr.sun_family = Family::UNIX.sa_family();

        if strlen(path) > const { raw::UNIX_PATH_MAX - 1 } {
            return Err(AddrError::ExcessivePath);
        }

        unsafe {
            addr.sun_path
                .as_mut_ptr()
                .copy_from_nonoverlapping(path.as_ptr(), raw::UNIX_PATH_MAX);
        };
        Ok(addr)
    }

    /// Cast to generic [`SockAddr`]
    #[inline]
    pub const fn as_sockaddr(&self) -> &SockAddr {
        // SAFETY: SockAddr is a subset of SockAddrUn
        unsafe { &*(self as *const Self as *const SockAddr) }
    }

    /// Returns the address as pathname.
    ///
    /// Returns [`None`] if the address is unnamed or abstract.
    #[inline]
    pub fn as_pathname(&self) -> Option<&Char> {
        if self.sun_path[0] == 0 {
            return None;
        }
        // SAFETY: `sun_path` guarantee to be null terminated
        unsafe { Some(&*self.sun_path.as_ptr().cast()) }
    }

    /// Returns the address as abstract address.
    ///
    /// Returns [`None`] if the addres is unnamed or pathname.
    #[inline]
    pub fn as_abstract(&self) -> Option<&Char> {
        if self.sun_path[0] != 0 {
            // pathname
            return None;
        }
        if self.sun_path[1] == 0 {
            // unnamed
            return None;
        }
        unsafe { Some(&*self.sun_path.as_ptr().add(2).cast()) }
    }
}

// ===== trait impls =====

impl fmt::Debug for SockAddrUn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let path = self.as_pathname().unwrap_or_else(|| Char::new(c".."));
        f.debug_tuple("SockAddrUn").field(&path).finish()
    }
}

// ===== cmsg =====

/// Control message to send or receive a set of open fd from another process.
#[derive(Debug)]
pub struct SCMRights(marker::PhantomData<()>);

impl SCMRights {
    /// Maximum number of file descriptors that can be in the control message buffer.
    pub const MAX_FD: i32 = raw::SCM_MAX_FD;
}

impl CMsgKind for SCMRights {
    type Data = i32;

    const TYPE: CMsgType = CMsgType::RIGHTS;
}

const fn strlen(str: &Char) -> usize {
    let mut n = 0;
    while unsafe { *str.as_ptr().add(n) != 0 } {
        n += 1;
    }
    n
}

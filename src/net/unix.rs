//! UNIX socket address.
use core::ffi::CStr;
use core::{ffi, fmt, mem};

use crate::net::addr::{AddrError, Family, SockAddr, sealed};
use crate::sys;

// ===== SockaddrUn =====

/// UNIX domain socket address.
///
/// See `sockaddr_un(3type)`.
#[repr(C)]
pub struct SockAddrUn {
    sun_family: sys::sa_family_t,
    sun_path: [ffi::c_char; sys::UNIX_PATH_MAX],
}

impl SockAddrUn {
    /// Creates [`SockAddrUn`] with given path.
    ///
    /// # Errors
    ///
    /// Returns error if path is too long. UNIX domain address path musst be less than 108 including
    /// the null termination byte.
    #[inline]
    pub const fn from_path(path: &CStr) -> Result<Self, AddrError> {
        let mut addr = unsafe { mem::zeroed::<Self>() };
        addr.sun_family = Family::UNIX.sa_family();

        if path.count_bytes() > const { sys::UNIX_PATH_MAX - 1 } {
            return Err(AddrError::ExcessivePath);
        }

        unsafe {
            addr.sun_path
                .as_mut_ptr()
                .copy_from_nonoverlapping(path.as_ptr(), sys::UNIX_PATH_MAX);
        };
        Ok(addr)
    }

    fn path(&self) -> &CStr {
        // SAFETY: `sun_path` guarantee to be null terminated
        unsafe { CStr::from_ptr(self.sun_path.as_ptr().cast()) }
    }

    /// Returns the address pathname.
    #[inline]
    pub fn as_pathname(&self) -> Option<&CStr> {
        // `unix(7)`
        let addr_len = self.path().count_bytes() + 1;
        if addr_len == 0 {
            // unnamed
            return None;
        } else if self.sun_path[0] == 0 {
            // abstract
            return None;
        }
        unsafe {
            let path = mem::transmute::<&[ffi::c_char], &[u8]>(&self.sun_path[..]);
            let path = path.get_unchecked(..addr_len);
            Some(CStr::from_bytes_with_nul_unchecked(path))
        }
    }
}

// ===== trait impls =====

impl SockAddr for SockAddrUn {
    const FAMILY: Family = Family::UNIX;
}

impl sealed::Sealed for SockAddrUn {}

impl fmt::Debug for SockAddrUn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("SockAddrUn")
            .field(&self.as_pathname().unwrap_or(c"<abstract>"))
            .finish()
    }
}

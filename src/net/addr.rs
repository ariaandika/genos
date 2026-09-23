//! Socket address.
use core::ffi;

use crate::net::{SaFamily, raw};

// ===== SockAddr =====

/// `sockaddr(3type)`
#[derive(Debug)]
#[repr(C)]
pub struct SockAddr {
    /// `sockaddr.sa_family`
    pub sa_family: SaFamily,
    /// `sockaddr.sa_data`
    pub sa_data: [ffi::c_char; 14],
}

// ===== Family =====

/// Socket address family (`address_families(7)`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Family(i32);

impl Family {
    /// `AF_UNIX`
    pub const UNIX: Self = Self(raw::AF_UNIX);
    /// `AF_LOCAL`
    pub const LOCAL: Self = Self(raw::AF_LOCAL);
    /// `AF_INET`
    pub const INET: Self = Self(raw::AF_INET);
}

impl Family {
    pub(super) const fn sa_family(self) -> SaFamily {
        self.0 as _
    }
}

impl From<Family> for i32 {
    #[inline]
    fn from(value: Family) -> Self {
        value.0
    }
}

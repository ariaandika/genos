//! Socket address.
use core::{error, fmt};

use crate::sys;

// ===== SockAddr =====

/// Socket family specific address.
pub trait SockAddr: sealed::Sealed {
    /// The socket address family.
    const FAMILY: Family;
}

pub(super) mod sealed {
    use crate::sys;

    /// Implementor must guarantee:
    ///
    /// - the struct layout is a superset of [`sys::sockaddr`].
    /// - the struct layout is valid for all zero bits
    pub trait Sealed: Sized {
        fn sa_family(&self) -> sys::sa_family_t {
            // SAFETY: guarantee by the implementor
            unsafe { &*(self as *const Self as *const sys::sockaddr) }.sa_family
        }

        fn zeroed() -> Self {
            // SAFETY: guarantee by the implementor
            unsafe { core::mem::zeroed() }
        }

        fn socklen_t() -> sys::socklen_t {
            size_of::<Self>() as _
        }
    }
}

// ===== Family =====

/// Socket address family.
///
/// For more details, see `address_families(7)`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Family(i32);

impl Family {
    /// Local communication.
    ///
    /// See `unix(7)`.
    pub const UNIX: Self = Self(sys::AF_UNIX);
    /// Synonym for [`Family::LOCAL`].
    pub const LOCAL: Self = Self(sys::AF_LOCAL);
    /// IPv4 Internet protocols.
    ///
    /// See `ip(7)`.
    pub const INET: Self = Self(sys::AF_INET);
}

impl Family {
    pub(super) const fn sa_family(self) -> sys::sa_family_t {
        self.0 as _
    }
}

impl From<Family> for i32 {
    #[inline]
    fn from(value: Family) -> Self {
        value.0
    }
}

// ===== SockAddrError =====

/// An error that occur when validating socket address.
#[derive(Debug, Clone, Copy)]
pub enum AddrError {
    /// Address path length exceeds maximum capacity.
    ExcessivePath,
    /// Family in generic socket address does not match.
    MissmatchFamily,
}

impl error::Error for AddrError {}

impl fmt::Display for AddrError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match self {
            Self::ExcessivePath => "excessive path length",
            Self::MissmatchFamily => "missmatch socket family name",
        };
        msg.fmt(f)
    }
}

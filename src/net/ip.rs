//! IPv4 socket address.
use core::mem::MaybeUninit;
use core::{ffi, fmt};

use crate::net::SaFamily;
use crate::net::addr::{Family, SockAddr};
use crate::net::raw::Socklen;

/// IPv4 socket address (`sockaddr_in(3type)`).
#[repr(C)]
pub struct SockAddrIn {
    sin_family: SaFamily,
    sin_port: u16,
    sin_addr: in_addr,
    __pad: [MaybeUninit<u8>;
        size_of::<SockAddr>()
            - size_of::<ffi::c_short>()
            - size_of::<ffi::c_ushort>()
            - size_of::<in_addr>()],
}

#[repr(C)]
struct in_addr {
    s_addr: u32,
}

impl SockAddrIn {
    /// Create new [`SockAddrIn`].
    ///
    /// Note that both value must be in network byte order.
    #[inline]
    pub const fn new(port: u16, addr: u32) -> Self {
        Self {
            sin_family: Family::INET.sa_family(),
            sin_port: port,
            sin_addr: in_addr { s_addr: addr },
            __pad: [const { MaybeUninit::uninit() }; _],
        }
    }

    /// Cast address to [`SockAddr`]
    #[inline]
    pub const fn as_sockaddr(&self) -> &SockAddr {
        // SAFETY: SockAddr is a subset of SockAddrIn
        unsafe { &*(self as *const Self as *const SockAddr) }
    }

    /// Returns this struct size as [`Socklen`].
    #[inline]
    pub const fn addrlen(&self) -> Socklen {
        size_of::<Self>() as Socklen
    }
}

// ===== trait impls =====

impl fmt::Debug for SockAddrIn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SockAddrIn")
            .field("addr", &self.sin_addr.s_addr)
            .field("port", &self.sin_port)
            .finish()
    }
}

// ===== extern =====

// include/uapi/linux/in.h

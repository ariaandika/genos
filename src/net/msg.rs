//! Socket message.
use crate::flags::impl_bitops_simple;

// ===== SendFlags =====

/// Message sending operation flags.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct SendFlags(i32);

impl SendFlags {
    /// Enables nonblocking operation; if the operation would block, the call fails with EAGAIN or
    /// EWOULDBLOCK.
    pub const DONTWAIT: Self = Self(libc::MSG_DONTWAIT);
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
    pub const CMSG_CLOEXEC: Self = Self(libc::MSG_CMSG_CLOEXEC);
    /// Enables nonblocking operation; if the operation would block, the call fails with EAGAIN or
    /// EWOULDBLOCK.
    pub const DONTWAIT: Self = Self(libc::MSG_DONTWAIT);
    /// Receive message without removing that data from the queue.
    pub const PEEK: Self = Self(libc::MSG_PEEK);
}

impl_bitops_simple!(RecvFlags);

// ===== impl traits =====

impl From<SendFlags> for i32 {
    #[inline]
    fn from(value: SendFlags) -> Self {
        value.0
    }
}

impl From<RecvFlags> for i32 {
    #[inline]
    fn from(value: RecvFlags) -> Self {
        value.0
    }
}

//! Socket options.
use core::marker;

use crate::net::{Socklen, raw};

// ===== OptLevel =====

/// Socket options level.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct OptLevel(i32);

impl OptLevel {
    /// Returns the raw integer value.
    #[inline]
    pub const fn to_raw(self) -> i32 {
        self.0
    }
}

impl From<OptLevel> for i32 {
    #[inline]
    fn from(value: OptLevel) -> Self {
        value.0
    }
}

/// `SOL_IP`
pub const IP: OptLevel = OptLevel(raw::SOL_IP);
/// `SOL_SOCKET`
pub const SOCKET: OptLevel = OptLevel(raw::SOL_SOCKET);
/// `SOL_TCP`
pub const TCP: OptLevel = OptLevel(raw::SOL_TCP);
/// `SOL_UDP`
pub const UDP: OptLevel = OptLevel(raw::SOL_UDP);
/// `SOL_IPV6`
pub const IPV6: OptLevel = OptLevel(raw::SOL_IPV6);
/// `SOL_RAW`
pub const RAW: OptLevel = OptLevel(raw::SOL_RAW);

// ===== OptName =====

/// Socket options name (`socket(7)`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct OptName(i32);

/// `SO_DEBUG`
pub const DEBUG: OptName = OptName(raw::SO_DEBUG);
/// `SO_REUSEADDR`
pub const REUSEADDR: OptName = OptName(raw::SO_REUSEADDR);
/// `SO_TYPE`
pub const TYPE: OptName = OptName(raw::SO_TYPE);
/// `SO_ERROR`
pub const ERROR: OptName = OptName(raw::SO_ERROR);
/// `SO_DONTROUTE`
pub const DONTROUTE: OptName = OptName(raw::SO_DONTROUTE);
/// `SO_BROADCAST`
pub const BROADCAST: OptName = OptName(raw::SO_BROADCAST);
/// `SO_KEEPALIVE`
pub const KEEPALIVE: OptName = OptName(raw::SO_KEEPALIVE);

impl From<OptName> for i32 {
    #[inline]
    fn from(value: OptName) -> Self {
        value.0
    }
}

// ===== OptValue =====

/// Socket options value (`socket(7)`).
#[derive(Debug)]
pub struct OptValue {
    _p: marker::PhantomData<()>,
}

/// Create boolean integer option value.
#[inline]
pub const fn from_bool(boolean: bool) -> OptInt {
    OptInt::new(boolean as i32)
}

// ===== values =====

/// Socket options integer value (`socket(7)`).
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct OptInt(i32);

impl OptInt {
    /// Creates new [`OptInt`] with given integer value.
    #[inline]
    pub const fn new(int: i32) -> Self {
        Self(int)
    }

    /// Returns the [`OptValue`] representation.
    #[inline]
    pub const fn as_value(&self) -> &OptValue {
        unsafe { &*(self as *const _ as *const _) }
    }

    /// Returns the mutable [`OptValue`] representation.
    #[inline]
    pub const fn as_value_mut(&mut self) -> &mut OptValue {
        unsafe { &mut *(self as *mut _ as *mut _) }
    }

    /// Returns the value length in bytes.
    #[inline]
    pub const fn size(&self) -> Socklen {
        size_of::<i32>() as Socklen
    }

    /// Returns the raw integer value.
    #[inline]
    pub const fn to_raw(&self) -> i32 {
        self.0
    }
}

impl From<OptInt> for i32 {
    #[inline]
    fn from(value: OptInt) -> Self {
        value.0
    }
}

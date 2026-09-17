//! Random number generator.
use core::mem::MaybeUninit;

use crate::error::{ErrCode, SysResExt};
use crate::{error, flags, sys};

/// Generate a random number to the given buffer (`getrandom(2)`).
#[inline]
pub fn getrandom(buf: &mut [MaybeUninit<u8>], flags: Flags) -> Result<usize, Error> {
    sys::call!(sys_getrandom, buf.as_mut_ptr(), buf.len(), flags.0).io2()
}

// ===== Flags =====

/// [`getrandom`] flags.
#[derive(Debug, Default, Clone, Copy)]
pub struct Flags(u32);

flags::impl_bitops_simple!(Flags);

impl Flags {
    /// `GRND_RANDOM`
    pub const RANDOM: Self = Self(GRND_RANDOM);
    /// `GRND_NONBLOCK`
    pub const NONBLOCK: Self = Self(GRND_NONBLOCK);
    /// `GRND_INSECURE`
    pub const INSECURE: Self = Self(GRND_INSECURE);
}

// ===== Error =====

/// An error that may occur during [`getrandom`] operation.
#[derive(Clone, Copy)]
pub struct Error(ErrCode);

error::impl_error_os_simple!(Error, "generate random number");

// ===== extern =====

// include/uapi/linux/random.h

const GRND_NONBLOCK: u32 = 0x0001;
const GRND_RANDOM: u32 = 0x0002;
const GRND_INSECURE: u32 = 0x0004;

//! Random number generator.
use core::mem::MaybeUninit;

use crate::sys::SysRes;
use crate::{flags, sys};

/// Generate a random number to the given buffer (`getrandom(2)`).
#[inline]
pub fn getrandom(buf: &mut [MaybeUninit<u8>], flags: Flags) -> impl SysRes<usize> {
    sys::call!(sys_getrandom, buf.as_mut_ptr(), buf.len(), flags.0)
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

// ===== extern =====

// include/uapi/linux/random.h

const GRND_NONBLOCK: u32 = 0x0001;
const GRND_RANDOM: u32 = 0x0002;
const GRND_INSECURE: u32 = 0x0004;

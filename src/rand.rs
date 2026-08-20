//! Random number generator.
use core::mem::MaybeUninit;

use crate::error::{ErrCode, SysResExt};
use crate::{error, flags, sys};

/// Generate a random number to the given buffer.
#[inline]
pub fn rand(buf: &mut [MaybeUninit<u8>], flags: Flags) -> Result<usize, Error> {
    sys::call!(__NR_getrandom, &mut *buf, buf.len(), flags.0).io2()
}

// ===== Flags =====

/// [`rand`] flags.
#[derive(Debug, Default, Clone, Copy)]
pub struct Flags(u32);

impl Flags {
    /// Random bytes are drawn from the random source instead of the urandom source.
    ///
    /// The random source is limited based on the entropy that can be obtained from environmental
    /// noise. If the number of available bytes in the random source is less than requested in size,
    /// the call returns just the avail‐ able random bytes. If no random bytes are available, the
    /// behavior depends on the presence of [`Flags::NONBLOCK`] in the flags argument.
    pub const RANDOM: Self = Self(sys::GRND_RANDOM);
    /// Do not block, but instead immediately returns EAGAIN error.
    ///
    /// By default, when reading from the random source, [`rand`] blocks if no random bytes are
    /// available, and when reading from the urandom source, it blocks if the entropy pool has not
    /// yet been initialized.
    pub const NONBLOCK: Self = Self(sys::GRND_NONBLOCK);
    /// Return non-cryptographic random bytes.
    pub const INSECURE: Self = Self(sys::GRND_INSECURE);
}

flags::impl_bitops_simple!(Flags);

// ===== Error =====

/// An error that may occur when getting random number.
#[derive(Clone, Copy)]
pub struct Error(ErrCode);

error::impl_error_os_simple!(Error, "get random number");

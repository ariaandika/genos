//! [`Mmap`] associated types.
use core::ptr::NonNull;
use core::slice;

use crate::error::{ErrCode, SysResExt};
use crate::fd::BorrowedFd;
use crate::{error, flags, sys};

/// Map files or devices into memory.
#[derive(Debug)]
pub struct Mmap {
    ptr: NonNull<u8>,
    len: usize,
}

impl Drop for Mmap {
    #[inline]
    fn drop(&mut self) {
        sys::call!(RD, __NR_munmap, self.ptr, self.len);
    }
}

impl Mmap {
    /// Creates new [`Mmap`].
    #[inline]
    pub fn new(
        len: usize,
        prot: Protection,
        flags: Flags,
        fd: BorrowedFd<'_>,
        offset: i64,
    ) -> Result<Self, Error> {
        sys::call!(RD, __NR_mmap, 0, len, prot.0, flags.0, fd, offset)
            .p2()
            .map(|ptr| Self { ptr, len })
    }

    /// Returns the memory as slice bytes.
    ///
    /// # Safety
    ///
    /// Caller must have the read permission.
    #[inline]
    pub unsafe fn as_slice(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self.ptr.as_ptr(), self.len) }
    }

    /// Returns the memory as mutable slice bytes.
    ///
    /// # Safety
    ///
    /// Caller must have the write permission.
    #[inline]
    pub unsafe fn as_mut_slice(&mut self) -> &mut [u8] {
        unsafe { slice::from_raw_parts_mut(self.ptr.as_ptr(), self.len) }
    }
}

// ===== Protection =====

/// Memory protection flags of the [`Mmap`] mapping.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Protection(i32);

impl Protection {
    /// Pages may not be accessed.
    ///
    /// This is the default empty value, not to be ORed with other constant.
    pub const NONE: Self = Self(sys::PROT_NONE);
    /// Pages may be executed.
    pub const EXEC: Self = Self(sys::PROT_EXEC);
    /// Pages may be read.
    pub const READ: Self = Self(sys::PROT_READ);
    /// Pages may be write.
    pub const WRITE: Self = Self(sys::PROT_WRITE);
}

flags::impl_bitops_simple!(Protection);

// ===== Flags =====

/// [`Mmap`] creation flags.
///
/// This struct does not implement default, either [`Flags::SHARED`] [`Flags::SHARED_VALIDATE`]
/// [`Flags::PRIVATE`] must be specified mutually exclusive.
///
/// See `mmap(2)`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Flags(i32);

impl Flags {
    /// Shared this mapping.
    pub const SHARED: Self = Self(sys::MAP_SHARED);
    /// Same as [`Flags::SHARED`] and validate unknown flags.
    ///
    /// Since Linux 4.15.
    pub const SHARED_VALIDATE: Self = Self(sys::MAP_SHARED_VALIDATE);
    /// Create a private copy-on-write mapping.
    pub const PRIVATE: Self = Self(sys::MAP_PRIVATE);

    /// Synonym for MAP_ANONYMOUS.
    pub const ANON: Self = Self(sys::MAP_ANONYMOUS);
    /// The mapping is not backed by any file; its contents are initialized to zero.
    ///
    /// The `fd` argument is ignored; however, some implementations require `fd` to be -1 if
    /// MAP_ANONYMOUS (or MAP_ANON) is specified, and portable applications should ensure this. The
    /// offset argument should be zero.
    pub const ANONYMUS: Self = Self(sys::MAP_ANONYMOUS);
}

flags::impl_bitops_simple!(Flags);

// ===== Error =====

/// An error that may occur when mapping into memory.
#[derive(Clone, Copy)]
pub struct Error(ErrCode);

error::impl_error_os_simple!(Error, "map into memory");

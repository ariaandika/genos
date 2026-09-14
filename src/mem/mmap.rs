use core::ffi::c_void;
use core::ptr::NonNull;
use core::slice;

use crate::error::{ErrCode, SysResExt};
use crate::fd::BorrowedFd;
use crate::{error, flags, sys};

// ===== mmap =====

/// Map files or devices into memory.
///
/// Reference: `mmap(2)`.
#[inline]
pub fn mmap(
    addr: *mut c_void,
    size: usize,
    prot: Protection,
    flags: MmapFlags,
    fd: BorrowedFd<'_>,
    offset: i64,
) -> Result<NonNull<u8>, MmapError> {
    sys::call_rd!(sys_mmap, addr, size, prot.0, flags.0, fd, offset).p2()
}

/// Unmap files or devices from memory.
///
/// Reference: `munmap(2)`.
#[inline]
pub fn munmap(addr: *mut c_void, size: usize) -> Result<(), MmapError> {
    sys::call_rd!(sys_munmap, addr, size).e2()
}

// ===== Mmap =====

/// A handle to mapped files or devices in memory.
///
/// Reference: `mmap(2)`.
#[derive(Debug)]
pub struct Mmap {
    ptr: NonNull<u8>,
    len: usize,
}

impl Drop for Mmap {
    #[inline]
    fn drop(&mut self) {
        let _ = munmap(self.ptr.as_ptr().cast(), self.len);
    }
}

impl Mmap {
    /// Creates new [`Mmap`].
    #[inline]
    pub fn new(
        addr: *mut c_void,
        len: usize,
        prot: Protection,
        flags: MmapFlags,
        fd: BorrowedFd<'_>,
        offset: i64,
    ) -> Result<Self, MmapError> {
        mmap(addr, len, prot, flags, fd, offset).map(|ptr| Self { ptr, len })
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
///
/// Reference: `mmap(2)`.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Protection(i32);

flags::impl_bitops_simple!(Protection);

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

// ===== Flags =====

/// [`Mmap`] creation flags.
///
/// This struct does not implement default, either [`MmapFlags::SHARED`]
/// [`MmapFlags::SHARED_VALIDATE`] [`MmapFlags::PRIVATE`] must be specified mutually exclusive.
///
/// Reference: `mmap(2)`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MmapFlags(i32);

flags::impl_bitops_simple!(MmapFlags);

impl MmapFlags {
    /// Shared this mapping.
    pub const SHARED: Self = Self(sys::MAP_SHARED);
    /// Same as [`MmapFlags::SHARED`] and validate unknown flags.
    pub const SHARED_VALIDATE: Self = Self(sys::MAP_SHARED_VALIDATE);
    /// Create a private copy-on-write mapping.
    pub const PRIVATE: Self = Self(sys::MAP_PRIVATE);

    /// Synonym for MAP_ANONYMOUS.
    pub const ANON: Self = Self(sys::MAP_ANONYMOUS);
    /// The mapping is not backed by any file; its contents are initialized to zero.
    pub const ANONYMUS: Self = Self(sys::MAP_ANONYMOUS);
}

// ===== Error =====

/// An error that may occur when mapping into memory.
#[derive(Clone, Copy)]
pub struct MmapError(ErrCode);

error::impl_error_os_simple!(MmapError, "map into memory");

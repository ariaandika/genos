//! [`mmap`] associated types.
use core::ffi::c_void;
use core::ptr::NonNull;

use crate::error::{ErrCode, SysResExt};
use crate::fd::BorrowedFd;
use crate::ffi::Off;
use crate::{error, flags, sys};

// ===== mmap =====

/// Map files or devices into memory (`mmap(2)`).
#[inline]
pub fn mmap(
    addr: *mut c_void,
    length: usize,
    prot: Prot,
    flags: Flags,
    fd: BorrowedFd<'_>,
    offset: Off,
) -> Result<NonNull<u8>, Error> {
    sys::call_rd!(sys_mmap, addr, length, prot.0, flags.0, fd, offset).p2()
}

/// Unmap files or devices from memory (`munmap(2)`).
#[inline]
pub fn munmap(addr: *mut c_void, length: usize) -> Result<(), Error> {
    sys::call_rd!(sys_munmap, addr, length).e2()
}

// ===== Flags =====

/// [`mmap`] flags.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Flags(i32);

flags::impl_bitops_simple!(Flags);

impl Flags {
    /// `MAP_SHARED`
    pub const SHARED: Self = Self(MAP_SHARED);
    /// `MAP_PRIVATE`
    pub const PRIVATE: Self = Self(MAP_PRIVATE);
    /// `MAP_SHARED_VALIDATE`
    pub const SHARED_VALIDATE: Self = Self(MAP_SHARED_VALIDATE);
    /// `MAP_TYPE`
    pub const TYPE: Self = Self(MAP_TYPE);
    /// `MAP_FIXED`
    pub const FIXED: Self = Self(MAP_FIXED);
    /// `MAP_ANONYMOUS`
    pub const ANONYMUS: Self = Self(MAP_ANONYMOUS);
    /// `MAP_POPULATE`
    pub const POPULATE: Self = Self(MAP_POPULATE);
    /// `MAP_NONBLOCK`
    pub const NONBLOCK: Self = Self(MAP_NONBLOCK);
    /// `MAP_STACK`
    pub const STACK: Self = Self(MAP_STACK);
    /// `MAP_HUGETLB`
    pub const HUGETLB: Self = Self(MAP_HUGETLB);
    /// `MAP_SYNC`
    pub const SYNC: Self = Self(MAP_SYNC);
    /// `MAP_FIXED_NOREPLACE`
    pub const FIXED_NOREPLACE: Self = Self(MAP_FIXED_NOREPLACE);
    /// `MAP_UNINITIALIZED`
    pub const UNINITIALIZED: Self = Self(MAP_UNINITIALIZED);
    /// `MAP_GROWSDOWN`
    pub const GROWSDOWN: Self = Self(MAP_GROWSDOWN);
    /// `MAP_LOCKED`
    pub const LOCKED: Self = Self(MAP_LOCKED);
    /// `MAP_NORESERVE`
    pub const NORESERVE: Self = Self(MAP_NORESERVE);
    /// `MAP_ANON`
    pub const ANON: Self = Self(MAP_ANONYMOUS);
}

// ===== Prot =====

/// [`mmap`] protection flags.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Prot(i32);

flags::impl_bitops_simple!(Prot);

impl Prot {
    /// `PROT_READ`
    pub const READ: Self = Self(PROT_READ);
    /// `PROT_WRITE`
    pub const WRITE: Self = Self(PROT_WRITE);
    /// `PROT_EXEC`
    pub const EXEC: Self = Self(PROT_EXEC);
    /// `PROT_SEM`
    pub const SEM: Self = Self(PROT_SEM);
    /// `PROT_NONE`
    pub const NONE: Self = Self(PROT_NONE);
    /// `PROT_GROWSDOWN`
    pub const GROWSDOWN: Self = Self(PROT_GROWSDOWN);
    /// `PROT_GROWSUP`
    pub const GROWSUP: Self = Self(PROT_GROWSUP);
}

// ===== Error =====

/// An error that may occur when mapping memory.
#[derive(Clone, Copy)]
pub struct Error(ErrCode);

error::impl_error_os_simple!(Error, "map memory");

// ===== extern =====

// include/uapi/linux/mman.h

const MAP_SHARED: i32 = 0x01;
const MAP_PRIVATE: i32 = 0x02;
const MAP_SHARED_VALIDATE: i32 = 0x03;

// include/uapi/asm-generic/mman-common.h

const PROT_READ: i32 = 0x1;
const PROT_WRITE: i32 = 0x2;
const PROT_EXEC: i32 = 0x4;
const PROT_SEM: i32 = 0x8;
const PROT_NONE: i32 = 0x0;
const PROT_GROWSDOWN: i32 = 0x01000000;
const PROT_GROWSUP: i32 = 0x02000000;

const MAP_TYPE: i32 = 0x0f;
const MAP_FIXED: i32 = 0x10;
const MAP_ANONYMOUS: i32 = 0x20;

const MAP_POPULATE: i32 = 0x008000;
const MAP_NONBLOCK: i32 = 0x010000;
const MAP_STACK: i32 = 0x020000;
const MAP_HUGETLB: i32 = 0x040000;
const MAP_SYNC: i32 = 0x080000;
const MAP_FIXED_NOREPLACE: i32 = 0x100000;
const MAP_UNINITIALIZED: i32 = 0x4000000;

// include/uapi/asm-generic/mman.h

const MAP_GROWSDOWN: i32 = 0x0100;
// const MAP_DENYWRITE: i32 = 0x0800;
// const MAP_EXECUTABLE: i32 = 0x1000;
const MAP_LOCKED: i32 = 0x2000;
const MAP_NORESERVE: i32 = 0x4000;

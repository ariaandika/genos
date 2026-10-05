use core::ffi;

use crate::{arch, sys};

/// Set architecture specific state (`arch_prctl(2)`).
#[inline]
pub fn arch_prctl(op: ArchOp, addr: ffi::c_ulong) -> Result<(), sys::Error<arch::sys_arch_prctl>> {
    sys::call!(sys_arch_prctl, op.0, addr)
}

// ===== ArchOp =====

/// [`arch_prctl`] operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ArchOp(i32);

impl ArchOp {
    /// `ARCH_SET_GS`
    pub const SET_GS: Self = Self(ARCH_SET_GS);
    /// `ARCH_SET_FS`
    pub const SET_FS: Self = Self(ARCH_SET_FS);
    /// `ARCH_GET_FS`
    pub const GET_FS: Self = Self(ARCH_GET_FS);
    /// `ARCH_GET_GS`
    pub const GET_GS: Self = Self(ARCH_GET_GS);
    /// `ARCH_GET_CPUID`
    pub const GET_CPUID: Self = Self(ARCH_GET_CPUID);
    /// `ARCH_SET_CPUID`
    pub const SET_CPUID: Self = Self(ARCH_SET_CPUID);
}

impl ArchOp {
    /// Set architecture specific state (`arch_prctl(2)`).
    ///
    /// This will call [`arch_prctl`] with `op` of self and given `addr`.
    #[inline]
    pub fn arch_prctl(self, addr: ffi::c_ulong) -> Result<(), sys::Error<arch::sys_arch_prctl>> {
        arch_prctl(self, addr)
    }
}

// ===== extern =====

// arch/x86/include/uapi/asm/prctl.h

const ARCH_SET_GS: i32 = 0x1001;
const ARCH_SET_FS: i32 = 0x1002;
const ARCH_GET_FS: i32 = 0x1003;
const ARCH_GET_GS: i32 = 0x1004;

const ARCH_GET_CPUID: i32 = 0x1011;
const ARCH_SET_CPUID: i32 = 0x1012;

pub(crate) use shared::*;
pub(crate) use arch::*;

// constants, structs
mod shared;

pub(crate) fn optmut<T>(opt: Option<&mut T>) -> *mut T {
    // this will generate to just a `mov`
    opt.map_or(core::ptr::null_mut(), |e|e as *mut _)
}

/// Syscall result.
///
/// Wrapped to prevent missuse.
#[must_use]
pub(crate) struct SysRes(isize);

impl SysRes {
    /// Should only be extracted by helper api.
    pub(crate) fn into_inner(self) -> isize {
        self.0
    }
}

// ===== architecture =====

// default fallback for arch specific definitions
mod asm_generic;

#[cfg_attr(target_arch = "x86_64", path = "x86_64.rs")]
mod arch;

#[cfg(not(target_arch = "x86_64"))]
compile_error!("this architecture is not yet supported");

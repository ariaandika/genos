//! System calls primitives.
pub(crate) use arch::{call, call_impl, call_rd, call_rd_raw};

pub use code::ErrCode;
pub use sysres::{Error, SysId, SysRaw};

mod code;
mod sysres;

// ===== architecture =====

#[cfg_attr(target_arch = "x86_64", path = "x86_64.rs")]
pub mod arch;

#[cfg(not(target_arch = "x86_64"))]
compile_error!("this architecture is not yet supported");

// ===== misc =====

pub(crate) fn optmut<T>(opt: Option<&mut T>) -> *mut T {
    // this will generate to just a `mov`
    opt.map_or(core::ptr::null_mut(), |e|e as *mut _)
}

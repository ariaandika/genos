//! System calls primitives.
pub(crate) use crate::arch::{self, call, call_impl, call_rd, call_rd_raw};

pub use code::ErrCode;
pub use sysres::{Error, SysId, SysRaw};

mod code;
mod sysres;

// ===== misc =====

pub(crate) fn optmut<T>(opt: Option<&mut T>) -> *mut T {
    // this will generate to just a `mov`
    opt.map_or(core::ptr::null_mut(), |e|e as *mut _)
}

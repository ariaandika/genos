use core::ffi::c_void;
use core::mem;
use core::ptr::NonNull;

use crate::sys;

/// Sets the end of the data segment to given value (`brk(2)`).
///
/// Note that this is pure system calls to `brk`, any explanation in `brk(2)` may refer to glibc
/// implementation.
#[inline]
pub fn brk(addr: *mut c_void) -> NonNull<c_void> {
    let res = sys::call_rd_raw!(sys_brk, addr);
    // SAFETY: `sys_brk` never returns zero
    unsafe { mem::transmute(res) }
}

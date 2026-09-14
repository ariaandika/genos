use core::ffi::c_void;
use core::ptr::NonNull;

use crate::mem::OutOfMemory;
use crate::sys;

/// Sets the end of the data segment to given value.
///
/// Returns the new end of data segment.
///
/// Calling [`brk`] with value of `0` returns the current end of the data segment.
#[inline]
pub fn brk(addr: *mut c_void) -> Result<NonNull<c_void>, OutOfMemory> {
    let new = brk_inner(addr);
    if new < addr {
        return Err(OutOfMemory);
    }
    Ok(unsafe { NonNull::new_unchecked(new) })
}

/// Returns the current end of the data segment.
///
/// This is the same as calling [`brk`] with value of `0`.
#[inline]
pub fn current_brk() -> NonNull<c_void> {
    unsafe { NonNull::new_unchecked(brk_inner(0 as _)) }
}

fn brk_inner(addr: *mut c_void) -> *mut c_void {
    sys::call_rd!(sys_brk, addr).into_inner() as _
}

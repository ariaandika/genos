use core::ffi::c_void;
use core::ptr::NonNull;

use crate::sys::{self, Error, arch};

/// Sets the end of the data segment to given value (`brk(2)`).
///
/// Note that this is pure system calls to `brk`, any explanation in `brk(2)` may refer to glibc
/// implementation.
#[inline]
pub fn brk(addr: *mut c_void) -> Result<NonNull<c_void>, Error<arch::sys_brk>> {
    sys::call_rd!(sys_brk, addr)
}

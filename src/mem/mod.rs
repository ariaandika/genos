//! Memory management.
use core::ptr::NonNull;
use core::ffi::c_void;

pub use mmap::Mmap;
pub use error::OutOfMemory;

pub mod mmap;
mod error;

/// Sets the end of the data segment to given value.
///
/// Returns `None` if failed.
///
/// Calling [`brk`] with value of `0` returns the current end of the data segment.
#[inline]
pub fn brk(addr: *mut c_void) -> Option<NonNull<c_void>> {
    let res = crate::sys::call!(RD, __NR_brk, addr).into_inner();
    usize::try_from(res).ok().and_then(|e| NonNull::new(e as _))
}

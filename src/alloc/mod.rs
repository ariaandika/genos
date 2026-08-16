//! Dynamic memory allocation.
use core::ptr;

pub use error::OutOfMemory;

mod error;

/// Allocate `size` bytes and returns a pointer to the allocated memory.
#[inline]
pub fn malloc(size: usize) -> Result<ptr::NonNull<u8>, OutOfMemory> {
    ptr::NonNull::new(unsafe { libc::malloc(size).cast() }).ok_or(OutOfMemory)
}

/// Changes the size of the memory block pointed to by p to size bytes.
///
/// Note if `size` is zero, the memory is freed and returns `Err(OutOfMemory)`.
#[inline]
pub fn realloc(ptr: ptr::NonNull<u8>, size: usize) -> Result<ptr::NonNull<u8>, OutOfMemory> {
    // return NULL if p is not NULL and the requested size is zero;
    ptr::NonNull::new(unsafe { libc::realloc(ptr.as_ptr().cast(), size).cast() }).ok_or(OutOfMemory)
}

/// Allocate `size` bytes and returns a pointer to the allocated memory.
///
/// # Safety
///
/// `ptr` should have not been freed.
#[inline]
pub unsafe fn free(ptr: ptr::NonNull<u8>) {
    unsafe { libc::free(ptr.as_ptr().cast()) };
}

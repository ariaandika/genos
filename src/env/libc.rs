use core::ffi::CStr;
use core::{ffi, ptr};

use crate::error::{self, ErrCode, FromErrCode};

// ===== libc bindings =====

/// Returns environment variable value for given name.
#[inline]
pub fn var(name: &CStr) -> Option<&CStr> {
    unsafe { ptr::NonNull::new(getenv(name.as_ptr())).map(|e| CStr::from_ptr(e.as_ptr())) }
}

/// Adds the variable name to the environment with the given value, if name does not already exist.
///
/// If name does exist in the environment, then its value is changed to value if overwrite is
/// nonzero; if overwrite is zero, then the value of name is not changed. This function makes copies
/// of the strings pointed to by name and value
#[inline]
pub fn set_var(name: &CStr, value: &CStr, overwrite: bool) -> Result<(), Error> {
    <_>::e(unsafe { setenv(name.as_ptr(), value.as_ptr(), overwrite as _) })
}

/// Deletes the variable name from the environment.
///
/// If name does not exist in the environment, then the function succeeds, and the environment is
/// unchanged.
#[inline]
pub fn unset_var(name: &CStr) -> Result<(), Error> {
    <_>::e(unsafe { unsetenv(name.as_ptr()) })
}

/// Clears the environment of all name-value pairs and sets the value of the external variable
/// environ to NULL.
#[inline]
pub fn clear_vars() -> Result<(), Error> {
    <_>::e(unsafe { clearenv() })
}

/// An error that may occur when configuring environment variable.
#[derive(Clone, Copy)]
pub struct Error(ErrCode);

error::impl_error_os_simple!(Error, "configure environment variable");

// ===== extern =====

unsafe extern "C" {
    fn getenv(name: *const ffi::c_char) -> *mut ffi::c_char;
    fn setenv(name: *const ffi::c_char, val: *const ffi::c_char, overwrite: i32) -> i32;
    fn unsetenv(name: *const ffi::c_char) -> i32;
    fn clearenv() -> i32;
}

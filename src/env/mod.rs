//! Environment variables.
use core::ffi::CStr;
use core::ptr;

pub use error::Error;

/// Returns environment variable value for given name.
#[inline]
pub fn var(name: &CStr) -> Option<&CStr> {
    unsafe { ptr::NonNull::new(libc::getenv(name.as_ptr())).map(|e| CStr::from_ptr(e.as_ptr())) }
}

/// Adds the variable name to the environment with the given value, if name does not already exist.
///
/// If name does exist in the environment, then its value is changed to value if overwrite is
/// nonzero; if overwrite is zero, then the value of name is not changed. This function makes copies
/// of the strings pointed to by name and value
#[inline]
pub fn set_var(name: &CStr, value: &CStr, overwrite: bool) -> Result<(), Error> {
    unsafe { Error::from_res(libc::setenv(name.as_ptr(), value.as_ptr(), overwrite as _)) }
}

/// Deletes the variable name from the environment.
///
/// If name does not exist in the environment, then the function succeeds, and the environment is
/// unchanged.
#[inline]
pub fn unset_var(name: &CStr) {
    unsafe { libc::unsetenv(name.as_ptr()) };
}

/// Clears the environment of all name-value pairs and sets the value of the external variable
/// environ to NULL.
#[inline]
pub fn clear_vars() {
    unsafe { libc::clearenv() };
}

mod error;

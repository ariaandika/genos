use crate::ffi::{Char, Off};
use crate::sys::{self, Error, arch};

/// Truncate file to a size of precisely length bytes (`truncate(2)`).
#[inline]
pub fn truncate(path: &Char, length: Off) -> Result<(), Error<arch::sys_truncate>> {
    sys::call_rd!(sys_truncate, path, length)
}

/// Change the name or location of a file (`rename(2)`).
#[inline]
pub fn rename(path: &Char, newpath: &Char) -> Result<(), Error<arch::sys_rename>> {
    sys::call_rd!(sys_rename, path, newpath)
}

/// Make a new name for a file (`link(2)`).
#[inline]
pub fn link(path: &Char, newpath: &Char) -> Result<(), Error<arch::sys_link>> {
    sys::call_rd!(sys_link, path, newpath)
}

/// Make a new name for a file (`symlink(2)`).
#[inline]
pub fn symlink(path: &Char, linkpath: &Char) -> Result<(), Error<arch::sys_symlink>> {
    sys::call_rd!(sys_symlink, path, linkpath)
}

/// Delete a name from the filesystem (`unlink(2)`).
#[inline]
pub fn unlink(path: &Char) -> Result<(), Error<arch::sys_unlink>> {
    sys::call_rd!(sys_unlink, path)
}

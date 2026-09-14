use crate::error::SysResExt;
use crate::ffi::Char;
use crate::fs::file::{Kind, Result};
use crate::io::Offset;
use crate::sys;

/// Truncate file to a size of precisely length bytes (`truncate(2)`).
#[inline]
pub fn truncate(path: &Char, length: Offset) -> Result<()> {
    sys::call_rd!(sys_truncate, path, length).e(Kind::Truncate)
}

/// Change the name or location of a file (`rename(2)`).
#[inline]
pub fn rename(path: &Char, newpath: &Char) -> Result<()> {
    sys::call_rd!(sys_rename, path, newpath).e(Kind::Rename)
}

/// Make a new name for a file (`link(2)`).
#[inline]
pub fn link(path: &Char, newpath: &Char) -> Result<()> {
    sys::call_rd!(sys_link, path, newpath).e(Kind::Link)
}

/// Make a new name for a file (`symlink(2)`).
#[inline]
pub fn symlink(path: &Char, linkpath: &Char) -> Result<()> {
    sys::call_rd!(sys_symlink, path, linkpath).e(Kind::Link)
}

/// Delete a name from the filesystem (`unlink(2)`).
#[inline]
pub fn unlink(path: &Char) -> Result<()> {
    sys::call_rd!(sys_unlink, path).e(Kind::Unlink)
}

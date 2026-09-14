//! [`Path`] associated types.

use core::ffi::CStr;
use core::ptr::NonNull;
use core::{ffi, marker};

use crate::error::SysResExt;
use crate::fs::file::{Kind, Result};
use crate::sys;

/// File path.
#[derive(Debug)]
#[repr(transparent)]
pub struct Path {
    buf: NonNull<ffi::c_char>,
    _p: marker::PhantomData<CStr>,
}

impl Path {
    /// Create new [`Path`].
    #[inline]
    pub const fn new(cstr: &CStr) -> &Self {
        unsafe { &*(cstr.as_ptr() as *const _) }
    }

    /// Returns the underlying pointer.
    #[inline]
    pub const fn as_ptr(&self) -> *mut ffi::c_char {
        self.buf.as_ptr()
    }

    /// Truncate file to a size of precisely length bytes.
    #[inline]
    pub fn truncate(&self, length: sys::off_t) -> Result<()> {
        sys::call_rd!(sys_truncate, self.as_ptr(), length).e(Kind::Rename)
    }

    /// Change the name or location of a file.
    #[inline]
    pub fn rename(&self, newpath: &ffi::CStr) -> Result<()> {
        sys::call_rd!(sys_rename, self.as_ptr(), newpath).e(Kind::Rename)
    }

    /// Make a new name for a file.
    #[inline]
    pub fn link(&self, newpath: &ffi::CStr) -> Result<()> {
        sys::call_rd!(sys_link, self.as_ptr(), newpath).e(Kind::Link)
    }

    /// Make a new name for a file.
    #[inline]
    pub fn symlink(&self, linkpath: &ffi::CStr) -> Result<()> {
        sys::call_rd!(sys_symlink, self.as_ptr(), linkpath).e(Kind::Link)
    }

    /// Delete a name from the filesystem.
    #[inline]
    pub fn unlink(&self) -> Result<()> {
        sys::call_rd!(sys_unlink, self.as_ptr()).e(Kind::Unlink)
    }
}

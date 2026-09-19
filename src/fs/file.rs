//! [`File`] associated types.
use crate::fd::{AsFd, Open, OwnedFd};
use crate::ffi::{Char, Mode, Off};
use crate::sys::SysRes;
use crate::{fd, sys};

/// Open file descriptor.
#[derive(Debug)]
pub struct File(OwnedFd);

fd::impl_fd_simple!(File);

impl File {
    /// Opens specified file (`open(2)`).
    #[inline]
    pub fn open(path: &Char, mode: Open) -> impl SysRes<Self> {
        sys::call_rd!(sys_open, path, mode.raw())
    }

    /// Create file if does not exists, open in write-only mode, and truncate to length 0
    /// (`creat(2)`).
    #[inline]
    pub fn create(path: &Char, mode: Mode) -> impl SysRes<Self> {
        sys::call_rd!(sys_creat, path, mode)
    }

    /// Reposition read/write offset (`lseek(2)`).
    #[inline]
    pub fn seek(&self, offset: Off, seek: Seek) -> impl SysRes<Off> {
        sys::call_rd!(sys_lseek, self.as_raw_fd(), offset, seek.0)
    }

    /// Truncate file to a size of precisely length bytes (`ftruncate(2)`).
    #[inline]
    pub fn truncate(&self, length: Off) -> impl SysRes<()> {
        sys::call_rd!(sys_ftruncate, self.as_raw_fd(), length)
    }
}

// ===== Seek =====

/// File seeking mode (`lseek(2)`).
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Seek(i32);

impl File {
    /// `SEEK_SET`
    pub const SET: Seek = Seek(SEEK_SET);
    /// `SEEK_CUR`
    pub const CUR: Seek = Seek(SEEK_CUR);
    /// `SEEK_END`
    pub const END: Seek = Seek(SEEK_END);
    /// `SEEK_DATA`
    pub const DATA: Seek = Seek(SEEK_DATA);
    /// `SEEK_HOLE`
    pub const HOLE: Seek = Seek(SEEK_HOLE);
}

// ===== extern =====

// include/uapi/linux/fs.h

const SEEK_SET: i32 = 0;
const SEEK_CUR: i32 = 1;
const SEEK_END: i32 = 2;
const SEEK_DATA: i32 = 3;
const SEEK_HOLE: i32 = 4;

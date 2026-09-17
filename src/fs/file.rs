//! [`File`] associated types.
use core::{fmt, result};

use crate::error::{ErrCode, SysResExt};
use crate::fd::{AsFd, Open, OwnedFd};
use crate::ffi::{Char, Mode, Off};
use crate::{error, fd, sys};

/// Open file descriptor.
#[derive(Debug)]
pub struct File(OwnedFd);

fd::impl_fd_simple!(File);

impl File {
    /// Opens specified file (`open(2)`).
    #[inline]
    pub fn open(path: &Char, mode: Open) -> Result<Self> {
        sys::call_rd!(sys_open, path, mode.raw()).fd(Kind::Open)
    }

    /// Create file if does not exists, open in write-only mode, and truncate to length 0
    /// (`creat(2)`).
    #[inline]
    pub fn create(path: &Char, mode: Mode) -> Result<Self> {
        sys::call_rd!(sys_creat, path, mode).fd(Kind::Create)
    }

    /// Reposition read/write offset (`lseek(2)`).
    #[inline]
    pub fn seek(&self, offset: Off, seek: Seek) -> Result<Off> {
        sys::call_rd!(sys_lseek, self.as_raw_fd(), offset, seek.0)
            .io(Kind::Seek)
            .map(|e| e as _)
    }

    /// Truncate file to a size of precisely length bytes (`ftruncate(2)`).
    #[inline]
    pub fn truncate(&self, length: Off) -> Result<()> {
        sys::call_rd!(sys_ftruncate, self.as_raw_fd(), length).e(Kind::Truncate)
    }
}

// ===== Seek =====

/// File seeking mode (`lseek(2)`).
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Seek(i32);

impl Seek {
    /// `SEEK_SET`
    pub const SET: Self = Self(SEEK_SET);
    /// `SEEK_CUR`
    pub const CUR: Self = Self(SEEK_CUR);
    /// `SEEK_END`
    pub const END: Self = Self(SEEK_END);
    /// `SEEK_DATA`
    pub const DATA: Self = Self(SEEK_DATA);
    /// `SEEK_HOLE`
    pub const HOLE: Self = Self(SEEK_HOLE);
}

// ===== Error =====

/// Type alias for result of [`File`] operations.
pub type Result<T> = result::Result<T, Error>;

/// An error that may occur during any [`File`] operations.
#[derive(Debug, Clone)]
pub struct Error {
    kind: Kind,
    code: ErrCode,
}

#[derive(Debug, Clone)]
pub(super) enum Kind {
    Open,
    Create,
    Seek,
    Rename,
    Truncate,
    Link,
    Unlink,
}

error::impl_error_with_kind!(Error, Kind);

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self { kind, code } = self;
        let msg = match kind {
            Kind::Open => "open",
            Kind::Create => "create",
            Kind::Seek => "seek",
            Kind::Rename => "rename",
            Kind::Truncate => "truncate",
            Kind::Link => "link",
            Kind::Unlink => "unlink",
        };
        write!(f, "failed to {msg} file: {code}")
    }
}

// ===== extern =====

// include/uapi/linux/fs.h

const SEEK_SET: i32 = 0;
const SEEK_CUR: i32 = 1;
const SEEK_END: i32 = 2;
const SEEK_DATA: i32 = 3;
const SEEK_HOLE: i32 = 4;

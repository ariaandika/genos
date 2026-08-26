//! [`File`] associated types.

use core::ffi::CStr;

use crate::error::SysResExt;
use crate::fd::{AsFd, OwnedFd};
use crate::fs::{Error, Kind, Result};
use crate::io::{Read, Write};
use crate::{fd, flags, sys};

/// Integer representing file size.
///
/// Reference: `off_t(3type)`.
pub type Off = sys::off_t;

/// Open file.
#[derive(Debug)]
pub struct File(OwnedFd);

fd::impl_fd_simple!(File);

impl File {
    /// Opens specified file.
    ///
    /// Reference: `open(2)`.
    #[inline]
    pub fn open(path: &CStr, mode: AccessMode) -> Result<Self> {
        sys::call!(RD, open, path, mode.0).fd(Kind::Open)
    }

    /// Create file if does not exists, open in write-only mode, and truncate to length 0.
    ///
    /// Reference: `creat(2)`.
    #[inline]
    pub fn create(path: &CStr, mode: sys::mode_t) -> Result<Self> {
        sys::call!(RD, creat, path, mode).fd(Kind::Create)
    }

    /// Reposition read/write offset.
    ///
    /// Reference: `lseek(2)`.
    #[inline]
    pub fn seek(&self, offset: sys::off_t, seek: Seek) -> Result<Off> {
        sys::call!(RD, lseek, self.as_fd(), offset, seek.0)
            .io(Kind::Seek)
            .map(|e| e as _)
    }

    /// Truncate file to a size of precisely length bytes.
    ///
    /// Reference: `ftruncate(2)`.
    #[inline]
    pub fn truncate(&self, length: Off) -> Result<()> {
        sys::call!(RD, ftruncate, self.as_fd(), length).e(Kind::Truncate)
    }
}

impl Read for File {
    type Error = Error;
}

impl Write for File {
    type Error = Error;
}

// ===== AccessMode =====

/// File accessing mode.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct AccessMode(i32);

flags::impl_bitops_simple!(AccessMode, CreateFlags);
flags::impl_bitops_simple!(AccessMode, StatusFlags);

impl AccessMode {
    /// Open file in read-only mode.
    pub const RDONLY: Self = Self(sys::O_RDONLY);
    /// Open file in write-only mode.
    pub const WRONLY: Self = Self(sys::O_WRONLY);
    /// Open file in read/write mode.
    pub const RDWR: Self = Self(sys::O_RDWR);
}

// ===== CreateFlags =====

/// File creation flags.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct CreateFlags(i32);

flags::impl_bitops_simple!(CreateFlags);
flags::impl_bitops_simple!(CreateFlags, AccessMode);
flags::impl_bitops_simple!(CreateFlags, StatusFlags);

impl CreateFlags {
    /// If path does not exist, create it as a regular file.
    pub const CREAT: Self = Self(sys::O_CREAT);
    /// Enable the close-on-exec flag for the new file descriptor.
    pub const CLOEXEC: Self = Self(sys::O_CLOEXEC);
    /// If path is not a directory, cause [`File::open`] to fail.
    pub const DIRECTORY: Self = Self(sys::O_DIRECTORY);
    /// Ensure that this call creates the file: if this flag is specified in conjunction with
    /// [`CreateFlags::CREAT`], and path already exists, then [`File::open`] fails with the error
    /// `EEXIST`.
    pub const EXCL: Self = Self(sys::O_EXCL);
    /// If path refers to a terminal device it will not become the process's controlling terminal
    /// even if the process does not have one.
    pub const NOCTTY: Self = Self(sys::O_NOCTTY);
    /// If the trailing component of path is a symbolic link, then the open fails, with the error
    /// `ELOOP`.
    pub const NOFOLLOW: Self = Self(sys::O_NOFOLLOW);
    /// Create an unnamed temporary regular file.
    pub const TMPFILE: Self = Self(sys::O_TMPFILE);
    /// If the file already exists and is a regular file and the access mode allows writing it will
    /// be truncated to length 0.
    pub const TRUNC: Self = Self(sys::O_TRUNC);
}

// ===== StatusFlags =====

/// File status flags.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct StatusFlags(i32);

flags::impl_bitops_simple!(StatusFlags);

impl StatusFlags {
    /// The file is opened in append mode.
    ///
    /// Before each `write(2)`, the file offset is positioned at the end of the file, as if with
    /// `lseek(2)`.
    pub const APPEND: Self = Self(sys::O_APPEND);
    /// Enable signal-driven I/O: generate a signal (SIGIO by default) when input or output becomes
    /// possible on this file descriptor.
    pub const ASYNC: Self = Self(sys::O_ASYNC);
    /// Try to minimize cache effects of the I/O to and from this file.
    pub const DIRECT: Self = Self(sys::O_DIRECT);
    /// Write operations on the file will complete according to the requirements of synchronized I/O
    /// data integrity completion.
    pub const DSYNC: Self = Self(sys::O_DSYNC);
    /// (LFS) Allow files whose sizes cannot be represented in an `off_t` (but can be represented in
    /// an `off64_t`) to be opened.
    pub const LARGEFILE: Self = Self(sys::O_LARGEFILE);
    /// Do not update the file last access time when the file is `read(2)`.
    pub const NOATIME: Self = Self(sys::O_NOATIME);
    /// Write operations on the file will complete according to the requirements of synchronized I/O
    /// file integrity completion.
    pub const SYNC: Self = Self(sys::O_SYNC);
}

// ===== Seek =====

/// File seeking mode.
///
/// Reference: `lseek(2)`.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Seek(i32);

impl Seek {
    /// Set file offset to `offset` bytes.
    pub const SET: Self = Self(sys::SEEK_SET);
    /// Set file offset to current location plus offset bytes.
    pub const CUR: Self = Self(sys::SEEK_CUR);
    /// Set file offset to the size of the file plus offset bytes.
    pub const END: Self = Self(sys::SEEK_END);
    /// Adjust the file offset to the next location in the file greater than or equal to offset
    /// containing data.
    pub const DATA: Self = Self(sys::SEEK_DATA);
    /// Adjust the file offset to the next hole in the file greater than or equal to offset.
    pub const HOLE: Self = Self(sys::SEEK_HOLE);
}

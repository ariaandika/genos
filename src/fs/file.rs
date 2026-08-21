//! [`File`] associated types.

use core::ffi::CStr;
use core::{fmt, result};

use crate::error::{ErrCode, SysResExt};
use crate::fd::OwnedFd;
use crate::io::{Read, ReadError, Write, WriteError};
use crate::{error, fd, flags, sys};

/// File handle.
#[derive(Debug)]
pub struct File(OwnedFd);

fd::impl_fd_simple!(File);

impl File {
    /// Opens specified file.
    #[inline]
    pub fn open(path: &CStr, mode: AccessMode) -> Result<Self> {
        sys::call!(__NR_open, path.as_ptr(), mode.0).fd(Kind::Open)
    }

    /// Create file if does not exists, open in write-only mode, and truncate to length 0.
    #[inline]
    pub fn create(path: &CStr, mode: sys::mode_t) -> Result<Self> {
        sys::call!(__NR_creat, path.as_ptr(), mode).fd(Kind::Create)
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

// ===== errors =====

/// Type alias for result of [`File`] operations.
pub type Result<T> = result::Result<T, Error>;

/// An error that may occur during any [`File`] operations.
#[derive(Debug, Clone)]
pub struct Error {
    kind: Kind,
    code: ErrCode,
}

#[derive(Debug, Clone)]
enum Kind {
    Open,
    Create,
    Read,
    Write,
}

error::impl_error_with_kind!(Error, Kind);

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self { kind, code } = self;
        let msg = match kind {
            Kind::Open => "open file",
            Kind::Create => "create file",
            Kind::Read => "read file",
            Kind::Write => "write file",
        };
        write!(f, "failed to {msg}: {code}")
    }
}

impl From<ReadError> for Error {
    #[inline]
    fn from(value: ReadError) -> Self {
        Self { kind: Kind::Read, code: value.into() }
    }
}

impl From<WriteError> for Error {
    #[inline]
    fn from(value: WriteError) -> Self {
        Self { kind: Kind::Write, code: value.into() }
    }
}

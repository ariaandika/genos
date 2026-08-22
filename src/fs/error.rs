use core::{fmt, result};

use crate::error;
use crate::error::ErrCode;
use crate::io::{ReadError, WriteError};

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
pub(super) enum Kind {
    Open,
    Create,
    Read,
    Write,
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
            Kind::Read => "read",
            Kind::Write => "write",
            Kind::Rename => "rename",
            Kind::Truncate => "truncate",
            Kind::Link => "link",
            Kind::Unlink => "unlink",
        };
        write!(f, "failed to {msg} file: {code}")
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

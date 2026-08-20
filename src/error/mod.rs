//! Error types and traits.

// ===== reexports =====

pub use core::error::Error;
pub use self::code::ErrCode;
pub use self::ext::{AsErrCode, ResultExt};

// # Private
//
// in this crate, there is 2 kind of error:
// - os error, simple error containing error code
// - composite error, error containing error code and kind
//
// # OS Error
//
// - `struct Error(ErrCode)`
// - implements `FromErrCode`
// - uses `impl_error_os_simple` for declaration
//
// # Composite Error
//
// - `struct Error { kind: Kind, code: ErrCode }`
// - `Kind` implement `ErrorKind`
// - uses `impl_error_with_kind` for declaration
//
// # `trait SysResExt`
//
// Converts syscall raw result into `Result` using either of the two error kind.

pub(crate) use self::errno::{FromErrCode, impl_error_os_simple};
pub(crate) use self::kind::{ErrorKind, impl_error_with_kind};
pub(crate) use self::sys::SysResExt;

// ===== modules =====

// public
mod code;
mod ext;

// private
mod errno;
mod kind;
mod sys;

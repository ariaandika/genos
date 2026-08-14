//! Error types.
use core::ffi::CStr;
use core::num::NonZeroU8;
use core::{error, fmt};

// ===== traits =====

/// Error type that can be created from error code.
pub(crate) trait FromErrCode: Sized {
    /// Creates error with given error code.
    fn from_err_code(code: ErrCode) -> Self;

    /// Creates error with value from `errno`.
    #[inline]
    fn errno() -> Self {
        Self::from_err_code(ErrCode::errno())
    }

    /// Returns `Err` if `res` is -1.
    #[inline]
    fn e<T: FromOkCode>(res: i32) -> Result<T, Self> {
        if res == -1 {
            return Err(Self::errno());
        }
        Ok(T::from_ok_code(res))
    }
}

pub(crate) trait FromOkCode {
    fn from_ok_code(code: i32) -> Self;
}

impl FromOkCode for () {
    fn from_ok_code(_: i32) -> Self {}
}

// ===== ErrCode =====

/// Error Code.
#[derive(Debug, Clone, Copy)]
pub struct ErrCode(NonZeroU8);

impl ErrCode {
    /// Creates [`ErrCode`] with given error code.
    #[inline]
    pub fn new(code: i32) -> Self {
        Self(NonZeroU8::new(code as _).unwrap_or(NonZeroU8::MAX))
    }

    /// Creates [`ErrCode`] with value retrieved from `errno`.
    #[inline]
    pub fn errno() -> Self {
        Self::new(Self::raw_errno())
    }

    /// Returns raw error code from `errno`.
    #[inline]
    pub fn raw_errno() -> i32 {
        unsafe { *libc::__errno_location() }
    }

    /// Returns the contained raw error code.
    #[inline]
    pub fn code(self) -> i32 {
        self.0.get() as i32
    }

    /// Returns `true` if error code is `EINTR`.
    #[inline]
    pub fn is_interrupt(self) -> bool {
        matches!(self.code(), libc::EINTR)
    }

    /// Returns `true` if error code is `EWOULDBLOCK` or `EAGAIN`.
    #[inline]
    pub fn would_block(self) -> bool {
        matches!(self.code(), libc::EWOULDBLOCK)
    }
}

// ===== core traits =====

impl error::Error for ErrCode {}

impl fmt::Display for ErrCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let code = Self::code(*self);
        let mut buf = [0u8; 128];
        let res = unsafe { libc::strerror_r(code, buf.as_mut_ptr().cast(), buf.len()) };
        let msg = if res >= 0 {
            CStr::from_bytes_until_nul(&buf[..])
                .unwrap_or(c"unknown")
                .to_string_lossy()
        } else {
            "unknown".into()
        };
        write!(f, "{msg} (os error {code})",)
    }
}

// ===== macros =====

/// implements `FromErrCode`, `Display`, `Debug`, `Error`, `{From,Into}<ErrCode>`.
macro_rules! os_error_simple {
    ($me:ident, $c:expr) => {
        const _: () = {
            use core::fmt;
            use crate::error::{ErrCode, FromErrCode};
            impl FromErrCode for $me {
                #[inline]
                fn from_err_code(code: ErrCode) -> Self {
                    Self(code)
                }
            }
            impl fmt::Display for $me {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    write!(f, "failed to {}: {}", $c, self.0)
                }
            }
            impl fmt::Debug for $me {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    f.debug_tuple(stringify!($me)).field(&self.0).finish()
                }
            }
            impl core::error::Error for $me {}
            impl From<ErrCode> for $me {
                #[inline]
                fn from(v: ErrCode) -> Self {
                    Self(v)
                }
            }
            impl From<$me> for ErrCode {
                #[inline]
                fn from(v: $me) -> Self {
                    v.0
                }
            }
        };
    };
}
pub(crate) use os_error_simple;

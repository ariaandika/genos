//! Error types.
use core::ffi::{CStr, c_char};
use core::{error, fmt};
use core::num::NonZeroU8;

// ===== traits =====

/// Error type that can be created from error code.
pub trait FromErrCode: Sized {
    /// Creates error with given error code.
    fn from_err_code(code: ErrCode) -> Self;

    /// Creates error with value from `errno`.
    #[inline]
    fn errno() -> Self {
        Self::from_err_code(ErrCode::errno())
    }
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
        unsafe { *__errno_location() }
    }

    /// Returns the contained raw error code.
    #[inline]
    pub fn code(self) -> i32 {
        self.0.get() as i32
    }

    /// Returns `true` if error code is `EINTR`.
    #[inline]
    pub fn is_interrupt(self) -> bool {
        matches!(self.code(), EINTR)
    }

    /// Returns `true` if error code is `EWOULDBLOCK` or `EAGAIN`.
    #[inline]
    pub fn would_block(self) -> bool {
        matches!(self.code(), EWOULDBLOCK)
    }
}

// ===== core traits =====

impl error::Error for ErrCode { }

impl fmt::Display for ErrCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let code = self.code();
        let mut buf = [0u8; 128];
        let res = unsafe { strerror_r(code, buf.as_mut_ptr().cast(), buf.len()) };
        write!(
            f,
            "{} (os error {code})",
            if res >= 0 {
                CStr::from_bytes_until_nul(&buf[..])
                    .unwrap_or(c"unknown")
                    .to_string_lossy()
            } else {
                "unknown".into()
            }
        )
    }
}

// ===== extern =====

// error.h
const EINTR: i32 = 4;
const EAGAIN: i32 = 11;
const EWOULDBLOCK: i32 = EAGAIN;

unsafe extern "C" {
    fn __errno_location() -> *mut i32;
    #[cfg_attr(
        not(any(target_env = "musl", target_env = "ohos")),
        link_name = "__xpg_strerror_r"
    )]
    fn strerror_r(code: i32, buf: *mut c_char, len: usize) -> i32;
}

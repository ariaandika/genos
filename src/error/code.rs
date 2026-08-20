use core::error::Error;
use core::num::NonZeroU8;
use core::{ffi, fmt};

// ===== ErrCode =====

/// Error Code.
#[derive(Debug, Clone, Copy)]
pub struct ErrCode(NonZeroU8);

impl ErrCode {
    /// Creates [`ErrCode`] with code for invalid arguments.
    pub const EINVAL: Self = Self(NonZeroU8::new(EINVAL as _).unwrap());

    pub(crate) const ENOMEM: Self = Self(NonZeroU8::new(ENOMEM as _).unwrap());

    /// Creates [`ErrCode`] with given error code.
    #[inline]
    pub fn new(code: i32) -> Self {
        Self(NonZeroU8::new(code as _).unwrap_or(NonZeroU8::MAX))
    }

    /// Extract error code from syscall result.
    pub(super) fn sys(code: isize) -> Self {
        Self(NonZeroU8::new((code as u8).wrapping_neg()).unwrap_or(NonZeroU8::MAX))
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

impl Error for ErrCode {}

impl fmt::Display for ErrCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let code = Self::code(*self);
        let mut buf = [0u8; 128];
        let res = unsafe { strerror_r(code, buf.as_mut_ptr().cast(), buf.len()) };
        let msg = if res >= 0 {
            format_args!(
                "{}",
                fmt::from_fn(|f| {
                    fmt_lossy(ffi::CStr::from_bytes_until_nul(&buf[..]).unwrap_or(c"unknown"), f)
                })
            )
        } else {
            format_args!("unknown")
        };
        write!(f, "{msg} (os error {code})",)
    }
}

fn fmt_lossy(cstr: &ffi::CStr, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    for chunk in cstr.to_bytes().utf8_chunks() {
        for c in chunk.valid().chars() {
            match c {
                '\0' => write!(f, "\\0")?,
                '\x01'..='\x7f' => write!(f, "{}", (c as u8).escape_ascii())?,
                _ => write!(f, "{}", c.escape_debug())?,
            }
        }
        write!(f, "{}", chunk.invalid().escape_ascii())?;
    }
    Ok(())
}

// ===== extern =====

// source: include/uapi/asm-generic/errno-base.h

const EINTR: i32 = 4; /* Interrupted system call */
const EAGAIN: i32 = 11; /* Try again */
const ENOMEM: i32 = 12; /* Out of memory */
const EINVAL: i32 = 22; /* Invalid argument */

// source: include/uapi/asm-generic/errno.h

const EWOULDBLOCK: i32 = EAGAIN; /* Operation would block */

unsafe extern "C" {
    #[link_name = "__xpg_strerror_r"]
    fn strerror_r(errno: i32, buf: *mut ffi::c_char, len: usize) -> i32;
    fn __errno_location() -> *mut i32;
}

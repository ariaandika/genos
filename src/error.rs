//! Error types.
pub use core::error::Error;
use core::num::NonZeroU8;
use core::task::Poll;
use core::{ffi, fmt, ptr};

use crate::fd::FromRawFd;
use crate::sys::SysRes;

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
        Ok(T::from_ok_code(res as _))
    }
}

pub(crate) trait FromOkCode {
    fn from_ok_code(code: usize) -> Self;
}

impl FromOkCode for usize {
    fn from_ok_code(val: usize) -> Self {
        val
    }
}

impl FromOkCode for () {
    fn from_ok_code(_: usize) -> Self {}
}

/// Descriptor trait for error pattern that contains error kind and code.
pub(crate) trait ErrorKind {
    type Error;

    fn into_error(self, code: ErrCode) -> Self::Error;
}

/// Extensions trait for converting syscall result.
pub(crate) trait SysResExt: Sized {
    fn into_inner(self) -> isize;

    #[doc(hidden)]
    fn inner(self) -> Result<usize, ErrCode> {
        let r = self.into_inner();
        usize::try_from(r).map_err(|_| ErrCode::sys(r))
    }

    /// Creates file descriptor from success result.
    fn fd<T: FromRawFd, E: ErrorKind>(self, kind: E) -> Result<T, E::Error> {
        match self.inner() {
            Ok(ok) => Ok(unsafe { T::from_raw_fd(ok as _) }),
            Err(err) => Err(kind.into_error(err)),
        }
    }

    /// Checks for error.
    fn e<E: ErrorKind>(self, kind: E) -> Result<(), E::Error> {
        match self.inner() {
            Ok(_) => Ok(()),
            Err(err) => Err(kind.into_error(err)),
        }
    }

    /// Checks for error, but use error implementing [`FromErrCode`].
    fn e2<E: FromErrCode>(self) -> Result<(), E> {
        match self.inner() {
            Ok(_) => Ok(()),
            Err(err) => Err(E::from_err_code(err)),
        }
    }

    /// Returns unsigned integer from success result.
    fn io<E: ErrorKind>(self, kind: E) -> Result<usize, E::Error> {
        self.inner().map_err(|e| kind.into_error(e))
    }

    /// Returns unsigned integer from success result.
    ///
    /// In contrast with [`SysResExt::io`], this uses error implementing [`FromErrCode`].
    fn io2<E: FromErrCode>(self) -> Result<usize, E> {
        self.inner().map_err(E::from_err_code)
    }

    /// Returns non-null pointer from success result.
    fn p2<T, E: FromErrCode>(self) -> Result<ptr::NonNull<T>, E> {
        let res = self.into_inner();
        if res > 0 {
            // SAFETY: `res > 0` means `res != 0`
            Ok(unsafe { ptr::NonNull::new_unchecked(res as *mut T) })
        } else {
            Err(E::from_err_code(ErrCode::sys(res)))
        }
    }
}

impl SysResExt for SysRes {
    fn into_inner(self) -> isize {
        SysRes::into_inner(self)
    }
}

// ===== AsErrCode =====

/// An error that is associated with [`ErrCode`].
pub trait AsErrCode: Sized {
    /// Returns the contained [`ErrCode`].
    fn as_err_code(&self) -> ErrCode;

    /// Returns `true` if the contained error code is `EINTR`.
    #[inline]
    fn is_interrupt(&self) -> bool {
        self.as_err_code().is_interrupt()
    }

    /// Returns `true` if the contained error code is `EWOULDBLOCK`.
    #[inline]
    fn would_block(&self) -> bool {
        self.as_err_code().would_block()
    }

    /// Returns `Ok(v)` if error is `EINTR`.
    #[inline]
    fn nointr<T>(self, v: T) -> Result<T, Self> {
        if self.is_interrupt() { Ok(v) } else { Err(self) }
    }

    /// Returns `Ok(f())` if error is `EINTR`.
    #[inline]
    fn nointr_with<T, F: FnOnce() -> T>(self, f: F) -> Result<T, Self> {
        if self.is_interrupt() { Ok(f()) } else { Err(self) }
    }
}

// ===== ResultExt =====

/// Extension trait for [`Result`] to work with error containing [`ErrCode`].
pub trait ResultExt<T, E>: Sized
where
    E: AsErrCode,
{
    /// Convert self to `Result<T, E>`.
    fn into_result(self) -> Result<T, E>;

    /// Returns `Ok(Some(ok))` if success, and `Ok(None)` if error is `EINTR`.
    #[inline]
    fn nointr(self) -> Result<Option<T>, E> {
        match self.into_result() {
            Ok(ok) => Ok(Some(ok)),
            Err(err) => err.nointr(None),
        }
    }

    /// Returns `Ok(Some(ok))` if success, and `Ok(None)` if error is `EINTR`.
    #[inline]
    fn nointr_with_default(self) -> Result<T, E>
    where
        T: Default,
    {
        match self.into_result() {
            Ok(ok) => Ok(ok),
            Err(err) => err.nointr_with(T::default),
        }
    }

    /// Returns `Poll::Pending` if error is `EWOULDBLOCK`.
    #[inline]
    fn noblock(self) -> Poll<Result<T, E>> {
        match self.into_result() {
            Ok(ok) => Poll::Ready(Ok(ok)),
            Err(err) => {
                if err.would_block() {
                    Poll::Pending
                } else {
                    Poll::Ready(Err(err))
                }
            }
        }
    }
}

impl<T, E> ResultExt<T, E> for Result<T, E>
where
    E: AsErrCode,
{
    #[inline]
    fn into_result(self) -> Result<T, E> {
        self
    }
}

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
    fn sys(code: isize) -> Self {
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

// ===== macros =====

/// implements `FromErrCode`, `Display`, `Debug`, `Error`, `{From,Into}<ErrCode>`.
macro_rules! os_error_simple {
    ($me:ident, $c:expr) => {
        const _: () = {
            use core::fmt;
            use crate::error::{AsErrCode, ErrCode, FromErrCode};
            impl FromErrCode for $me {
                #[inline]
                fn from_err_code(code: ErrCode) -> Self {
                    Self(code)
                }
            }
            impl AsErrCode for $me {
                #[inline]
                fn as_err_code(&self) -> ErrCode {
                    self.0
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
#[allow(unused_imports)]
pub(crate) use os_error_simple as impl_error_os_simple;

/// Helper macro to declare error pattern that contains error kind and code.
macro_rules! impl_error_with_kind {
    ($err:ident, $kind:ident) => {
        impl crate::error::Error for $err {}
        impl crate::error::ErrorKind for $kind {
            type Error = $err;

            fn into_error(self, code: crate::error::ErrCode) -> Self::Error {
                $err { kind: self, code }
            }
        }
        impl From<$err> for crate::error::ErrCode {
            #[inline]
            fn from(value: $err) -> Self {
                value.code
            }
        }
        impl crate::error::AsErrCode for $err {
            #[inline]
            fn as_err_code(&self) -> crate::error::ErrCode {
                self.code
            }
        }
    };
}
pub(crate) use impl_error_with_kind;

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

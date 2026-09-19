use core::marker::PhantomData;
use core::{ffi, num, ptr};

use crate::error::ErrCode;
use crate::fd::FromRawFd;

/// Raw systemcall return value.
pub type SysRaw = ffi::c_long;

// ===== SysRes =====

/// System call result.
pub trait SysRes<T>: sealed::SealedRes<T> + Sized {
    /// Returns the raw syscall return value.
    #[inline]
    fn into_raw(self) -> SysRaw {
        self.raw()
    }

    /// Panic
    #[inline]
    #[expect(clippy::panic, reason = "this will cover all panic use cases")]
    fn unwrap(self) -> T {
        let raw = self.raw();
        if raw >= 0 {
            Self::ok_from_raw(raw)
        } else {
            panic!("`{}` syscall failed: {}", Self::NAME, ErrCode::sys(raw as _))
        }
    }

    /// Checks for error and returns [`Result<T, ErrCode>`].
    #[inline]
    fn rescode(self) -> Result<T, ErrCode> {
        let raw = self.raw();
        if raw >= 0 { Ok(Self::ok_from_raw(raw)) } else { Err(ErrCode::sys(raw as _)) }
    }

    /// Checks for error and returns [`Result<T, E>`].
    ///
    /// The custom error must implement [`FromSysErr`].
    #[inline]
    fn result<E: FromSysErr<Self::Sysno>>(self) -> Result<T, E> {
        let raw = self.raw();
        if raw >= 0 {
            Ok(Self::ok_from_raw(raw))
        } else {
            Err(E::from_syserr(ErrCode::sys(raw as _)))
        }
    }
}

// ===== SysErr =====

/// Systemcall number.
pub trait SysErr {
    /// Systemcall name.
    const NAME: &str;
}

/// An error that can be created from [`SysErr`] and [`ErrCode`].
pub trait FromSysErr<S: SysErr> {
    /// Create self from [`SysErr`] and [`ErrCode`].
    fn from_syserr(code: ErrCode) -> Self;
}

// ===== SysOk =====

/// Systemcall number.
pub(crate) trait SysOk {
    fn from_raw(raw: SysRaw) -> Self;
}

impl SysOk for () {
    #[inline]
    fn from_raw(_: SysRaw) {}
}

impl SysOk for i64 {
    #[inline]
    fn from_raw(raw: SysRaw) -> Self {
        raw
    }
}

impl SysOk for usize {
    #[inline]
    fn from_raw(raw: SysRaw) -> Self {
        raw as usize
    }
}

impl<T> SysOk for ptr::NonNull<T> {
    #[inline]
    fn from_raw(raw: SysRaw) -> Self {
        unsafe { ptr::NonNull::without_provenance(num::NonZeroUsize::new_unchecked(raw as usize)) }
    }
}

impl<T: FromRawFd> SysOk for T {
    #[inline]
    fn from_raw(raw: SysRaw) -> Self {
        unsafe { T::from_raw_fd(raw as i32) }
    }
}

// ===== sealed =====

mod sealed {
    use super::SysRaw;
    pub trait SealedRes<T> {
        type Sysno: super::SysErr;
        const NAME: &str;
        fn raw(self) -> SysRaw;
        fn ok_from_raw(raw: SysRaw) -> T;
    }
}

// ===== SysResRaw =====

/// System call result.
#[derive(Debug)]
#[repr(transparent)]
#[must_use]
pub struct SysResRaw<T, E> {
    raw: SysRaw,
    _t: PhantomData<T>,
    _e: PhantomData<fn() -> E>,
}

impl<T: SysOk, E: SysErr> SysRes<T> for SysResRaw<T, E> {}
impl<T: SysOk, E: SysErr> sealed::SealedRes<T> for SysResRaw<T, E> {
    type Sysno = E;
    const NAME: &str = E::NAME;

    #[inline]
    fn raw(self) -> SysRaw {
        self.raw
    }

    #[inline]
    fn ok_from_raw(raw: SysRaw) -> T {
        T::from_raw(raw)
    }
}

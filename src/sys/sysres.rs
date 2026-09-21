use core::marker::PhantomData;
use core::{error, ffi, fmt, num, ptr};

use crate::fd::FromRawFd;
use crate::sys::ErrCode;

// ===== SysRaw =====

/// Raw systemcall return value.
pub type SysRaw = ffi::c_long;

// ===== SysId =====

/// Systemcall identifier.
pub trait SysId {
    /// Systemcall number.
    const NO: SysRaw;
    /// Systemcall name.
    const NAME: &str;
}

// ===== Error =====

/// System call [`Error`] implementation.
///
/// [`Error`]: core::error::Error
pub struct Error<Id> {
    code: ErrCode,
    _id: PhantomData<Id>,
}

impl<Id> Error<Id> {
    pub(crate) fn from_raw<T>(raw: SysRaw) -> Result<T, Self>
    where
        T: SysOk,
    {
        match ErrCode::from_sys(raw as _) {
            None => Ok(T::from_raw(raw)),
            Some(code) => Err(Self { code, _id: PhantomData }),
        }
    }

    pub(crate) unsafe fn from_infallible(raw: SysRaw) -> Self {
        Self { code: unsafe { ErrCode::new(raw as _) }, _id: PhantomData }
    }

    /// Returns the [`SysId::NAME`].
    #[inline]
    pub const fn syscall_name(&self) -> &'static str
    where
        Id: SysId,
    {
        Id::NAME
    }

    /// Returns the [`ErrCode`].
    #[inline]
    pub const fn code(&self) -> ErrCode {
        self.code
    }
}

impl<Id: SysId> error::Error for Error<Id> {}

impl<Id: SysId> fmt::Debug for Error<Id> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Error")
            .field("code", &self.code)
            .field("syscall", &Id::NAME)
            .finish()
    }
}

impl<Id: SysId> fmt::Display for Error<Id> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "`{}` returns -{}", Id::NAME, self.code.code())
    }
}

// ===== SysOk =====

/// Systemcall number.
pub trait SysOk {
    fn from_raw(raw: SysRaw) -> Self;
}

impl SysOk for () {
    #[inline]
    fn from_raw(_: SysRaw) {}
}

impl SysOk for u64 {
    #[inline]
    fn from_raw(raw: SysRaw) -> Self {
        raw as u64
    }
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

use core::task::Poll;

use crate::error::ErrCode;

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

impl AsErrCode for ErrCode {
    #[inline]
    fn as_err_code(&self) -> ErrCode {
        *self
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

    /// Returns `Ok(f())` if success, and `Ok(None)` if error is `EINTR`.
    #[inline]
    fn nointr_with<F: FnOnce() -> T>(self, f: F) -> Result<T, E> {
        self.into_result().or_else(|e| e.nointr_with(f))
    }

    /// Returns `Ok(Some(ok))` if success, and `Ok(None)` if error is `EINTR`.
    #[inline]
    fn nointr_with_default(self) -> Result<T, E>
    where
        T: Default,
    {
        self.into_result().or_else(|e| e.nointr_with(T::default))
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

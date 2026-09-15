use crate::error::{ErrCode, ErrorKind, FromErrCode};
use crate::fd::FromRawFd;
use crate::sys::SysRes;

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
}

impl SysResExt for SysRes {
    fn into_inner(self) -> isize {
        SysRes::into_inner(self)
    }
}

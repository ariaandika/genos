use crate::error::ErrCode;

// ===== FromErrCode =====

/// Error type that can be created from error code.
pub(crate) trait FromErrCode: Sized {
    /// Creates error with given error code.
    fn from_err_code(code: ErrCode) -> Self;
}

// ===== macros =====

macro_rules! impl_error_os_simple {
    ($me:ident, $c:expr) => {
        const _: () = {
            use crate::error::ErrCode;
            impl crate::error::Error for $me {}
            impl crate::error::FromErrCode for $me {
                #[inline]
                fn from_err_code(code: ErrCode) -> Self {
                    Self(code)
                }
            }
            impl crate::error::AsErrCode for $me {
                #[inline]
                fn as_err_code(&self) -> ErrCode {
                    self.0
                }
            }
            impl From<$me> for ErrCode {
                #[inline]
                fn from(v: $me) -> Self {
                    v.0
                }
            }
            impl core::fmt::Display for $me {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    write!(f, "failed to {}: {}", $c, self.0)
                }
            }
            impl core::fmt::Debug for $me {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    f.debug_tuple(stringify!($me)).field(&self.0).finish()
                }
            }
        };
    };
}
pub(crate) use impl_error_os_simple;

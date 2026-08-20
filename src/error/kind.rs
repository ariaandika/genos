use crate::error::ErrCode;

// ===== ErrorKind =====

/// Descriptor trait for error pattern that contains error kind and code.
pub(crate) trait ErrorKind {
    type Error;

    fn into_error(self, code: ErrCode) -> Self::Error;
}

// ===== macros =====

/// Helper macro to declare error pattern that contains error kind and code.
///
/// This trait does not implements `Display`, caller must implement it manually.
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

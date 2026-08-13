//! Bit flags helper.

/// Common bitflag values for opening file descriptor.
pub trait OpenFlag {
    /// Enable the close-on-exec flag for the new file descriptor.
    ///
    /// If the associated API does not support this, the value is empty.
    const CLOEXEC: Self;

    /// Open the file in non-blocking mode.
    ///
    /// If the associated API does not support this, the value is empty.
    const NONBLOCK: Self;
}

macro_rules! impl_bitops_simple {
    ($me:ident) => {
        crate::flags::impl_bitops_simple!($me, Self);
    };
    ($me:ident, $rhs:ident) => {
        impl std::ops::BitOr<$rhs> for Flags {
            type Output = Self;
            #[inline]
            fn bitor(self, rhs: Self) -> Self::Output { Self(self.0.bitor(rhs.0)) }
        }
        impl std::ops::BitAnd for Flags {
            type Output = Self;
            #[inline]
            fn bitand(self, rhs: Self) -> Self::Output { Self(self.0.bitand(rhs.0)) }
        }
        impl std::ops::BitXor for Flags {
            type Output = Self;
            #[inline]
            fn bitxor(self, rhs: Self) -> Self::Output { Self(self.0.bitxor(rhs.0)) }
        }
    };
}
pub(crate) use impl_bitops_simple;

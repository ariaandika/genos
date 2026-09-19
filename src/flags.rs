//! Bit flags helper.

macro_rules! impl_bitops_simple {
    ($me:ident) => {
        crate::flags::impl_bitops_simple!($me, Self, Output = Self);
    };
    ($me:ident, $rhs:ident) => {
        crate::flags::impl_bitops_simple!($me, $rhs, Output = Self);
    };
    ($me:ident, $rhs:ident, Output = $o:ident) => {
        impl core::ops::BitOr<$rhs> for $me {
            type Output = $o;
            #[inline]
            fn bitor(self, rhs: $rhs) -> Self::Output {
                $o(self.0.bitor(rhs.0))
            }
        }
        impl core::ops::BitAnd<$rhs> for $me {
            type Output = $o;
            #[inline]
            fn bitand(self, rhs: $rhs) -> Self::Output {
                $o(self.0.bitand(rhs.0))
            }
        }
        impl core::ops::BitXor<$rhs> for $me {
            type Output = $o;
            #[inline]
            fn bitxor(self, rhs: $rhs) -> Self::Output {
                $o(self.0.bitxor(rhs.0))
            }
        }
    };
}
pub(crate) use impl_bitops_simple;

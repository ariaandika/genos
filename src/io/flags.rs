use crate::{flags, sys};

// ===== RWFlags =====

/// [`preadv`] and [`pwritev`] flags.
///
/// [`preadv`]: crate::io::preadv
/// [`pwritev`]: crate::io::pwritev
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct RWFlags(i32);

flags::impl_bitops_simple!(RWFlags);

impl RWFlags {
    /// `RWF_HIPRI`
    pub const HIPRI: Self = Self(sys::RWF_HIPRI);
    /// `RWF_NOWAIT`
    pub const NOWAIT: Self = Self(sys::RWF_NOWAIT);
    /// `RWF_DONTCACHE`
    pub const DONTCACHE: Self = Self(sys::RWF_DONTCACHE);
    /// `RWF_DSYNC`
    pub const DSYNC: Self = Self(sys::RWF_DSYNC);
    /// `RWF_SYNC`
    pub const SYNC: Self = Self(sys::RWF_SYNC);
    /// `RWF_APPEND`
    pub const APPEND: Self = Self(sys::RWF_APPEND);
    /// `RWF_NOAPPEND`
    pub const NOAPPEND: Self = Self(sys::RWF_NOAPPEND);
    /// `RWF_ATOMIC`
    pub const ATOMIC: Self = Self(sys::RWF_ATOMIC);
}

impl From<RWFlags> for i32 {
    #[inline]
    fn from(value: RWFlags) -> Self {
        value.0
    }
}

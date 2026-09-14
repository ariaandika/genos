use crate::flags;

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
    pub const HIPRI: Self = Self(RWF_HIPRI);
    /// `RWF_NOWAIT`
    pub const NOWAIT: Self = Self(RWF_NOWAIT);
    /// `RWF_DONTCACHE`
    pub const DONTCACHE: Self = Self(RWF_DONTCACHE);
    /// `RWF_DSYNC`
    pub const DSYNC: Self = Self(RWF_DSYNC);
    /// `RWF_SYNC`
    pub const SYNC: Self = Self(RWF_SYNC);
    /// `RWF_APPEND`
    pub const APPEND: Self = Self(RWF_APPEND);
    /// `RWF_NOAPPEND`
    pub const NOAPPEND: Self = Self(RWF_NOAPPEND);
    /// `RWF_ATOMIC`
    pub const ATOMIC: Self = Self(RWF_ATOMIC);
}

impl From<RWFlags> for i32 {
    #[inline]
    fn from(value: RWFlags) -> Self {
        value.0
    }
}

// ===== extern =====

// include/uapi/linux/fs.h

const RWF_HIPRI: i32 = 0x00000001;
const RWF_DSYNC: i32 = 0x00000002;
const RWF_SYNC: i32 = 0x00000004;
const RWF_NOWAIT: i32 = 0x00000008;
const RWF_APPEND: i32 = 0x00000010;
const RWF_NOAPPEND: i32 = 0x00000020;
const RWF_ATOMIC: i32 = 0x00000040;
const RWF_DONTCACHE: i32 = 0x00000080;

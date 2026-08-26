use crate::{flags, sys};

// ===== ReadFlags =====

/// [`Read::preadv`] and [`Write::pwritev`] flags.
///
/// Reference: `readv2(2)`.
///
/// [`Read::preadv`]: crate::io::Read::preadv
/// [`Write::pwritev`]: crate::io::Write::pwritev
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct IOFlags(i32);

flags::impl_bitops_simple!(IOFlags);

impl IOFlags {
    /// High priority read/write.
    pub const HIPRI: Self = Self(sys::RWF_HIPRI);
    /// Do not wait for data which is not immediately available.
    ///
    /// Currently, this flag is meaningful only for [`Read::preadv`].
    ///
    /// [`Read::preadv`]: crate::io::Read::preadv
    pub const NOWAIT: Self = Self(sys::RWF_NOWAIT);
    /// Reads or writes to a regular file will prune instantiated page cache content when the
    /// operation completes.
    pub const DONTCACHE: Self = Self(sys::RWF_DONTCACHE);

    /// Provide a per-write equivalent of the `O_DSYNC` `open(2)` flag.
    ///
    /// This flag is meaningful only for [`Write::pwritev`].
    ///
    /// [`Write::pwritev`]: crate::io::Write::pwritev
    pub const DSYNC: Self = Self(sys::RWF_DSYNC);
    /// Provide a per-write equivalent of the `O_SYNC` `open(2)` flag.
    ///
    /// This flag is meaningful only for [`Write::pwritev`].
    ///
    /// [`Write::pwritev`]: crate::io::Write::pwritev
    pub const SYNC: Self = Self(sys::RWF_SYNC);
    /// Provide a per-write equivalent of the `O_APPEND` `open(2)` flag.
    ///
    /// This flag is meaningful only for [`Write::pwritev`].
    ///
    /// [`Write::pwritev`]: crate::io::Write::pwritev
    pub const APPEND: Self = Self(sys::RWF_APPEND);
    /// Do not honor the `O_APPEND` `open(2)` flag.
    ///
    /// This flag is meaningful only for [`Write::pwritev`].
    ///
    /// [`Write::pwritev`]: crate::io::Write::pwritev
    pub const NOAPPEND: Self = Self(sys::RWF_NOAPPEND);
    /// Requires that writes to regular files in block-based filesystems be issued with torn-write
    /// protection.
    ///
    /// This flag is meaningful only for [`Write::pwritev`].
    ///
    /// [`Write::pwritev`]: crate::io::Write::pwritev
    pub const ATOMIC: Self = Self(sys::RWF_ATOMIC);
}

impl From<IOFlags> for i32 {
    #[inline]
    fn from(value: IOFlags) -> Self {
        value.0
    }
}

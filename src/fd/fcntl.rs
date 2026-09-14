use crate::flags;

/// `open(2)` flags.
#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct Open(i32);

flags::impl_bitops_simple!(Open);

impl Open {
    /// `O_RDONLY`
    pub const RDONLY: Open = Open(O_RDONLY);
    /// `O_WRONLY`
    pub const WRONLY: Open = Open(O_WRONLY);
    /// `O_RDWR`
    pub const RDWR: Open = Open(O_RDWR);
    /// `O_CREAT`
    pub const CREAT: Open = Open(O_CREAT);
    /// `O_EXCL`
    pub const EXCL: Open = Open(O_EXCL);
    /// `O_NOCTTY`
    pub const NOCTTY: Open = Open(O_NOCTTY);
    /// `O_TRUNC`
    pub const TRUNC: Open = Open(O_TRUNC);
    /// `O_APPEND`
    pub const APPEND: Open = Open(O_APPEND);
    /// `O_NONBLOCK`
    pub const NONBLOCK: Open = Open(O_NONBLOCK);
    /// `O_DSYNC`
    pub const DSYNC: Open = Open(O_DSYNC);
    /// `FASYNC`
    pub const FASYNC: Open = Open(FASYNC);
    /// `O_DIRECT`
    pub const DIRECT: Open = Open(O_DIRECT);
    /// `O_LARGEFILE`
    pub const LARGEFILE: Open = Open(O_LARGEFILE);
    /// `O_DIRECTORY`
    pub const DIRECTORY: Open = Open(O_DIRECTORY);
    /// `O_NOFOLLOW`
    pub const NOFOLLOW: Open = Open(O_NOFOLLOW);
    /// `O_NOATIME`
    pub const NOATIME: Open = Open(O_NOATIME);
    /// `O_CLOEXEC`
    pub const CLOEXEC: Open = Open(O_CLOEXEC);
    /// `O_SYNC`
    pub const SYNC: Open = Open(O_SYNC);
    /// `O_TMPFILE`
    pub const TMPFILE: Open = Open(O_TMPFILE);

    /// `O_ASYNC`
    ///
    /// The Linux header file `<asm/fcntl.h>` doesn't define `O_ASYNC`; the (BSD-derived) `FASYNC`
    /// synonym is defined instead.
    pub const ASYNC: Open = Self::FASYNC;

    pub(crate) const fn raw(self) -> i32 {
        self.0
    }
}

// ===== extern =====

// include/uapi/asm-generic/fcntl.h

pub const O_RDONLY: i32 = 0;
pub const O_WRONLY: i32 = 1 << 0;
pub const O_RDWR: i32 = 1 << 1;
pub const O_CREAT: i32 = 1 << 6;
pub const O_EXCL: i32 = 1 << 7;
pub const O_NOCTTY: i32 = 1 << 8;
pub const O_TRUNC: i32 = 1 << 9;
pub const O_APPEND: i32 = 1 << 10;
pub const O_NONBLOCK: i32 = 1 << 11;
pub const O_DSYNC: i32 = 1 << 12;
pub const FASYNC: i32 = 1 << 13;
pub const O_DIRECT: i32 = 1 << 14;
pub const O_LARGEFILE: i32 = 1 << 15;
pub const O_DIRECTORY: i32 = 1 << 16;
pub const O_NOFOLLOW: i32 = 1 << 17;
pub const O_NOATIME: i32 = 1 << 18;
pub const O_CLOEXEC: i32 = 1 << 19;

const __O_SYNC: i32 = 1 << 20;
pub const O_SYNC: i32 = __O_SYNC | O_DSYNC;

const __O_TMPFILE: i32 = 1 << 22;
// a horrid kludge trying to make sure that this will fail on old kernels
pub const O_TMPFILE: i32 = __O_TMPFILE | O_DIRECTORY;

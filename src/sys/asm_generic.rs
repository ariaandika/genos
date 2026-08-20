// source: include/uapi/asm-generic/fcntl.h

pub const O_NONBLOCK: i32 = 1 << 11;
pub const O_CLOEXEC: i32 = 1 << 19;

// source: include/uapi/asm-generic/errno-base.h

pub const EINTR: i32 = 4;
pub const EAGAIN: i32 = 11;
pub const ENOMEM: i32 = 12;
pub const EINVAL: i32 = 22;

// source: include/uapi/asm-generic/errno.h

pub const EWOULDBLOCK: i32 = EAGAIN;

// source: include/uapi/asm-generic/socket.h

pub const SOL_SOCKET: i32 = 1;

// source: include/uapi/asm-generic/signal-defs.h

pub const SIG_BLOCK: i32 = 0;
pub const SIG_UNBLOCK: i32 = 1;
pub const SIG_SETMASK: i32 = 2;

// source: include/uapi/asm-generic/mman-common.h

pub const PROT_READ: i32 = 0x1;
pub const PROT_WRITE: i32 = 0x2;
pub const PROT_EXEC: i32 = 0x4;
pub const PROT_NONE: i32 = 0x0;
pub const MAP_ANONYMOUS: i32 = 0x20;

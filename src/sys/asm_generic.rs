#![allow(non_camel_case_types, non_snake_case)]
use core::ffi;

// source: include/uapi/asm-generic/fcntl.h

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

// The Linux header file <asm/fcntl.h> doesn't define O_ASYNC;
// the (BSD-derived) FASYNC synonym is defined instead.
// - open(2)
pub const O_ASYNC: i32 = FASYNC;

const __O_TMPFILE: i32 = 1 << 22;
// a horrid kludge trying to make sure that this will fail on old kernels
pub const O_TMPFILE: i32 = __O_TMPFILE | O_DIRECTORY;

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

// source: include/uapi/asm-generic/posix_types.h

pub type __kernel_mode_t = ffi::c_uint;
pub type __kernel_off_t = ffi::c_long;

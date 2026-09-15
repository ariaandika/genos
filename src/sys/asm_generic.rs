#![allow(non_camel_case_types, non_snake_case)]
pub const O_NONBLOCK: i32 = 1 << 11;
pub const O_CLOEXEC: i32 = 1 << 19;

// source: include/uapi/asm-generic/socket.h

pub const SOL_SOCKET: i32 = 1;

// source: include/uapi/asm-generic/signal-defs.h

pub const SIG_BLOCK: i32 = 0;
pub const SIG_UNBLOCK: i32 = 1;
pub const SIG_SETMASK: i32 = 2;

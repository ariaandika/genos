use core::ffi;

use crate::fd;

// ===== bits/socket.h =====

/// `sockaddr(3type)`
pub type Socklen = u32;

// ===== include/linux/socket.h =====

/// `sockaddr(3type)`
pub type SaFamily = ffi::c_ushort;

pub const SCM_RIGHTS: i32 = 0x01;
// pub const SCM_CREDENTIALS: i32 = 0x02;
// pub const SCM_SECURITY: i32 = 0x03;
// pub const SCM_PIDFD: i32 = 0x04;

pub const AF_UNIX: i32 = 1;
pub const AF_LOCAL: i32 = 1;
pub const AF_INET: i32 = 2;

pub const MSG_OOB: i32 = 1;
pub const MSG_PEEK: i32 = 2;
pub const MSG_CTRUNC: i32 = 8;
pub const MSG_TRUNC: i32 = 0x20;
pub const MSG_DONTWAIT: i32 = 0x40;
pub const MSG_EOR: i32 = 0x80;
pub const MSG_ERRQUEUE: i32 = 0x2000;
pub const MSG_CMSG_CLOEXEC: i32 = 0x40000000;

// ===== include/linux/net.h =====

// enum sock_type
pub const SOCK_STREAM: i32 = 1;
pub const SOCK_DGRAM: i32 = 2;
pub const SOCK_RAW: i32 = 3;

pub const SOCK_CLOEXEC: i32 = fd::O_CLOEXEC;
pub const SOCK_NONBLOCK: i32 = fd::O_NONBLOCK;

// enum sock_shutdown_cmd
pub const SHUT_RD: i32 = 0;
pub const SHUT_WR: i32 = 1;
pub const SHUT_RDWR: i32 = 2;

// ===== include/net/scm.h =====

pub const SCM_MAX_FD: i32 = 253;

// ===== include/uapi/linux/un.h =====

pub const UNIX_PATH_MAX: usize = 108;

// ===== include/uapi/asm-generic/socket.h =====

pub const SOL_SOCKET: i32 = 1;

// ===== include/linux/socket.h =====

/// `CMSG_ALIGN`
pub const fn cmsg_align(len: usize) -> usize {
    (len + size_of::<usize>() - 1) & !(size_of::<usize>() - 1)
}

/// `CMSG_SPACE`
pub const fn cmsg_space(length: usize) -> usize {
    cmsg_align(length) + cmsg_align(size_of::<cmsghdr>())
}

/// `CMSG_LEN`
pub const fn cmsg_len(length: usize) -> usize {
    cmsg_align(size_of::<cmsghdr>()) + length
}

#[derive(Debug)]
#[repr(C)]
pub struct cmsghdr {
    pub cmsg_len: usize,
    pub cmsg_level: i32,
    pub cmsg_type: i32,
}

// ===== bits/socket.h =====

// struct msghdr

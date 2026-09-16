#![allow(non_camel_case_types, non_snake_case)]
use core::ffi;

use crate::sys;

// source: include/net/scm.h

pub const SCM_MAX_FD: i32 = 253;

// source: include/linux/net.h

pub const SOCK_STREAM: i32 = 1;
pub const SOCK_DGRAM: i32 = 2;
pub const SOCK_RAW: i32 = 3;
pub const SOCK_CLOEXEC: i32 = sys::O_CLOEXEC;
pub const SOCK_NONBLOCK: i32 = sys::O_NONBLOCK;
pub const SHUT_RD: i32 = 0;
pub const SHUT_WR: i32 = 1;
pub const SHUT_RDWR: i32 = 2;

// source: include/linux/socket.h

pub type sa_family_t = __kernel_sa_family_t;

pub const SCM_RIGHTS: i32 = 0x01;
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

pub const fn CMSG_ALIGN(len: usize) -> usize {
    (len + size_of::<usize>() - 1) & !(size_of::<usize>() - 1)
}

pub const fn CMSG_SPACE(length: usize) -> usize {
    CMSG_ALIGN(length) + CMSG_ALIGN(size_of::<cmsghdr>())
}

pub const fn CMSG_LEN(length: usize) -> usize {
    CMSG_ALIGN(size_of::<cmsghdr>()) + length
}

#[repr(C)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [ffi::c_char; 14],
}

/// `recvmsg(2)` `msghdr` struct.
///
/// In the kernel, the struct name is `user_msghdr`, that have the same layout with different field
/// types.
#[repr(C)]
pub struct msghdr {
    pub msg_name: *mut ffi::c_void,
    pub msg_namelen: sys::socklen_t,
    pub msg_iov: *mut iovec,
    pub msg_iovlen: usize,
    pub msg_control: *mut ffi::c_void,
    pub msg_controllen: usize,
    pub msg_flags: i32,
}

#[derive(Debug)]
#[repr(C)]
pub struct cmsghdr {
    pub cmsg_len: usize,
    pub cmsg_level: i32,
    pub cmsg_type: i32,
}

impl cmsghdr {
    /// Returns data length in bytes.
    pub const fn data_len(&self) -> usize {
        // the reverse of `CMSG_LEN`
        self.cmsg_len - CMSG_ALIGN(size_of::<cmsghdr>())
    }
}

// ===== uapi =====

/// source: `sockaddr(3type)`
pub type socklen_t = u32;

// source: include/uapi/linux/uio.h

#[repr(C)]
pub struct iovec {
    pub iov_base: *mut ffi::c_void,
    pub iov_len: usize,
}

// source: include/uapi/linux/un.h

pub type __kernel_sa_family_t = ffi::c_ushort;

pub const UNIX_PATH_MAX: usize = 108;

#![allow(non_camel_case_types, non_snake_case)]
use core::ffi;

use crate::sys;

// source: include/linux/net.h

pub const SOCK_STREAM: i32 = 1;
pub const SOCK_DGRAM: i32 = 2;
pub const SOCK_RAW: i32 = 3;
pub const SOCK_CLOEXEC: i32 = sys::O_CLOEXEC;
pub const SOCK_NONBLOCK: i32 = sys::O_NONBLOCK;

// source: include/linux/socket.h

pub const SCM_RIGHTS: i32 = 0x01;
pub const AF_UNIX: i32 = 1;
pub const AF_LOCAL: i32 = 1;
pub const AF_INET: i32 = 2;
pub const MSG_PEEK: i32 = 2;
pub const MSG_DONTWAIT: i32 = 0x40;
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

#[derive(Debug)]
#[repr(C)]
pub struct cmsghdr {
    pub cmsg_len: usize,
    pub cmsg_level: i32,
    pub cmsg_type: i32,
}

// ===== uapi =====

/// source: `sockaddr(3type)`
pub type socklen_t = u32;

// source: include/uapi/linux/eventpoll.h

pub const EPOLL_CLOEXEC: i32 = sys::O_CLOEXEC;
pub const EPOLL_CTL_ADD: i32 = 1;
pub const EPOLL_CTL_DEL: i32 = 2;
pub const EPOLL_CTL_MOD: i32 = 3;
pub const EPOLLIN: u32 = 0x00000001;
pub const EPOLLPRI: u32 = 0x00000002;
pub const EPOLLOUT: u32 = 0x00000004;
pub const EPOLLERR: u32 = 0x00000008;
pub const EPOLLHUP: u32 = 0x00000010;
pub const EPOLLRDHUP: u32 = 0x00002000;
pub const EPOLLEXCLUSIVE: u32 = 1 << 28;
pub const EPOLLWAKEUP: u32 = 1 << 29;
pub const EPOLLONESHOT: u32 = 1 << 30;
pub const EPOLLET: u32 = 1 << 31;

// source: include/uapi/linux/mman.h

pub const MAP_SHARED: i32 = 0x01;
pub const MAP_PRIVATE: i32 = 0x02;
pub const MAP_SHARED_VALIDATE: i32 = 0x03;

// source: include/uapi/linux/random.h

pub const GRND_NONBLOCK: u32 = 0x0001;
pub const GRND_RANDOM: u32 = 0x0002;
pub const GRND_INSECURE: u32 = 0x0004;

// source: include/uapi/linux/signalfd.h

pub const SFD_CLOEXEC: i32 = sys::O_CLOEXEC;
pub const SFD_NONBLOCK: i32 = sys::O_NONBLOCK;

#[derive(Debug, Clone)]
#[repr(C)]
pub struct signalfd_siginfo {
    pub ssi_signo: u32,
    pub ssi_errno: i32,
    pub ssi_code: i32,
    pub ssi_pid: u32,
    pub ssi_uid: u32,
    pub ssi_fd: i32,
    pub ssi_tid: u32,
    pub ssi_band: u32,
    pub ssi_overrun: u32,
    pub ssi_trapno: u32,
    pub ssi_status: i32,
    pub ssi_int: i32,
    pub ssi_ptr: u64,
    pub ssi_utime: u64,
    pub ssi_stime: u64,
    pub ssi_addr: u64,
    pub ssi_addr_lsb: u16,
    pub __pad2: u16,
    pub ssi_syscall: i32,
    pub ssi_call_addr: u64,
    pub ssi_arch: u32,
    pub __pad: [u8; 28],
}

// source: include/uapi/linux/time.h

pub const CLOCK_REALTIME: i32 = 0;
pub const CLOCK_MONOTONIC: i32 = 1;
pub const CLOCK_BOOTTIME: i32 = 7;
pub const CLOCK_REALTIME_ALARM: i32 = 8;
pub const CLOCK_BOOTTIME_ALARM: i32 = 9;

#[repr(C)]
pub struct itimerspec {
    pub it_interval: timespec,
    pub it_value: timespec,
}

#[repr(C)]
pub struct timespec {
    pub tv_sec: ffi::c_long,
    pub tv_nsec: ffi::c_long,
}

// source: include/uapi/linux/timerfd.h

pub const TFD_TIMER_ABSTIME: i32 = 1 << 0;
pub const TFD_TIMER_CANCEL_ON_SET: i32 = 1 << 1;
pub const TFD_CLOEXEC: i32 = sys::O_CLOEXEC;
pub const TFD_NONBLOCK: i32 = sys::O_NONBLOCK;

// source: include/uapi/linux/un.h

pub type __kernel_sa_family_t = ffi::c_ushort;

pub const UNIX_PATH_MAX: usize = 108;

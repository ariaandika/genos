#![allow(non_camel_case_types, non_snake_case)]
use core::ffi;

use crate::sys;

// source: include/net/scm.h

pub const SCM_MAX_FD: i32 = 253;

// source: include/linux/splice.h

pub const SPLICE_F_MOVE: u32 = 0x01;
pub const SPLICE_F_NONBLOCK: u32 = 0x02;
pub const SPLICE_F_MORE: u32 = 0x04;
pub const SPLICE_F_GIFT: u32 = 0x08;

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

// source: include/linux/types.h

pub type mode_t = sys::__kernel_mode_t;
pub type off_t = sys::__kernel_off_t;

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

// source: include/uapi/linux/fs.h

pub const SEEK_SET: i32 = 0;
pub const SEEK_CUR: i32 = 1;
pub const SEEK_END: i32 = 2;
pub const SEEK_DATA: i32 = 3;
pub const SEEK_HOLE: i32 = 4;

pub const RWF_HIPRI: i32 = 0x00000001;
pub const RWF_DSYNC: i32 = 0x00000002;
pub const RWF_SYNC: i32 = 0x00000004;
pub const RWF_NOWAIT: i32 = 0x00000008;
pub const RWF_APPEND: i32 = 0x00000010;
pub const RWF_NOAPPEND: i32 = 0x00000020;
pub const RWF_ATOMIC: i32 = 0x00000040;
pub const RWF_DONTCACHE: i32 = 0x00000080;

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
// pub const CLOCK_PROCESS_CPUTIME_ID: i32 = 2;
// pub const CLOCK_THREAD_CPUTIME_ID: i32 = 3;
pub const CLOCK_MONOTONIC_RAW: i32 = 4;
pub const CLOCK_REALTIME_COARSE: i32 = 5;
pub const CLOCK_MONOTONIC_COARSE: i32 = 6;
pub const CLOCK_BOOTTIME: i32 = 7;
pub const CLOCK_REALTIME_ALARM: i32 = 8;
pub const CLOCK_BOOTTIME_ALARM: i32 = 9;
pub const CLOCK_TAI: i32 = 11;

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

// source: include/uapi/linux/uio.h

#[repr(C)]
pub struct iovec {
    pub iov_base: *mut ffi::c_void,
    pub iov_len: usize,
}

// source: include/uapi/linux/un.h

pub type __kernel_sa_family_t = ffi::c_ushort;

pub const UNIX_PATH_MAX: usize = 108;

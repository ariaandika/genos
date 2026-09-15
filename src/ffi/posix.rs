#![expect(non_camel_case_types)]
use core::ffi::{c_int, c_long, c_uint};

// include/uapi/asm-generic/posix_types.h

type __kernel_long_t = c_long;
type __kernel_mode_t = c_uint;
type __kernel_pid_t = c_int;

type __kernel_uid32_t = c_uint;
type __kernel_gid32_t = c_uint;

type __kernel_off_t = __kernel_long_t;
type __kernel_time_t = __kernel_long_t;
type __kernel_timer_t = c_int;
type __kernel_clockid_t = c_int;

// include/uapi/linux/types.h

// __poll_t

// include/linux/types.h

/// `mode_t(3type)`
pub type Mode = __kernel_mode_t;

/// `off_t(3type)`
pub type Off = __kernel_off_t;

/// `pid_t(3type)`
pub type Pid = __kernel_pid_t;

/// `timer_t(3type)`
pub type Timer = __kernel_timer_t;

/// `clockid_t(3type)`
pub type ClockID = __kernel_clockid_t;

/// `uid_t(3type)`
pub type Uid = __kernel_uid32_t;

/// `gid_t(3type)`
pub type Gid = __kernel_gid32_t;

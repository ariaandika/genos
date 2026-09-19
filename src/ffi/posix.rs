use core::ffi::{c_int, c_long, c_uint};

// time/bits/types/time_t.h

/// `time_t(3type)`
pub type Time = c_long;

// include/linux/types.h

/// `mode_t(3type)`
pub type Mode = c_uint;

/// `off_t(3type)`
pub type Off = c_long;

/// `pid_t(3type)`
pub type Pid = c_int;

/// `timer_t(3type)`
pub type Timer = c_int;

/// `clockid_t(3type)`
pub type ClockID = c_int;

/// `uid_t(3type)`
pub type Uid = c_uint;

/// `gid_t(3type)`
pub type Gid = c_uint;

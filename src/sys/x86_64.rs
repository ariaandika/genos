//! Architecture specific value.
#![expect(non_camel_case_types, missing_debug_implementations)]
use crate::sys::{SysId, SysRaw};

macro_rules! defv2 {
    ($($no:literal $abi:tt $name:ident $entryp:ident $($alias:ident)?)*) => {$(
        #[doc = concat!(" `", stringify!($name), "`")]
        pub struct $entryp(());
        impl SysId for $entryp {
            const NO: crate::sys::SysRaw = $no;
            const NAME: &str = stringify!($name);
        }
    )*};
}

// arch/x86/entry/syscalls/syscall_64.tbl
// <number> <abi> <name> <entry point> [<compat entry point> [noreturn]]

defv2! {
    0   common  read                sys_read
    1   common  write               sys_write
    2   common  open                sys_open
    3   common  close               sys_close
    8   common  lseek               sys_lseek
    9   common  mmap                sys_mmap
    10  common  mprotect            sys_mprotect
    11  common  munmap              sys_munmap
    12  common  brk                 sys_brk
    14  common  rt_sigprocmask      sys_rt_sigprocmask
    17  common  pread64             sys_pread64
    18  common  pwrite64            sys_pwrite64
    19  64      readv               sys_readv
    20  64      writev              sys_writev
    35  common  nanosleep           sys_nanosleep
    39  common  getpid              sys_getpid
    40  common  sendfile            sys_sendfile64
    41  common  socket              sys_socket
    42  common  connect             sys_connect
    44  common  sendto              sys_sendto
    45  64  recvfrom                sys_recvfrom
    46  64  sendmsg                 sys_sendmsg
    47  64  recvmsg                 sys_recvmsg
    48  common  shutdown            sys_shutdown
    49  common  bind                sys_bind
    50  common  listen              sys_listen
    51  common  getsockname         sys_getsockname
    52  common  getpeername         sys_getpeername
    56  common  clone               sys_clone
    57  common  fork                sys_fork
    59  64      execve              sys_execve
    60  common  exit                sys_exit
    62  common  kill                sys_kill
    76  common  truncate            sys_truncate
    77  common  ftruncate           sys_ftruncate
    82  common  rename              sys_rename
    85  common  creat               sys_creat
    86  common  link                sys_link
    87  common  unlink              sys_unlink
    88  common  symlink             sys_symlink
    110 common  getppid             sys_getppid
    201 common  time                sys_time
    227 common  clock_settime       sys_clock_settime
    228 common  clock_gettime       sys_clock_gettime
    229 common  clock_getres        sys_clock_getres
    230 common  clock_nanosleep     sys_clock_nanosleep
    232 common  epoll_wait          sys_epoll_wait
    233 common  epoll_ctl           sys_epoll_ctl
    275 common  splice              sys_splice
    276 common  tee                 sys_tee
    278 64      vmsplice            sys_vmsplice
    283 common  timerfd_create      sys_timerfd_create
    286 common  timerfd_settime     sys_timerfd_settime
    287 common  timerfd_gettime     sys_timerfd_gettime
    288 common  accept4             sys_accept4
    289 common  signalfd4           sys_signalfd4
    290 common  eventfd2            sys_eventfd2
    291 common  epoll_create1       sys_epoll_create1
    318 common  getrandom           sys_getrandom
    319 common  memfd_create        sys_memfd_create
    327 64      preadv2             sys_preadv2
    328 64      pwritev2            sys_pwritev2
    435 common  clone3              sys_clone3
}

#[inline]
pub(crate) unsafe fn call1_noret(nr: SysRaw, a1: i32) -> ! {
    // [ud2]: <https://doc.rust-lang.org/reference/inline-assembly.html#r-asm.options.supported-options.noreturn>
    unsafe {
        core::arch::asm!(
            "syscall",
            "ud2",
            in("rax") nr,
            in("rdi") a1,
            options(nostack, noreturn)
        )
    }
}

macro_rules! call_impl {
    ($nr:ident, [$($flags:ident),*], |$ret_v:ident$(: $ret_ty:ty)?| $map:expr
     $(, $a1:expr
     $(, $a2:expr
     $(, $a3:expr
     $(, $a4:expr
     $(, $a5:expr
     $(, $a6:expr
     )?)?)?)?)?)?
    ) => { unsafe {
        use crate::sys;
        let $ret_v $(: $ret_ty)?;
        core::arch::asm!(
            "syscall",
            inlateout("rax") <sys::arch::$nr as sys::SysId>::NO => $ret_v,
            $(in("rdi") $a1,
            $(in("rsi") $a2,
            $(in("rdx") $a3,
            $(in("r10") $a4,
            $(in("r8") $a5,
            $(in("r9") $a6,
            )?)?)?)?)?)?
            lateout("rcx") _,
            lateout("r11") _,
            options(nostack, preserves_flags $(, $flags)*)
        );
        $map
    } };
}

/// Perform a syscall.
///
/// This is simple, function like macro, where the 1st argument is the syscall identifier, and the
/// rest is the syscall arguments.
macro_rules! call {
    ($nr:ident $($tt:tt)*) => {
        sys::call_impl!($nr, [], |ret|sys::Error::<sys::arch::$nr>::from_raw(ret) $($tt)*)
    };
}

/// Perform a readonly syscall.
///
/// This is simple, function like macro, where the 1st argument is the syscall identifier, and the
/// rest is the syscall arguments.
///
/// In contrast with [`call!`], this syscall should not mutate userspace memory.
macro_rules! call_rd {
    ($nr:ident $($tt:tt)*) => {
        sys::call_impl!($nr, [readonly], |ret|sys::Error::<sys::arch::$nr>::from_raw(ret) $($tt)*)
    };
}

/// Perform a syscall.
///
/// This is simple, function like macro, where the 1st argument is the syscall identifier, and the
/// rest is the syscall arguments.
macro_rules! call_rd_raw {
    ($nr:ident $($tt:tt)*) => {
        sys::call_impl!($nr, [readonly], |ret:sys::SysRaw|ret $($tt)*)
    };
}

pub(crate) use call;
pub(crate) use call_impl;
pub(crate) use call_rd;
pub(crate) use call_rd_raw;

#![allow(non_upper_case_globals, non_camel_case_types, unsafe_op_in_unsafe_fn)]
use core::arch::asm;
use core::ffi::{self, c_long, c_ulong};

use crate::sys::SysRes;
pub use crate::sys::asm_generic::*;

// syscall arguments use register-sized types
// syscall return values use register-sized types
//
// `man syscall(2)`
//
// # Architecture Calling Convention
//
// Registers used to pass the system call arguments.
//
// Arch/ABI      arg1  arg2  arg3  arg4  arg5  arg6  arg7  Notes
// ──────────────────────────────────────────────────────────────
// x86-64        rdi   rsi   rdx   r10   r8    r9    -
//
// # ASM
//
// "rax" contains syscall nr as input, then syscall returns the value back to it
//
// More on Rust inline assembly: https://doc.rust-lang.org/reference/inline-assembly.html

#[inline]
pub(crate) unsafe fn call0_rd(nr: c_long) -> SysRes {
    let ret;
    asm!(
        "syscall",
        inlateout("rax") nr => ret,
        lateout("rcx") _,
        lateout("r11") _,
        options(nostack, preserves_flags, readonly)
    );
    SysRes::new(ret)
}

#[inline]
pub(crate) unsafe fn call1_rd(nr: c_long, a1: usize) -> SysRes {
    let ret;
    asm!(
        "syscall",
        inlateout("rax") nr => ret,
        in("rdi") a1,
        lateout("rcx") _,
        lateout("r11") _,
        options(nostack, preserves_flags, readonly)
    );
    SysRes::new(ret)
}

#[inline]
pub(crate) unsafe fn call1_noret(nr: c_long, a1: usize) -> ! {
    // [ud2]: <https://doc.rust-lang.org/reference/inline-assembly.html#r-asm.options.supported-options.noreturn>
    asm!(
        "syscall",
        "ud2",
        in("rax") nr,
        in("rdi") a1,
        options(nostack, noreturn)
    )
}

#[inline]
pub(crate) unsafe fn call2(nr: c_long, a1: usize, a2: usize) -> SysRes {
    let ret;
    asm!(
        "syscall",
        inlateout("rax") nr => ret,
        in("rdi") a1,
        in("rsi") a2,
        lateout("rcx") _,
        lateout("r11") _,
        options(nostack, preserves_flags)
    );
    SysRes::new(ret)
}

#[inline]
pub(crate) unsafe fn call2_rd(nr: c_long, a1: usize, a2: usize) -> SysRes {
    let ret;
    asm!(
        "syscall",
        inlateout("rax") nr => ret,
        in("rdi") a1,
        in("rsi") a2,
        lateout("rcx") _,
        lateout("r11") _,
        options(nostack, preserves_flags, readonly)
    );
    SysRes::new(ret)
}

#[inline]
pub(crate) unsafe fn call3(nr: c_long, a1: usize, a2: usize, a3: usize) -> SysRes {
    let ret;
    asm!(
        "syscall",
        inlateout("rax") nr => ret,
        in("rdi") a1,
        in("rsi") a2,
        in("rdx") a3,
        lateout("rcx") _,
        lateout("r11") _,
        options(nostack, preserves_flags)
    );
    SysRes::new(ret)
}

#[inline]
pub(crate) unsafe fn call3_rd(nr: c_long, a1: usize, a2: usize, a3: usize) -> SysRes {
    let ret;
    asm!(
        "syscall",
        inlateout("rax") nr => ret,
        in("rdi") a1,
        in("rsi") a2,
        in("rdx") a3,
        lateout("rcx") _,
        lateout("r11") _,
        options(nostack, preserves_flags, readonly)
    );
    SysRes::new(ret)
}

#[inline]
pub(crate) unsafe fn call4(nr: c_long, a1: usize, a2: usize, a3: usize, a4: usize) -> SysRes {
    let ret;
    asm!(
        "syscall",
        inlateout("rax") nr => ret,
        in("rdi") a1,
        in("rsi") a2,
        in("rdx") a3,
        in("r10") a4,
        lateout("rcx") _,
        lateout("r11") _,
        options(nostack, preserves_flags)
    );
    SysRes::new(ret)
}

#[inline]
pub(crate) unsafe fn call4_rd(nr: c_long, a1: usize, a2: usize, a3: usize, a4: usize) -> SysRes {
    let ret;
    asm!(
        "syscall",
        inlateout("rax") nr => ret,
        in("rdi") a1,
        in("rsi") a2,
        in("rdx") a3,
        in("r10") a4,
        lateout("rcx") _,
        lateout("r11") _,
        options(nostack, preserves_flags, readonly)
    );
    SysRes::new(ret)
}

#[inline]
pub(crate) unsafe fn call5(
    nr: c_long,
    a1: usize,
    a2: usize,
    a3: usize,
    a4: usize,
    a5: usize,
) -> SysRes {
    let ret;
    asm!(
        "syscall",
        inlateout("rax") nr => ret,
        in("rdi") a1,
        in("rsi") a2,
        in("rdx") a3,
        in("r10") a4,
        in("r8") a5,
        lateout("rcx") _,
        lateout("r11") _,
        options(nostack, preserves_flags)
    );
    SysRes::new(ret)
}

#[inline]
pub(crate) unsafe fn call5_rd(
    nr: c_long,
    a1: usize,
    a2: usize,
    a3: usize,
    a4: usize,
    a5: usize,
) -> SysRes {
    let ret;
    asm!(
        "syscall",
        inlateout("rax") nr => ret,
        in("rdi") a1,
        in("rsi") a2,
        in("rdx") a3,
        in("r10") a4,
        in("r8") a5,
        lateout("rcx") _,
        lateout("r11") _,
        options(nostack, preserves_flags, readonly)
    );
    SysRes::new(ret)
}

#[inline]
pub(crate) unsafe fn call6(
    nr: c_long,
    a1: usize,
    a2: usize,
    a3: usize,
    a4: usize,
    a5: usize,
    a6: usize,
) -> SysRes {
    let ret;
    asm!(
        "syscall",
        inlateout("rax") nr => ret,
        in("rdi") a1,
        in("rsi") a2,
        in("rdx") a3,
        in("r10") a4,
        in("r8") a5,
        in("r9") a6,
        lateout("rcx") _,
        lateout("r11") _,
        options(nostack, preserves_flags)
    );
    SysRes::new(ret)
}

#[inline]
pub(crate) unsafe fn call6_rd(
    nr: c_long,
    a1: usize,
    a2: usize,
    a3: usize,
    a4: usize,
    a5: usize,
    a6: usize,
) -> SysRes {
    let ret;
    asm!(
        "syscall",
        inlateout("rax") nr => ret,
        in("rdi") a1,
        in("rsi") a2,
        in("rdx") a3,
        in("r10") a4,
        in("r8") a5,
        in("r9") a6,
        lateout("rcx") _,
        lateout("r11") _,
        options(nostack, preserves_flags, readonly)
    );
    SysRes::new(ret)
}

// Roughly speaking, the code belonging to the system call with
// number __NR_xxx defined in /usr/include/asm/unistd.h can be found
// in the Linux kernel source in the routine sys_xxx().  There are
// many exceptions, however, mostly because older system calls were
// superseded by newer ones, and this has been treated somewhat
// unsystematically.
//
// - `syscalls(2)`

// arch/x86/entry/syscalls/syscall_64.tbl

pub const sys_read: c_long = 0;
pub const sys_write: c_long = 1;
pub const sys_open: c_long = 2;
pub const sys_close: c_long = 3;
pub const sys_lseek: c_long = 8;
pub const sys_mmap: c_long = 9;
pub const sys_mprotect: c_long = 10;
pub const sys_munmap: c_long = 11;
pub const sys_brk: c_long = 12;
pub const sys_rt_sigprocmask: c_long = 14;
pub const sys_pread64: c_long = 17;
pub const sys_pwrite64: c_long = 18;
pub const sys_readv: c_long = 19;
pub const sys_writev: c_long = 20;
pub const sys_getpid: c_long = 39;
pub const sys_sendfile: c_long = 40;
pub const sys_socket: c_long = 41;
pub const sys_connect: c_long = 42;
pub const sys_sendto: c_long = 44;
pub const sys_recvfrom: c_long = 45;
pub const sys_sendmsg: c_long = 46;
pub const sys_recvmsg: c_long = 47;
pub const sys_shutdown: c_long = 48;
pub const sys_bind: c_long = 49;
pub const sys_listen: c_long = 50;
pub const sys_getsockname: c_long = 51;
pub const sys_getpeername: c_long = 52;
pub const sys_clone: c_long = 56;
pub const sys_fork: c_long = 57;
pub const sys_execve: c_long = 59;
pub const sys_exit: c_long = 60;
pub const sys_kill: c_long = 62;
pub const sys_truncate: c_long = 76;
pub const sys_ftruncate: c_long = 77;
pub const sys_rename: c_long = 82;
pub const sys_creat: c_long = 85;
pub const sys_link: c_long = 86;
pub const sys_unlink: c_long = 87;
pub const sys_symlink: c_long = 88;
pub const sys_getppid: c_long = 110;
pub const sys_time: c_long = 201;
pub const sys_clock_settime: c_long = 227;
pub const sys_clock_gettime: c_long = 228;
pub const sys_clock_getres: c_long = 229;
pub const sys_epoll_wait: c_long = 232;
pub const sys_epoll_ctl: c_long = 233;
pub const sys_splice: c_long = 275;
pub const sys_tee: c_long = 276;
pub const sys_vmsplice: c_long = 278;
pub const sys_signalfd: c_long = 282;
pub const sys_timerfd_create: c_long = 283;
pub const sys_timerfd_settime: c_long = 286;
pub const sys_timerfd_gettime: c_long = 287;
pub const sys_accept4: c_long = 288;
pub const sys_epoll_create1: c_long = 291;
pub const sys_getrandom: c_long = 318;
pub const sys_preadv2: c_long = 327;
pub const sys_pwritev2: c_long = 328;
pub const sys_clone3: c_long = 435;

// source: arch/x86/include/uapi/asm/signal.h

pub type sigset_t = ffi::c_ulong;

pub const SIGHUP: i32 = 1;
pub const SIGINT: i32 = 2;
pub const SIGQUIT: i32 = 3;
pub const SIGILL: i32 = 4;
// pub const SIGTRAP: i32 = 5;
pub const SIGABRT: i32 = 6;
pub const SIGIOT: i32 = 6;
// pub const SIGBUS: i32 = 7;
pub const SIGFPE: i32 = 8;
pub const SIGKILL: i32 = 9;
pub const SIGUSR1: i32 = 10;
pub const SIGSEGV: i32 = 11;
pub const SIGUSR2: i32 = 12;
pub const SIGPIPE: i32 = 13;
pub const SIGALRM: i32 = 14;
pub const SIGTERM: i32 = 15;
// pub const SIGSTKFLT: i32 = 16;
pub const SIGCHLD: i32 = 17;
pub const SIGCONT: i32 = 18;
// pub const SIGSTOP: i32 = 19;
// pub const SIGTSTP: i32 = 20;
// pub const SIGTTIN: i32 = 21;
// pub const SIGTTOU: i32 = 22;
// pub const SIGURG: i32 = 23;
// pub const SIGXCPU: i32 = 24;
// pub const SIGXFSZ: i32 = 25;
// pub const SIGVTALRM: i32 = 26;
// pub const SIGPROF: i32 = 27;
// pub const SIGWINCH: i32 = 28;
// pub const SIGIO: i32 = 29;
// pub const SIGPOLL: i32 = SIGIO;

// source: arch/x86/include/asm/signal.h
// simplified for 64 bit only

#[cfg(target_pointer_width = "64")]
pub const fn sigemptyset() -> sigset_t {
    0
}

#[cfg(target_pointer_width = "64")]
pub const fn sigfillset() -> sigset_t {
    -1i64 as u64
}

#[cfg(target_pointer_width = "64")]
pub const fn sigaddset(set: &mut sigset_t, sig: i32) {
    *set |= 1 << (sig - 1) as c_ulong;
}

#[cfg(target_pointer_width = "64")]
pub const fn sigdelset(set: &mut sigset_t, sig: i32) {
    *set &= !(1 << (sig - 1) as c_ulong);
}

#[cfg(target_pointer_width = "64")]
pub const fn sigismember(set: &sigset_t, sig: i32) -> u64 {
    1 & *set >> (sig - 1) as c_ulong
}

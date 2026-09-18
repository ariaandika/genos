#![expect(non_upper_case_globals, unsafe_op_in_unsafe_fn)]
use core::arch::asm;
use core::ffi::c_long;

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
pub const sys_memfd_create: c_long = 319;
pub const sys_preadv2: c_long = 327;
pub const sys_pwritev2: c_long = 328;
pub const sys_clone3: c_long = 435;

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

/// Perform a syscall.
///
/// This is simple, function like macro, where the 1st argument is the syscall identifier, and the
/// rest is the syscall arguments.
macro_rules! call {
    ($nr:ident
     $(, $a1:expr
     $(, $a2:expr
     $(, $a3:expr
     $(, $a4:expr
     $(, $a5:expr
     $(, $a6:expr
     )?)?)?)?)?)?
    ) => { unsafe {
        let ret;
        core::arch::asm!(
            "syscall",
            inlateout("rax") crate::sys::$nr => ret,
            $(in("rdi") $a1,
            $(in("rsi") $a2,
            $(in("rdx") $a3,
            $(in("r10") $a4,
            $(in("r8") $a5,
            $(in("r9") $a6,
            )?)?)?)?)?)?
            lateout("rcx") _,
            lateout("r11") _,
            options(nostack, preserves_flags)
        );
        core::mem::transmute::<core::ffi::c_long, crate::sys::SysRes>(ret)
    } };
}

/// Perform a readonly syscall.
///
/// This is simple, function like macro, where the 1st argument is the syscall identifier, and the
/// rest is the syscall arguments.
///
/// In contrast with [`call!`], this syscall should not mutate userspace memory.
macro_rules! call_rd {
    ($nr:ident
     $(, $a1:expr
     $(, $a2:expr
     $(, $a3:expr
     $(, $a4:expr
     $(, $a5:expr
     $(, $a6:expr
     )?)?)?)?)?)?
    ) => { unsafe {
        let ret;
        core::arch::asm!(
            "syscall",
            inlateout("rax") crate::sys::$nr => ret,
            $(in("rdi") $a1,
            $(in("rsi") $a2,
            $(in("rdx") $a3,
            $(in("r10") $a4,
            $(in("r8") $a5,
            $(in("r9") $a6,
            )?)?)?)?)?)?
            lateout("rcx") _,
            lateout("r11") _,
            options(nostack, preserves_flags, readonly)
        );
        core::mem::transmute::<core::ffi::c_long, crate::sys::SysRes>(ret)
    } };
}

pub(crate) use call;
pub(crate) use call_rd;

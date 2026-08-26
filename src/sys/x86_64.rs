#![allow(non_upper_case_globals, non_camel_case_types, unsafe_op_in_unsafe_fn)]
use core::arch::asm;
use core::ffi::{self, c_long, c_ulong};

use crate::sys::SysRes;

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
#[doc(hidden)]
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
#[doc(hidden)]
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
#[doc(hidden)]
pub(crate) unsafe fn call1_noret(nr: c_long, a1: usize) -> ! {
    // [ud2]: <https://doc.rust-lang.org/reference/inline-assembly.html#r-asm.options.supported-options.noreturn>
    // #[doc(hidden)]
    asm!(
        "syscall",
        "ud2",
        in("rax") nr,
        in("rdi") a1,
        options(nostack, noreturn)
    )
}

#[inline]
#[doc(hidden)]
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
#[doc(hidden)]
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
#[doc(hidden)]
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
#[doc(hidden)]
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
#[doc(hidden)]
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
#[doc(hidden)]
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
#[doc(hidden)]
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
#[doc(hidden)]
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

pub use crate::sys::asm_generic::*;

// Roughly speaking, the code belonging to the system call with
// number __NR_xxx defined in /usr/include/asm/unistd.h can be found
// in the Linux kernel source in the routine sys_xxx().  There are
// many exceptions, however, mostly because older system calls were
// superseded by newer ones, and this has been treated somewhat
// unsystematically.
//
// - `syscalls(2)`

// source: arch/x86/entry/syscalls/syscall_64.tbl

pub const open: c_long = 2; // __NR_open
pub const lseek: c_long = 8; // __NR_lseek
pub const creat: c_long = 85; // __NR_creat
pub const ftruncate: c_long = 77; // __NR_ftruncate
pub const epoll_wait: c_long = 232; // __NR_epoll_wait
pub const epoll_ctl: c_long = 233; // __NR_epoll_ctl
pub const epoll_create1: c_long = 291; // __NR_epoll_create1

// replaced prefix `sys_` with `__NR_`

pub const __NR_read: c_long = 0;
pub const __NR_write: c_long = 1;
pub const __NR_close: c_long = 3;
pub const __NR_stat: c_long = 4;
pub const __NR_fstat: c_long = 5;
pub const __NR_lstat: c_long = 6;
pub const __NR_poll: c_long = 7;
pub const __NR_mmap: c_long = 9;
pub const __NR_mprotect: c_long = 10;
pub const __NR_munmap: c_long = 11;
pub const __NR_brk: c_long = 12;
pub const __NR_rt_sigaction: c_long = 13;
pub const __NR_rt_sigprocmask: c_long = 14;
pub const __NR_rt_sigreturn: c_long = 15;
pub const __NR_ioctl: c_long = 16;
pub const __NR_pread64: c_long = 17;
pub const __NR_pwrite64: c_long = 18;
pub const __NR_readv: c_long = 19;
pub const __NR_writev: c_long = 20;
pub const __NR_access: c_long = 21;
pub const __NR_pipe: c_long = 22;
pub const __NR_select: c_long = 23;
pub const __NR_sched_yield: c_long = 24;
pub const __NR_mremap: c_long = 25;
pub const __NR_msync: c_long = 26;
pub const __NR_mincore: c_long = 27;
pub const __NR_madvise: c_long = 28;
pub const __NR_shmget: c_long = 29;
pub const __NR_shmat: c_long = 30;
pub const __NR_shmctl: c_long = 31;
pub const __NR_dup: c_long = 32;
pub const __NR_dup2: c_long = 33;
pub const __NR_pause: c_long = 34;
pub const __NR_nanosleep: c_long = 35;
pub const __NR_getitimer: c_long = 36;
pub const __NR_alarm: c_long = 37;
pub const __NR_setitimer: c_long = 38;
pub const __NR_getpid: c_long = 39;
pub const __NR_sendfile: c_long = 40;
pub const __NR_socket: c_long = 41;
pub const __NR_connect: c_long = 42;
pub const __NR_accept: c_long = 43;
pub const __NR_sendto: c_long = 44;
pub const __NR_recvfrom: c_long = 45;
pub const __NR_sendmsg: c_long = 46;
pub const __NR_recvmsg: c_long = 47;
pub const __NR_shutdown: c_long = 48;
pub const __NR_bind: c_long = 49;
pub const __NR_listen: c_long = 50;
pub const __NR_getsockname: c_long = 51;
pub const __NR_getpeername: c_long = 52;
pub const __NR_socketpair: c_long = 53;
pub const __NR_setsockopt: c_long = 54;
pub const __NR_getsockopt: c_long = 55;
pub const __NR_clone: c_long = 56;
pub const __NR_fork: c_long = 57;
pub const __NR_vfork: c_long = 58;
pub const __NR_execve: c_long = 59;
pub const __NR_exit: c_long = 60;
pub const __NR_wait4: c_long = 61;
pub const __NR_kill: c_long = 62;
pub const __NR_uname: c_long = 63;
pub const __NR_semget: c_long = 64;
pub const __NR_semop: c_long = 65;
pub const __NR_semctl: c_long = 66;
pub const __NR_shmdt: c_long = 67;
pub const __NR_msgget: c_long = 68;
pub const __NR_msgsnd: c_long = 69;
pub const __NR_msgrcv: c_long = 70;
pub const __NR_msgctl: c_long = 71;
pub const __NR_fcntl: c_long = 72;
pub const __NR_flock: c_long = 73;
pub const __NR_fsync: c_long = 74;
pub const __NR_fdatasync: c_long = 75;
pub const __NR_truncate: c_long = 76;
pub const __NR_getdents: c_long = 78;
pub const __NR_getcwd: c_long = 79;
pub const __NR_chdir: c_long = 80;
pub const __NR_fchdir: c_long = 81;
pub const __NR_rename: c_long = 82;
pub const __NR_mkdir: c_long = 83;
pub const __NR_rmdir: c_long = 84;
pub const __NR_link: c_long = 86;
pub const __NR_unlink: c_long = 87;
pub const __NR_symlink: c_long = 88;
pub const __NR_readlink: c_long = 89;
pub const __NR_chmod: c_long = 90;
pub const __NR_fchmod: c_long = 91;
pub const __NR_chown: c_long = 92;
pub const __NR_fchown: c_long = 93;
pub const __NR_lchown: c_long = 94;
pub const __NR_umask: c_long = 95;
pub const __NR_gettimeofday: c_long = 96;
pub const __NR_getrlimit: c_long = 97;
pub const __NR_getrusage: c_long = 98;
pub const __NR_sysinfo: c_long = 99;
pub const __NR_times: c_long = 100;
pub const __NR_ptrace: c_long = 101;
pub const __NR_getuid: c_long = 102;
pub const __NR_syslog: c_long = 103;
pub const __NR_getgid: c_long = 104;
pub const __NR_setuid: c_long = 105;
pub const __NR_setgid: c_long = 106;
pub const __NR_geteuid: c_long = 107;
pub const __NR_getegid: c_long = 108;
pub const __NR_setpgid: c_long = 109;
pub const __NR_getppid: c_long = 110;
pub const __NR_getpgrp: c_long = 111;
pub const __NR_setsid: c_long = 112;
pub const __NR_setreuid: c_long = 113;
pub const __NR_setregid: c_long = 114;
pub const __NR_getgroups: c_long = 115;
pub const __NR_setgroups: c_long = 116;
pub const __NR_setresuid: c_long = 117;
pub const __NR_getresuid: c_long = 118;
pub const __NR_setresgid: c_long = 119;
pub const __NR_getresgid: c_long = 120;
pub const __NR_getpgid: c_long = 121;
pub const __NR_setfsuid: c_long = 122;
pub const __NR_setfsgid: c_long = 123;
pub const __NR_getsid: c_long = 124;
pub const __NR_capget: c_long = 125;
pub const __NR_capset: c_long = 126;
pub const __NR_rt_sigpending: c_long = 127;
pub const __NR_rt_sigtimedwait: c_long = 128;
pub const __NR_rt_sigqueueinfo: c_long = 129;
pub const __NR_rt_sigsuspend: c_long = 130;
pub const __NR_sigaltstack: c_long = 131;
pub const __NR_utime: c_long = 132;
pub const __NR_mknod: c_long = 133;
pub const __NR_uselib: c_long = 134;
pub const __NR_personality: c_long = 135;
pub const __NR_ustat: c_long = 136;
pub const __NR_statfs: c_long = 137;
pub const __NR_fstatfs: c_long = 138;
pub const __NR_sysfs: c_long = 139;
pub const __NR_getpriority: c_long = 140;
pub const __NR_setpriority: c_long = 141;
pub const __NR_sched_setparam: c_long = 142;
pub const __NR_sched_getparam: c_long = 143;
pub const __NR_sched_setscheduler: c_long = 144;
pub const __NR_sched_getscheduler: c_long = 145;
pub const __NR_sched_get_priority_max: c_long = 146;
pub const __NR_sched_get_priority_min: c_long = 147;
pub const __NR_sched_rr_get_interval: c_long = 148;
pub const __NR_mlock: c_long = 149;
pub const __NR_munlock: c_long = 150;
pub const __NR_mlockall: c_long = 151;
pub const __NR_munlockall: c_long = 152;
pub const __NR_vhangup: c_long = 153;
pub const __NR_modify_ldt: c_long = 154;
pub const __NR_pivot_root: c_long = 155;
pub const __NR__sysctl: c_long = 156;
pub const __NR_prctl: c_long = 157;
pub const __NR_arch_prctl: c_long = 158;
pub const __NR_adjtimex: c_long = 159;
pub const __NR_setrlimit: c_long = 160;
pub const __NR_chroot: c_long = 161;
pub const __NR_sync: c_long = 162;
pub const __NR_acct: c_long = 163;
pub const __NR_settimeofday: c_long = 164;
pub const __NR_mount: c_long = 165;
pub const __NR_umount2: c_long = 166;
pub const __NR_swapon: c_long = 167;
pub const __NR_swapoff: c_long = 168;
pub const __NR_reboot: c_long = 169;
pub const __NR_sethostname: c_long = 170;
pub const __NR_setdomainname: c_long = 171;
pub const __NR_iopl: c_long = 172;
pub const __NR_ioperm: c_long = 173;
pub const __NR_create_module: c_long = 174;
pub const __NR_init_module: c_long = 175;
pub const __NR_delete_module: c_long = 176;
pub const __NR_get_kernel_syms: c_long = 177;
pub const __NR_query_module: c_long = 178;
pub const __NR_quotactl: c_long = 179;
pub const __NR_nfsservctl: c_long = 180;
pub const __NR_getpmsg: c_long = 181;
pub const __NR_putpmsg: c_long = 182;
pub const __NR_afs_syscall: c_long = 183;
pub const __NR_tuxcall: c_long = 184;
pub const __NR_security: c_long = 185;
pub const __NR_gettid: c_long = 186;
pub const __NR_readahead: c_long = 187;
pub const __NR_setxattr: c_long = 188;
pub const __NR_lsetxattr: c_long = 189;
pub const __NR_fsetxattr: c_long = 190;
pub const __NR_getxattr: c_long = 191;
pub const __NR_lgetxattr: c_long = 192;
pub const __NR_fgetxattr: c_long = 193;
pub const __NR_listxattr: c_long = 194;
pub const __NR_llistxattr: c_long = 195;
pub const __NR_flistxattr: c_long = 196;
pub const __NR_removexattr: c_long = 197;
pub const __NR_lremovexattr: c_long = 198;
pub const __NR_fremovexattr: c_long = 199;
pub const __NR_tkill: c_long = 200;
pub const __NR_time: c_long = 201;
pub const __NR_futex: c_long = 202;
pub const __NR_sched_setaffinity: c_long = 203;
pub const __NR_sched_getaffinity: c_long = 204;
pub const __NR_set_thread_area: c_long = 205;
pub const __NR_io_setup: c_long = 206;
pub const __NR_io_destroy: c_long = 207;
pub const __NR_io_getevents: c_long = 208;
pub const __NR_io_submit: c_long = 209;
pub const __NR_io_cancel: c_long = 210;
pub const __NR_get_thread_area: c_long = 211;
pub const __NR_lookup_dcookie: c_long = 212;
pub const __NR_remap_file_pages: c_long = 216;
pub const __NR_getdents64: c_long = 217;
pub const __NR_set_tid_address: c_long = 218;
pub const __NR_restart_syscall: c_long = 219;
pub const __NR_semtimedop: c_long = 220;
pub const __NR_fadvise64: c_long = 221;
pub const __NR_timer_create: c_long = 222;
pub const __NR_timer_settime: c_long = 223;
pub const __NR_timer_gettime: c_long = 224;
pub const __NR_timer_getoverrun: c_long = 225;
pub const __NR_timer_delete: c_long = 226;
pub const __NR_clock_settime: c_long = 227;
pub const __NR_clock_gettime: c_long = 228;
pub const __NR_clock_getres: c_long = 229;
pub const __NR_clock_nanosleep: c_long = 230;
pub const __NR_exit_group: c_long = 231;
pub const __NR_tgkill: c_long = 234;
pub const __NR_utimes: c_long = 235;
pub const __NR_vserver: c_long = 236;
pub const __NR_mbind: c_long = 237;
pub const __NR_set_mempolicy: c_long = 238;
pub const __NR_get_mempolicy: c_long = 239;
pub const __NR_mq_open: c_long = 240;
pub const __NR_mq_unlink: c_long = 241;
pub const __NR_mq_timedsend: c_long = 242;
pub const __NR_mq_timedreceive: c_long = 243;
pub const __NR_mq_notify: c_long = 244;
pub const __NR_mq_getsetattr: c_long = 245;
pub const __NR_kexec_load: c_long = 246;
pub const __NR_waitid: c_long = 247;
pub const __NR_add_key: c_long = 248;
pub const __NR_request_key: c_long = 249;
pub const __NR_keyctl: c_long = 250;
pub const __NR_ioprio_set: c_long = 251;
pub const __NR_ioprio_get: c_long = 252;
pub const __NR_inotify_init: c_long = 253;
pub const __NR_inotify_add_watch: c_long = 254;
pub const __NR_inotify_rm_watch: c_long = 255;
pub const __NR_migrate_pages: c_long = 256;
pub const __NR_openat: c_long = 257;
pub const __NR_mkdirat: c_long = 258;
pub const __NR_mknodat: c_long = 259;
pub const __NR_fchownat: c_long = 260;
pub const __NR_futimesat: c_long = 261;
pub const __NR_newfstatat: c_long = 262;
pub const __NR_unlinkat: c_long = 263;
pub const __NR_renameat: c_long = 264;
pub const __NR_linkat: c_long = 265;
pub const __NR_symlinkat: c_long = 266;
pub const __NR_readlinkat: c_long = 267;
pub const __NR_fchmodat: c_long = 268;
pub const __NR_faccessat: c_long = 269;
pub const __NR_pselect6: c_long = 270;
pub const __NR_ppoll: c_long = 271;
pub const __NR_unshare: c_long = 272;
pub const __NR_set_robust_list: c_long = 273;
pub const __NR_get_robust_list: c_long = 274;
pub const __NR_splice: c_long = 275;
pub const __NR_tee: c_long = 276;
pub const __NR_sync_file_range: c_long = 277;
pub const __NR_vmsplice: c_long = 278;
pub const __NR_move_pages: c_long = 279;
pub const __NR_utimensat: c_long = 280;
pub const __NR_signalfd: c_long = 282;
pub const __NR_timerfd_create: c_long = 283;
pub const __NR_eventfd: c_long = 284;
pub const __NR_fallocate: c_long = 285;
pub const __NR_timerfd_settime: c_long = 286;
pub const __NR_timerfd_gettime: c_long = 287;
pub const __NR_accept4: c_long = 288;
pub const __NR_signalfd4: c_long = 289;
pub const __NR_eventfd2: c_long = 290;
pub const __NR_dup3: c_long = 292;
pub const __NR_pipe2: c_long = 293;
pub const __NR_inotify_init1: c_long = 294;
pub const __NR_preadv: c_long = 295;
pub const __NR_pwritev: c_long = 296;
pub const __NR_rt_tgsigqueueinfo: c_long = 297;
pub const __NR_perf_event_open: c_long = 298;
pub const __NR_recvmmsg: c_long = 299;
pub const __NR_fanotify_init: c_long = 300;
pub const __NR_fanotify_mark: c_long = 301;
pub const __NR_prlimit64: c_long = 302;
pub const __NR_name_to_handle_at: c_long = 303;
pub const __NR_open_by_handle_at: c_long = 304;
pub const __NR_clock_adjtime: c_long = 305;
pub const __NR_syncfs: c_long = 306;
pub const __NR_sendmmsg: c_long = 307;
pub const __NR_setns: c_long = 308;
pub const __NR_getcpu: c_long = 309;
pub const __NR_process_vm_readv: c_long = 310;
pub const __NR_process_vm_writev: c_long = 311;
pub const __NR_kcmp: c_long = 312;
pub const __NR_finit_module: c_long = 313;
pub const __NR_sched_setattr: c_long = 314;
pub const __NR_sched_getattr: c_long = 315;
pub const __NR_renameat2: c_long = 316;
pub const __NR_seccomp: c_long = 317;
pub const __NR_getrandom: c_long = 318;
pub const __NR_memfd_create: c_long = 319;
pub const __NR_kexec_file_load: c_long = 320;
pub const __NR_bpf: c_long = 321;
pub const __NR_execveat: c_long = 322;
pub const __NR_userfaultfd: c_long = 323;
pub const __NR_membarrier: c_long = 324;
pub const __NR_mlock2: c_long = 325;
pub const __NR_copy_file_range: c_long = 326;
pub const __NR_preadv2: c_long = 327;
pub const __NR_pwritev2: c_long = 328;
pub const __NR_pkey_mprotect: c_long = 329;
pub const __NR_pkey_alloc: c_long = 330;
pub const __NR_pkey_free: c_long = 331;
pub const __NR_statx: c_long = 332;
pub const __NR_io_pgetevents: c_long = 333;
pub const __NR_rseq: c_long = 334;
pub const __NR_uretprobe: c_long = 335;
pub const __NR_uprobe: c_long = 336;
pub const __NR_pidfd_send_signal: c_long = 424;
pub const __NR_io_uring_setup: c_long = 425;
pub const __NR_io_uring_enter: c_long = 426;
pub const __NR_io_uring_register: c_long = 427;
pub const __NR_open_tree: c_long = 428;
pub const __NR_move_mount: c_long = 429;
pub const __NR_fsopen: c_long = 430;
pub const __NR_fsconfig: c_long = 431;
pub const __NR_fsmount: c_long = 432;
pub const __NR_fspick: c_long = 433;
pub const __NR_pidfd_open: c_long = 434;
pub const __NR_clone3: c_long = 435;
pub const __NR_close_range: c_long = 436;
pub const __NR_openat2: c_long = 437;
pub const __NR_pidfd_getfd: c_long = 438;
pub const __NR_faccessat2: c_long = 439;
pub const __NR_process_madvise: c_long = 440;
pub const __NR_mount_setattr: c_long = 442;
pub const __NR_quotactl_fd: c_long = 443;
pub const __NR_landlock_create_ruleset: c_long = 444;
pub const __NR_landlock_add_rule: c_long = 445;
pub const __NR_landlock_restrict_self: c_long = 446;
pub const __NR_memfd_secret: c_long = 447;
pub const __NR_process_mrelease: c_long = 448;
pub const __NR_futex_waitv: c_long = 449;
pub const __NR_set_mempolicy_home_node: c_long = 450;
pub const __NR_cachestat: c_long = 451;
pub const __NR_fchmodat2: c_long = 452;
pub const __NR_map_shadow_stack: c_long = 453;
pub const __NR_futex_wake: c_long = 454;
pub const __NR_futex_wait: c_long = 455;
pub const __NR_futex_requeue: c_long = 456;
pub const __NR_statmount: c_long = 457;
pub const __NR_listmount: c_long = 458;
pub const __NR_lsm_get_self_attr: c_long = 459;
pub const __NR_lsm_set_self_attr: c_long = 460;
pub const __NR_lsm_list_modules: c_long = 461;
pub const __NR_mseal: c_long = 462;
pub const __NR_setxattrat: c_long = 463;
pub const __NR_getxattrat: c_long = 464;
pub const __NR_listxattrat: c_long = 465;
pub const __NR_removexattrat: c_long = 466;
pub const __NR_open_tree_attr: c_long = 467;
pub const __NR_file_getattr: c_long = 468;
pub const __NR_file_setattr: c_long = 469;
pub const __NR_listns: c_long = 470;
pub const __NR_rseq_slice_yield: c_long = 471;
pub const __NR_fchroot: c_long = 472;

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

//! [`clone`] associated types.
use core::ffi::{c_long, c_void};
use core::mem;

use crate::flags;
use crate::sys::{SysRes, SysResRaw, arch};

/// Create a child process (`clone(2)`).
///
/// After the clone, the child will pop the stack twice, the first word will be given as the
/// callback argument, and the second word is a function pointer to the callback itself.
///
/// # Safety
///
/// The stack must at least, contains 2 word, an arbitrary pointer, and a function pointer with
/// signature:
///
/// ```ignore
/// extern "C" fn(arg: *mut ()) -> !;
/// ```
#[inline]
pub unsafe fn clone(
    flags: Flags,
    stack: *mut c_void,
    parent_tid: *mut i32,
    child_tid: *mut i32,
    tls: u64,
) -> impl SysRes<()> {
    unsafe {
        let ret;
        core::arch::asm!(
            "syscall",
            "test eax, eax",    // test is `clone3` returns 0
            "jnz 2f",           // if it does, just returns

            "xor ebp, ebp",     // zero the frame address
            "pop rdi",          // pop the stack, put it in arg1 (rdi)
            "ret",              // pop the stack, jump to it

            "2:",
            inlateout("rax") arch::sys_clone::NO => ret,
            in("rdi") u64::from(flags),
            in("rsi") stack,
            in("rdx") parent_tid,
            in("r10") child_tid,
            in("r9") tls,
            lateout("rcx") _,
            lateout("r11") _,
            options(nostack, preserves_flags),
        );
        mem::transmute::<c_long, SysResRaw<_, arch::sys_clone>>(ret)
    }
}

/// Create a child process (`clone3(2)`).
///
/// After the clone, the child will pop the stack twice, the first word will be given as the
/// callback argument, and the second word is a function pointer to the callback itself.
///
/// # Safety
///
/// The stack must at least, contains 2 word, an arbitrary pointer, and a function pointer with
/// signature:
///
/// ```ignore
/// extern "C" fn(arg: *mut ()) -> !;
/// ```
#[inline]
pub unsafe fn clone3(args: *const CloneArgs, size: usize) -> impl SysRes<()> {
    unsafe {
        let ret;
        core::arch::asm!(
            "syscall",
            "test eax, eax",    // test is `clone3` returns 0
            "jnz 2f",           // if it does, just returns

            "xor ebp, ebp",     // zero the frame address
            "pop rdi",          // pop the stack, put it in arg1 (rdi)
            "ret",              // pop the stack, jump to it

            "2:",
            inlateout("rax") arch::sys_clone3::NO => ret,
            in("rdi") args,
            in("rsi") size,
            lateout("rcx") _,
            lateout("r11") _,
            options(nostack, preserves_flags),
        );
        mem::transmute::<c_long, SysResRaw<_, arch::sys_clone3>>(ret)
    }
}

// ===== CloneArgs =====

/// [`clone3`] arguments.
#[derive(Debug, Clone)]
#[repr(C)]
pub struct CloneArgs {
    /// Flags.
    pub flags: Flags,
    /// Where to store PID fd (`*mut i32`).
    pub pidfd: u64,
    /// Where to store child TID, in child's memory (`*mut pid_t`).
    pub child_tid: u64,
    /// Where to store child TID, in parent's memory (`*mut pid_t`).
    pub parent_tid: u64,
    /// Signal to deliver to parent on child termination.
    pub exit_signal: u64,
    /// Pointer to lowest byte of stack.
    pub stack: u64,
    /// Size of stack.
    pub stack_size: u64,
    /// Location of new TLS.
    pub tls: u64,
    /// Pointer to a `pid_t` array.
    pub set_tid: u64,
    /// Number of elements in `set_tid`.
    pub set_tid_size: u64,
    /// File descriptor for target cgroup.
    pub cgroup: u64,
}

impl CloneArgs {
    /// Create a child process.
    ///
    /// # Safety
    ///
    /// See [`clone3`].
    #[inline]
    pub unsafe fn clone3(&self) -> impl SysRes<()> {
        unsafe { clone3(self, size_of::<Self>()) }
    }
}

// ===== Flags =====

/// [`clone`] and [`clone3`] flags.
#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct Flags(u64);

flags::impl_bitops_simple!(Flags);

impl Flags {
    /// `CLONE_VM`
    pub const VM: Self = Self(CLONE_VM);
    /// `CLONE_FS`
    pub const FS: Self = Self(CLONE_FS);
    /// `CLONE_FILES`
    pub const FILES: Self = Self(CLONE_FILES);
    /// `CLONE_SIGHAND`
    pub const SIGHAND: Self = Self(CLONE_SIGHAND);
    /// `CLONE_PIDFD`
    pub const PIDFD: Self = Self(CLONE_PIDFD);
    /// `CLONE_PTRACE`
    pub const PTRACE: Self = Self(CLONE_PTRACE);
    /// `CLONE_VFORK`
    pub const VFORK: Self = Self(CLONE_VFORK);
    /// `CLONE_PARENT`
    pub const PARENT: Self = Self(CLONE_PARENT);
    /// `CLONE_THREAD`
    pub const THREAD: Self = Self(CLONE_THREAD);
    /// `CLONE_NEWNS`
    pub const NEWNS: Self = Self(CLONE_NEWNS);
    /// `CLONE_SYSVSEM`
    pub const SYSVSEM: Self = Self(CLONE_SYSVSEM);
    /// `CLONE_SETTLS`
    pub const SETTLS: Self = Self(CLONE_SETTLS);
    /// `CLONE_PARENT_SETTID`
    pub const PARENT_SETTID: Self = Self(CLONE_PARENT_SETTID);
    /// `CLONE_CHILD_CLEARTID`
    pub const CHILD_CLEARTID: Self = Self(CLONE_CHILD_CLEARTID);
    /// `CLONE_DETACHED`
    pub const DETACHED: Self = Self(CLONE_DETACHED);
    /// `CLONE_UNTRACED`
    pub const UNTRACED: Self = Self(CLONE_UNTRACED);
    /// `CLONE_CHILD_SETTID`
    pub const CHILD_SETTID: Self = Self(CLONE_CHILD_SETTID);
    /// `CLONE_NEWCGROUP`
    pub const NEWCGROUP: Self = Self(CLONE_NEWCGROUP);
    /// `CLONE_NEWUTS`
    pub const NEWUTS: Self = Self(CLONE_NEWUTS);
    /// `CLONE_NEWIPC`
    pub const NEWIPC: Self = Self(CLONE_NEWIPC);
    /// `CLONE_NEWUSER`
    pub const NEWUSER: Self = Self(CLONE_NEWUSER);
    /// `CLONE_NEWPID`
    pub const NEWPID: Self = Self(CLONE_NEWPID);
    /// `CLONE_NEWNET`
    pub const NEWNET: Self = Self(CLONE_NEWNET);
    /// `CLONE_IO`
    pub const IO: Self = Self(CLONE_IO);
    /// `CLONE_CLEAR_SIGHAND`
    pub const CLEAR_SIGHAND: Self = Self(CLONE_CLEAR_SIGHAND);
    /// `CLONE_INTO_CGROUP`
    pub const INTO_CGROUP: Self = Self(CLONE_INTO_CGROUP);
    /// `CLONE_AUTOREAP`
    pub const AUTOREAP: Self = Self(CLONE_AUTOREAP);
    /// `CLONE_NNP`
    pub const NNP: Self = Self(CLONE_NNP);
    /// `CLONE_PIDFD_AUTOKILL`
    pub const PIDFD_AUTOKILL: Self = Self(CLONE_PIDFD_AUTOKILL);
    /// `CLONE_EMPTY_MNTNS`
    pub const EMPTY_MNTNS: Self = Self(CLONE_EMPTY_MNTNS);
}

impl From<Flags> for u64 {
    #[inline]
    fn from(value: Flags) -> Self {
        value.0
    }
}

// ===== extern =====

// include/uapi/linux/sched.h

const CLONE_VM: u64 = 0x00000100;
const CLONE_FS: u64 = 0x00000200;
const CLONE_FILES: u64 = 0x00000400;
const CLONE_SIGHAND: u64 = 0x00000800;
const CLONE_PIDFD: u64 = 0x00001000;
const CLONE_PTRACE: u64 = 0x00002000;
const CLONE_VFORK: u64 = 0x00004000;
const CLONE_PARENT: u64 = 0x00008000;
const CLONE_THREAD: u64 = 0x00010000;
const CLONE_NEWNS: u64 = 0x00020000;
const CLONE_SYSVSEM: u64 = 0x00040000;
const CLONE_SETTLS: u64 = 0x00080000;
const CLONE_PARENT_SETTID: u64 = 0x00100000;
const CLONE_CHILD_CLEARTID: u64 = 0x00200000;
const CLONE_DETACHED: u64 = 0x00400000;
const CLONE_UNTRACED: u64 = 0x00800000;
const CLONE_CHILD_SETTID: u64 = 0x01000000;
const CLONE_NEWCGROUP: u64 = 0x02000000;
const CLONE_NEWUTS: u64 = 0x04000000;
const CLONE_NEWIPC: u64 = 0x08000000;
const CLONE_NEWUSER: u64 = 0x10000000;
const CLONE_NEWPID: u64 = 0x20000000;
const CLONE_NEWNET: u64 = 0x40000000;
const CLONE_IO: u64 = 0x80000000;

const CLONE_CLEAR_SIGHAND: u64 = 1 << 32;
const CLONE_INTO_CGROUP: u64 = 1 << 33;
const CLONE_AUTOREAP: u64 = 1 << 34;
const CLONE_NNP: u64 = 1 << 35;
const CLONE_PIDFD_AUTOKILL: u64 = 1 << 36;
const CLONE_EMPTY_MNTNS: u64 = 1 << 37;

// struct clone_args

//! `clone` syscall.
use core::ffi::c_void;

use crate::error::{self, ErrCode};
use crate::{flags, sys};

/// Create a child process.
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
    flags: CloneFlags,
    stack: *mut c_void,
    parent_tid: *mut i32,
    child_tid: *mut i32,
    tls: u64,
) -> Result<(), CloneError> {
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
            inlateout("rax") sys::sys_clone => ret,
            in("rdi") u64::from(flags),
            in("rsi") stack,
            in("rdx") parent_tid,
            in("r10") child_tid,
            in("r9") tls,
            lateout("rcx") _,
            lateout("r11") _,
            options(nostack, preserves_flags),
        );
        if ret >= 0 { Ok(()) } else { Err(CloneError(ErrCode::sys(ret))) }
    }
}

/// Create a child process.
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
pub unsafe fn clone3(args: *const CloneArgs, size: usize) -> Result<(), CloneError> {
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
            inlateout("rax") sys::sys_clone3 => ret,
            in("rdi") args,
            in("rsi") size,
            lateout("rcx") _,
            lateout("r11") _,
            options(nostack, preserves_flags),
        );
        if ret >= 0 { Ok(()) } else { Err(CloneError(ErrCode::sys(ret))) }
    }
}

// ===== CloneArgs =====

/// [`clone3`] arguments.
#[derive(Debug, Clone)]
#[repr(C)]
pub struct CloneArgs {
    /// Flags.
    pub flags: CloneFlags,
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
    pub unsafe fn clone3(&self) -> Result<(), CloneError> {
        unsafe { clone3(self, size_of::<Self>()) }
    }
}

// ===== CloneFlags =====

/// [`clone`] and [`clone3`][CloneArgs::clone3] flags.
///
/// Reference: `clone(2)`.
#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct CloneFlags(u64);

flags::impl_bitops_simple!(CloneFlags);

impl CloneFlags {
    /// Calling process and the child process run in the same memory space.
    pub const VM: Self = Self(0x00000100);
    /// Caller and the child process share the same filesystem information.
    pub const FS: Self = Self(0x00000200);
    /// Calling process and the child process share the same file descriptor table.
    pub const FILES: Self = Self(0x00000400);
    /// Calling process and the child process share the same table of signal handlers.
    pub const SIGHAND: Self = Self(0x00000800);
    /// PID fd referring to the child process is allocated and placed at a specified location in the
    /// parent's memory.
    pub const PIDFD: Self = Self(0x00001000);
    /// Trace the child also.
    pub const PTRACE: Self = Self(0x00002000);
    /// Calling process is suspended until the child releases its virtual memory resources via a
    /// call to `execve(2)` or `_exit(3)` (as with `vfork(2)`).
    pub const VFORK: Self = Self(0x00004000);
    /// Parent of the new child will be the same as that of the calling process.
    pub const PARENT: Self = Self(0x00008000);
    /// Child is placed in the same thread group as the calling process.
    pub const THREAD: Self = Self(0x00010000);
    /// Cloned child is started in a new mount namespace, initialized with a copy of the namespace
    /// of the parent.
    pub const NEWNS: Self = Self(0x00020000);
    /// Child and the calling process share a single list of System V semaphore adjustment (semadj)
    /// values (see `semop(2)`).
    pub const SYSVSEM: Self = Self(0x00040000);
    /// The TLS (Thread Local Storage) descriptor is set to `tls`.
    pub const SETTLS: Self = Self(0x00080000);
}

impl From<CloneFlags> for u64 {
    #[inline]
    fn from(value: CloneFlags) -> Self {
        value.0
    }
}

// ===== errors =====

/// An error that may occur during process cloning.
#[derive(Clone, Copy)]
pub struct CloneError(ErrCode);

error::impl_error_os_simple!(CloneError, "clone process");

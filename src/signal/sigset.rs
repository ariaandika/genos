//! [`Sigset`] associated types.
use core::{ffi, mem, result};

use crate::error::{ErrCode, SysResExt};
use crate::signal::Signo;
use crate::{error, sys};

// ===== Sigset =====

/// Signal set (`sigset_t`).
#[derive(Debug, Clone)]
#[repr(transparent)]
pub struct Sigset(sigset_t);

impl Sigset {
    /// Creates new empty [`Sigset`].
    #[inline]
    pub const fn new() -> Self {
        Self(sigemptyset())
    }

    /// Creates new full [`Sigset`], including all signal.
    #[inline]
    pub const fn fill() -> Self {
        Self(sigfillset())
    }

    pub(crate) fn as_ref(&self) -> &sigset_t {
        &self.0
    }

    /// Add signal in the set.
    #[inline]
    pub const fn add(&mut self, sig: Signo) {
        sigaddset(&mut self.0, sig.as_raw());
    }

    /// Delete signal in the set.
    #[inline]
    pub const fn delete(&mut self, sig: Signo) {
        sigdelset(&mut self.0, sig.as_raw());
    }

    /// Returns `true` if given signo is a member of the set.
    #[inline]
    pub const fn is_member(&self, sig: Signo) -> bool {
        sigismember(&self.0, sig.as_raw()) != 0
    }
}

impl Default for Sigset {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl Sigset {
    /// Returns the current value of the signal mask.
    #[inline]
    pub fn current() -> Self {
        let mut set = mem::MaybeUninit::uninit();
        // possible errors are EFAULT and EINVAL
        let _res = rt_sigprocmask(0, 0 as _, set.as_mut_ptr()).into_inner();
        debug_assert!(_res >= 0);
        unsafe { set.assume_init() }
    }

    /// Set the blocked signals to the union of the current set and this set.
    #[inline]
    pub fn block(&self) -> Result<()> {
        rt_sigprocmask(SIG_BLOCK, self, 0 as _).e2()
    }

    /// Remove the blocked signals that is in this set.
    #[inline]
    pub fn unblock(&self) -> Result<()> {
        rt_sigprocmask(SIG_UNBLOCK, self, 0 as _).e2()
    }

    /// Set the blocked signals to this set.
    #[inline]
    pub fn setmask(&self) -> Result<()> {
        rt_sigprocmask(SIG_SETMASK, self, 0 as _).e2()
    }
}

fn rt_sigprocmask(how: i32, new: *const Sigset, old: *mut Sigset) -> impl SysResExt {
    sys::call!(sys_rt_sigprocmask, how, new, old, size_of::<Sigset>())
}

// ===== Error =====

/// The result of changing blocked signals.
pub type Result<T, E = Error> = result::Result<T, E>;

/// An error that may occur when changing blocked signals.
#[derive(Clone, Copy)]
pub struct Error(ErrCode);

error::impl_error_os_simple!(Error, "change blocked signals");

// ===== extern =====

// include/uapi/asm-generic/signal-defs.h

const SIG_BLOCK: i32 = 0;
const SIG_UNBLOCK: i32 = 1;
const SIG_SETMASK: i32 = 2;

// arch/x86/include/uapi/asm/signal.h

#[expect(non_camel_case_types)]
type sigset_t = ffi::c_ulong;

// arch/x86/include/asm/signal.h

// simplified for 64 bit only

#[cfg(target_pointer_width = "64")]
const fn sigemptyset() -> sigset_t {
    0
}

#[cfg(target_pointer_width = "64")]
const fn sigfillset() -> sigset_t {
    -1i64 as u64
}

#[cfg(target_pointer_width = "64")]
const fn sigaddset(set: &mut sigset_t, sig: i32) {
    *set |= 1 << (sig - 1) as sigset_t;
}

#[cfg(target_pointer_width = "64")]
const fn sigdelset(set: &mut sigset_t, sig: i32) {
    *set &= !(1 << (sig - 1) as sigset_t);
}

#[cfg(target_pointer_width = "64")]
const fn sigismember(set: &sigset_t, sig: i32) -> u64 {
    1 & *set >> (sig - 1) as sigset_t
}

use core::mem::MaybeUninit;
use core::{ffi, mem};

use crate::signal::Signo;
use crate::sys::{self, Error, arch, optmut};

pub type SigProcResult<T> = Result<T, Error<arch::sys_rt_sigprocmask>>;

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
    pub fn new_current() -> Self {
        let mut me = MaybeUninit::uninit();
        // possible errors are EFAULT and EINVAL
        let _ = rt_sigprocmask(Self::ZERO, None, Some(&mut me));
        unsafe { me.assume_init() }
    }

    /// Get the current value of the signal mask.
    #[inline]
    pub fn current(&mut self) {
        // possible errors are EFAULT and EINVAL
        let _ = rt_sigprocmask(Self::ZERO, None, optuninit(Some(self)));
    }

    /// Set the blocked signals to the union of the current set and this set.
    #[inline]
    pub fn block(&self) -> SigProcResult<()> {
        rt_sigprocmask(Self::BLOCK, Some(self), None)
    }

    /// Remove the blocked signals that is in this set.
    #[inline]
    pub fn unblock(&self) -> SigProcResult<()> {
        rt_sigprocmask(Self::UNBLOCK, Some(self), None)
    }

    /// Set the blocked signals to this set.
    #[inline]
    pub fn setmask(&self) -> SigProcResult<()> {
        rt_sigprocmask(Self::SETMASK, Some(self), None)
    }
}

fn optref<T>(opt: Option<&T>) -> *const T {
    // this will generate to just a `mov`
    opt.map_or(core::ptr::null_mut(), |e| e as *const _)
}

fn optuninit<T>(opt: Option<&mut T>) -> Option<&mut MaybeUninit<T>> {
    unsafe { mem::transmute(opt) }
}

/// Examine and change blocked signals (`rt_sigprocmask(2)`).
#[inline]
pub fn rt_sigprocmask(
    how: SigHow,
    new: Option<&Sigset>,
    old: Option<&mut MaybeUninit<Sigset>>,
) -> SigProcResult<()> {
    sys::call!(sys_rt_sigprocmask, how.0, optref(new), optmut(old), size_of::<Sigset>())
}

// ===== SigHow =====

/// [`rt_sigprocmask`] operation.
#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct SigHow(i32);

impl Sigset {
    const ZERO: SigHow = SigHow(0);
    /// `SIG_BLOCK`
    pub const BLOCK: SigHow = SigHow(SIG_BLOCK);
    /// `SIG_UNBLOCK`
    pub const UNBLOCK: SigHow = SigHow(SIG_UNBLOCK);
    /// `SIG_SETMASK`
    pub const SETMASK: SigHow = SigHow(SIG_SETMASK);
}

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

//! [`Vars`] associated types.
use core::{ffi, fmt};

use crate::ffi::Char;

type Head = &'static Elem;

type Elem = Option<&'static Char>;

/// Environment variables.
pub struct Vars(Head);

impl Vars {
    /// Creates [`Vars`] from raw pointer.
    ///
    /// # Safety
    ///
    /// This is only intended to be used with libc style main function argument.
    #[doc(hidden)]
    #[inline]
    pub const unsafe fn from_raw(envp: *const *const ffi::c_char) -> Self {
        unsafe { Self(envp.cast::<Elem>().as_ref_unchecked()) }
    }

    /// Retruns the inner pointer.
    #[inline]
    pub const fn as_ptr(&self) -> *const *const ffi::c_char {
        &raw const self.0 as *const _
    }

    /// Returns an iterator over the variables.
    #[inline]
    pub fn iter(&self) -> Iter {
        Iter(self.0)
    }
}

impl fmt::Debug for Vars {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self).finish()
    }
}

impl IntoIterator for Vars {
    type Item = &'static ffi::CStr;

    type IntoIter = Iter;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        Iter(self.0)
    }
}

impl IntoIterator for &Vars {
    type Item = &'static ffi::CStr;

    type IntoIter = Iter;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        Iter(self.0)
    }
}

// ===== Iterator =====

/// [`Vars`] iterator.
#[derive(Debug)]
pub struct Iter(Head);

impl Iterator for Iter {
    type Item = &'static ffi::CStr;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let next = (*self.0)?;
        self.0 = unsafe { &*(self.0 as *const Elem).add(1) };
        Some(next.as_cstr())
    }
}

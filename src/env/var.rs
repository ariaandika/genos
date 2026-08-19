//! [`Vars`] associated types.
use core::{ffi, fmt};

/// Environment variables.
#[derive(Clone)]
pub struct Vars {
    ptr: *const *const ffi::c_char,
}

impl Vars {
    /// Creates [`Vars`] from raw pointer.
    ///
    /// # Safety
    ///
    /// This is only intended to be created right at the start of the main function.
    #[inline]
    pub unsafe fn from_raw(ptr: *const *const ffi::c_char) -> Self {
        Self { ptr }
    }

    /// Returns an iterator over the variables.
    #[inline]
    pub fn iter(&self) -> Iter {
        self.into_iter()
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
        Iter { ptr: self.ptr }
    }
}

impl IntoIterator for &Vars {
    type Item = &'static ffi::CStr;

    type IntoIter = Iter;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        Iter { ptr: self.ptr }
    }
}

// ===== Iterator =====

/// [`Vars`] iterator.
#[derive(Debug)]
pub struct Iter {
    ptr: *const *const ffi::c_char,
}

impl Iterator for Iter {
    type Item = &'static ffi::CStr;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            let next = *self.ptr;
            if next.is_null() {
                return None;
            }
            self.ptr = self.ptr.add(1);
            Some(ffi::CStr::from_ptr(next))
        }
    }
}

//! [`Args`] associated types.
use core::{ffi, fmt, slice};

/// Command line arguments.
#[derive(Clone)]
pub struct Args {
    argc: i32,
    argv: *const *const ffi::c_char,
}

impl Args {
    /// Creates [`Args`] from raw pointer and length.
    ///
    /// # Safety
    ///
    /// This is only intended to be created right at the start of the main function.
    #[inline]
    pub unsafe fn from_raw_parts(argc: i32, argv: *const *const ffi::c_char) -> Self {
        Self { argc, argv }
    }

    /// Returns the raw slice of the command line arguments.
    #[inline]
    pub fn as_slice(&self) -> &'static [*const ffi::c_char] {
        unsafe { slice::from_raw_parts(self.argv, self.argc as _) }
    }

    /// Returns an iterator over the arguments.
    #[inline]
    pub fn iter(&self) -> Iter {
        self.into_iter()
    }
}

impl fmt::Debug for Args {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self).finish()
    }
}

impl IntoIterator for Args {
    type Item = &'static ffi::CStr;

    type IntoIter = Iter;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        Iter(self.as_slice().iter())
    }
}

impl IntoIterator for &Args {
    type Item = &'static ffi::CStr;

    type IntoIter = Iter;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        Iter(self.as_slice().iter())
    }
}

// ===== Iterator =====

/// Iterator of [`Args`].
#[derive(Debug)]
pub struct Iter(slice::Iter<'static, *const i8>);

impl Iterator for Iter {
    type Item = &'static ffi::CStr;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.0
            .next()
            .filter(|e| !e.is_null()) // perhaps its redundant
            .map(|e| unsafe { ffi::CStr::from_ptr(*e) })
    }
}

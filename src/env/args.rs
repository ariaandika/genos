//! [`Args`] associated types.
use core::{ffi, fmt, slice};

use crate::ffi::Char;

/// Command line arguments.
pub struct Args<'c>([Option<&'c Char>]);

impl<'c> Args<'c> {
    /// Creates [`Args`] from raw pointer and length.
    ///
    /// # Safety
    ///
    /// This is only intended to be used with libc style main function argument.
    #[doc(hidden)]
    #[inline]
    pub const unsafe fn from_raw_parts<'a>(
        argc: i32,
        argv: *const *const ffi::c_char,
    ) -> &'a Args<'c> {
        unsafe {
            let slice = slice::from_raw_parts(argv.cast(), argc as usize + 1);
            Args::from_slice_with_nul_unchecked(slice)
        }
    }

    /// Creates [`Args`] from null terminated slice of `&Char`.
    ///
    /// # Safety
    ///
    /// All element must be `Some` and the last element must be `None`.
    #[inline]
    pub const unsafe fn from_slice_with_nul_unchecked<'a>(
        slice: &'a [Option<&'c Char>],
    ) -> &'a Args<'c> {
        #[cfg(debug_assertions)]
        debug_validate(slice);
        unsafe { &*(slice as *const _ as *const Self) }
    }

    /// Returns the argument length, this does not include the null terminator.
    #[inline]
    pub const fn len(&self) -> usize {
        self.0.len() - 1
    }

    /// Returns the list of arguments as slice.
    #[inline]
    pub const fn as_slice(&self) -> &[&'c Char] {
        // SAFETY: it is guarantee that only the last element is `None` (null pointer).
        unsafe { slice::from_raw_parts(self.0.as_ptr().cast(), self.len() - 1) }
    }

    /// Returns an iterator over the arguments.
    #[inline]
    pub fn iter(&self) -> Iter<'_, 'c> {
        Iter(self.as_slice().iter())
    }
}

#[cfg(debug_assertions)]
const fn debug_validate(args: &[Option<&Char>]) {
    let mut i = 0;
    while i < args.len() - 1 {
        debug_assert!(args[i].is_some(), "`Args::from_raw_parts` contains null pointer");
        i += 1;
    }
    debug_assert!(args[i].is_none(), "`Args::from_raw_parts` is not null terminated");
}

impl fmt::Debug for Args<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Args").field(&&self.0).finish()
    }
}

// ===== Iterator =====

impl<'a, 'c> IntoIterator for &'a Args<'c> {
    type Item = &'c ffi::CStr;

    type IntoIter = Iter<'a, 'c>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

/// Iterator of [`Args`].
#[derive(Debug)]
pub struct Iter<'a, 'c>(slice::Iter<'a, &'c Char>);

impl<'a, 'c> Iterator for Iter<'a, 'c> {
    type Item = &'c ffi::CStr;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|e| e.as_cstr())
    }
}

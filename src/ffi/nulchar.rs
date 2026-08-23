use crate::ffi::{self, CStr};

/// Wrapper type around [`c_char`].
///
/// This is intended to be used as a shared reference, `&Char`, to represent thin pointer over null
/// terminated string.
///
/// This can be used to store C string in a contiguous memory, avoiding pointer type.
///
/// [`c_char`]: ffi::c_char
#[derive(Debug)]
#[repr(transparent)]
pub struct Char(ffi::c_char);

impl Char {
    /// Create new `&Char` from [`CStr`].
    #[inline]
    pub const fn new(str: &CStr) -> &Char {
        unsafe { &*str.as_ptr().cast() }
    }

    /// Cast pointer to C char.
    #[inline]
    pub const fn as_ptr(&self) -> *const ffi::c_char {
        (self as *const Self).cast()
    }

    /// Convert to [`CStr`].
    #[inline]
    pub const fn as_cstr(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.as_ptr()) }
    }
}

impl<'a> From<&'a CStr> for &'a Char {
    #[inline]
    fn from(value: &'a CStr) -> Self {
        Char::new(value)
    }
}

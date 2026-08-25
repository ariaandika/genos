use core::ffi::{self, CStr};

/// Wrapper type around [`c_char`].
///
/// This is intended to be used as a shared reference, `&Char`, to represent thin pointer over null
/// terminated string.
///
/// This can be used to store C string in a contiguous memory, avoiding pointer type.
///
/// [`c_char`]: ffi::c_char
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

impl core::fmt::Debug for Char {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut c = self.as_ptr().cast::<u8>();
        loop {
            match unsafe { *c } {
                0 => break,
                c @ b'\x01'..=b'\x7f' => write!(f, "{}", c.escape_ascii())?,
                c => write!(f, "{}", c as char)?,
            }
            c = unsafe { c.add(1) };
        }
        Ok(())
    }
}

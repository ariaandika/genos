use core::ffi::{self, CStr};

/// Wrapper type around [`c_char`].
///
/// The type `&Char` is guaranteed to be null terminated.
///
/// In contrast with [`CStr`], this has `#[repr(transparent)]` over [`c_char`], thus `&Char` have
/// the same representation as C string `const char*`.
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

// utility function like `strlen` and `strcmp` may not be added

impl<'a> From<&'a CStr> for &'a Char {
    #[inline]
    fn from(value: &'a CStr) -> Self {
        Char::new(value)
    }
}

impl core::fmt::Debug for Char {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut c = self.as_ptr().cast::<u8>();
        f.write_str("\"")?;
        loop {
            match unsafe { *c } {
                0 => break,
                c @ b'\x01'..=b'\x7f' => write!(f, "{}", c.escape_ascii())?,
                c => write!(f, "{}", c as char)?,
            }
            c = unsafe { c.add(1) };
        }
        f.write_str("\"")
    }
}

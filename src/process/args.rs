use core::{ffi, slice};

pub use Iter as Args;

/// Returns raw command line arguments
#[inline]
pub fn raw_args() -> &'static [*const i8] {
    arg::raw_args()
}

/// Returns iterator of command line arguments.
#[inline]
pub fn args() -> Iter {
    Iter::new()
}

type Inner = slice::Iter<'static, *const i8>;

/// Iterator of command line arguments.
#[derive(Debug)]
pub struct Iter(Inner);

impl Iterator for Iter {
    type Item = &'static ffi::CStr;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.0
            .next()
            .filter(|e| !e.is_null())
            .map(|e| unsafe { ffi::CStr::from_ptr(*e) })
    }
}

impl Iter {
    /// Create arguments iterator from raw parts.
    ///
    /// # Safety
    ///
    /// This is only intended to be created right at the start of the main function.
    #[inline]
    pub unsafe fn from_raw_parts(argc: i32, argv: *const *const ffi::c_char) -> Self {
        Self(unsafe { slice::from_raw_parts(argv, argc as _).iter() })
    }

    fn new() -> Self {
        Self(raw_args().iter())
    }
}

// ===== extern =====

#[cfg(all(target_os = "linux", target_env = "gnu"))]
mod arg {
    // source: dtolnay/argv
    use core::{ffi, slice};

    static mut ARGC: i32 = 0;
    static mut ARGV: *const *const ffi::c_char = 0 as _;

    #[allow(dead_code)]
    unsafe extern "C" fn capture(argc: ffi::c_int, argv: *const *const ffi::c_char) {
        unsafe {
            ARGC = argc;
            ARGV = argv;
        }
    }

    #[cfg(target_os = "linux")]
    #[unsafe(link_section = ".init_array")]
    #[used]
    static CAPTURE: unsafe extern "C" fn(ffi::c_int, *const *const ffi::c_char) = capture;

    pub fn raw_args() -> &'static [*const i8] {
        unsafe { slice::from_raw_parts(ARGV, ARGC as _) }
    }
}

#[cfg(not(any(target_os = "linux", target_env = "gnu")))]
mod arg {
    pub fn raw_args() -> &'static [*const i8] {
        panic!("raw_args() is not yet supported in other platform");
    }
}

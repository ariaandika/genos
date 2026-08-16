pub use arg::Iter;

/// Returns raw command line arguments
#[inline]
pub fn raw_args<'a>() -> &'a [*const i8] {
    arg::raw_args()
}

/// Returns iterator of command line arguments.
#[inline]
pub fn args<'a>() -> Iter<'a> {
    Iter::new()
}

// ===== extern =====

#[cfg(all(target_os = "linux", target_env = "gnu"))]
mod arg {
    // source: dtolnay/argv
    use core::{ffi, iter, slice};

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

    #[inline]
    pub fn raw_args() -> &'static [*const i8] {
        unsafe { slice::from_raw_parts(ARGV, ARGC as _) }
    }

    type Inner<'a> = iter::Map<
        iter::Filter<iter::Copied<slice::Iter<'static, *const i8>>, fn(&*const i8) -> bool>,
        fn(*const i8) -> &'a ffi::CStr,
    >;

    /// Iterator of command line arguments.
    #[derive(Debug)]
    pub struct Iter<'a>(Inner<'a>);

    impl<'a> Iterator for Iter<'a> {
        type Item = &'a ffi::CStr;

        #[inline]
        fn next(&mut self) -> Option<Self::Item> {
            self.0.next()
        }
    }

    impl<'a> Iter<'a> {
        #[inline]
        pub(super) fn new() -> Self {
            fn fun_name2(e: &*const i8) -> bool {
                !e.is_null()
            }
            fn fun_name<'a>(e: *const i8) -> &'a std::ffi::CStr {
                unsafe { ffi::CStr::from_ptr(e) }
            }
            Self(
                raw_args()
                    .iter()
                    .copied()
                    .filter(fun_name2 as _)
                    .map(fun_name),
            )
        }
    }
}

#[cfg(not(any(target_os = "linux", target_env = "gnu")))]
mod arg {
    compile_error!("raw_args() not supported");
}

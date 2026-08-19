use core::{ffi, slice};

type InnerIter = slice::Iter<'static, *const i8>;

/// Raw command line arguments.
#[derive(Debug, Clone)]
pub struct Args {
    argc: i32,
    argv: *const *const ffi::c_char,
}

impl Args {
    /// Create arguments iterator from raw parts.
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

    /// Returns iterator over command line arguments.
    #[inline]
    pub fn iter(&self) -> ArgsIter {
        self.into_iter()
    }
}

// ===== Iterator =====

impl IntoIterator for Args {
    type Item = &'static ffi::CStr;

    type IntoIter = ArgsIter;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        ArgsIter(self.as_slice().iter())
    }
}

impl IntoIterator for &Args {
    type Item = &'static ffi::CStr;

    type IntoIter = ArgsIter;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        ArgsIter(self.as_slice().iter())
    }
}

/// Iterator of command line arguments.
#[derive(Debug)]
pub struct ArgsIter(InnerIter);

impl Iterator for ArgsIter {
    type Item = &'static ffi::CStr;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.0
            .next()
            .filter(|e| !e.is_null())
            .map(|e| unsafe { ffi::CStr::from_ptr(*e) })
    }
}

/// Declare entrypoint for `#![no_std]` and `#![no_main]` binary.
///
/// Caller define closure that accept [`Args`] as input and returns `i32`.
///
/// The panic handler will simply print the panic location and message, then exit with `101`.
///
/// # Examples
///
/// ```ignore
/// main!(|args| start(args).is_err() as _);
///
/// fn start(args: Args) -> Result<(), Error> {
///     // ...
///     Ok(())
/// }
/// ```
#[macro_export]
macro_rules! main {
    (|$arg:pat_param|$e:expr) => {
        #[unsafe(no_mangle)]
        fn main(argc: i32, argv: *const *const i8) -> i32 {
            (|$arg|$e)(unsafe { genos::process::Args::from_raw_parts(argc, argv) })
        }

        #[cfg(not(test))]
        #[panic_handler]
        fn panic_me(info: &core::panic::PanicInfo) -> ! {
            let at = core::fmt::from_fn(|f| info.location().map_or(Ok(()), |l| write!(f, " at {l}")));
            genos::println!("Thread panicked{at}: {}", info.message());
            genos::process::_exit(101)
        }

        #[unsafe(no_mangle)]
        extern "C" fn rust_eh_personality() {}

        #[link(name="c")]
        unsafe extern "C" { }
    };
}

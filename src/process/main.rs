/// Declare entrypoint for `#![no_std]` and `#![no_main]` binary.
///
/// Caller must define closure as macro input that returns `i32`.
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

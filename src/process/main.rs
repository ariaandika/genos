/// Declare entrypoint for `#![no_std]` and `#![no_main]` binary.
///
/// Caller define closure that accept [`Args`] and [`Vars`] as input; and returns `i32` status code.
///
/// The panic handler will simply print the panic location and message, then exit with `101`.
///
/// # Examples
///
/// ```ignore
/// main!(|args, vars| start(args, vars).is_err() as _);
///
/// fn start(args: Args, vars: Vars) -> Result<(), Error> {
///     // ...
///     Ok(())
/// }
/// ```
///
/// [`Args`]: crate::env::Args
/// [`Vars`]: crate::env::Vars
#[macro_export]
macro_rules! main {
    (|$arg:pat_param,$env:pat_param|$e:expr) => {
        #[unsafe(no_mangle)]
        fn main(argc: i32, argv: *const *const i8, envp: *const *const i8) -> i32 {
            unsafe {
                (|$arg,$env|$e)(
                    genos::env::Args::from_raw_parts(argc, argv),
                    genos::env::Vars::from_raw(envp),
                )
            }
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

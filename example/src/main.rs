#![no_std]
#![no_main]
use core::fmt;

use genos::env::{Args, Vars};
use genos::process;

macro_rules! print {
    ($($tt:tt)*) => {{
        use core::fmt::Write;
        let _ = core::write!(genos::io::Stdout, $($tt)*);
    }};
}

macro_rules! println {
    ($($tt:tt)*) => {{
        use core::fmt::Write;
        let _ = core::writeln!(genos::io::Stdout, $($tt)*);
    }};
}

fn start(args: &Args, vars: Vars) -> Result<(), Error> {
    print!("$");
    for arg in args {
        print!(" {arg:?}");
    }
    println!();

    for var in vars {
        println!("> {var:?}");
    }

    Ok(())
}

struct Error;

impl<E: fmt::Display> From<E> for Error {
    fn from(value: E) -> Self {
        println!("{value}");
        Self
    }
}

// ===== extern =====

#[unsafe(naked)]
#[unsafe(no_mangle)]
unsafe extern "C" fn _start() -> ! {
    core::arch::naked_asm!(
        // `rsp` contains the initial stack pointer
        // `rdi` is the first argument of a function based on System V AMD64 ABI
        "mov rdi, rsp",
        // because `jmp` is used, `call` requirement must be satisfied
        //
        // x86-64 System V ABI states that the stack pointer (`rsp`) must be aligned to a
        // 16-byte boundary right before a `call` instruction is executed
        //
        // following instruction will shifts the stack alignment by 8 bytes, which satisfies the
        // `call` requirement
        "push rbp",
        // `jmp` instead of `call`
        "jmp {}",
        sym init,
    )
}

fn init(stack: *mut usize) -> ! {
    let status = unsafe {
        let argc = *stack as i32;
        let argv = stack.add(1).cast::<*const i8>();
        let envp = argv.add(argc as usize + 1);
        let args = Args::from_raw_parts(argc, argv);
        let vars = Vars::from_raw(envp);
        start(args, vars).is_err() as _
    };
    process::_exit(status)
}

#[cfg(not(test))]
#[panic_handler]
fn panic_me(info: &core::panic::PanicInfo) -> ! {
    let at = fmt::from_fn(|f| info.location().map_or(Ok(()), |l| write!(f, " at {l}")));
    println!("Thread panicked{at}: {}", info.message());
    process::_exit(101)
}

#[unsafe(no_mangle)]
extern "C" fn rust_eh_personality() {}

// rust `core` module uses some functions from `libc`
#[unsafe(no_mangle)]
unsafe extern "C" fn memset(
    ptr: *mut core::ffi::c_void,
    val: i32,
    n: usize,
) -> *mut core::ffi::c_void {
    unsafe { core::ptr::write_bytes(ptr, val as u8, n) };
    ptr
}

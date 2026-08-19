pub(crate) use arch::*;
pub(crate) use types::*;

// shared definition that may be reexported by arch specific
mod shared;

// helper types
mod types;

// architecture specific definitions

#[cfg_attr(target_arch = "x86_64", path = "x86_64.rs")]
mod arch;

#[cfg(not(target_arch = "x86_64"))]
compile_error!("this architecture is not yet supported");


/// Perform a syscall.
///
/// `call!(__NR_write, fd, ptr, len) -> isize`
/// `call!(RD, __NR_write, fd, ptr, len) -> isize`
/// `call!(NORETURN, __NR_write, fd, ptr, len) -> isize`
macro_rules! call {
    (NORETURN, $nr:ident, $a1:expr) => {
        unsafe { crate::sys::call1_noret(crate::sys::$nr,
            crate::sys::IntoArg::into_arg($a1),
        ) }
    };
    ($nr:ident) => {
        unsafe { crate::sys::call0(crate::sys::$nr) }
    };
    (RD, $nr:ident, $a1:expr) => {
        unsafe { crate::sys::call1_rd(crate::sys::$nr,
            crate::sys::IntoArg::into_arg($a1),
        ) }
    };
    ($nr:ident, $a1:expr) => {
        unsafe { crate::sys::call1(crate::sys::$nr,
            crate::sys::IntoArg::into_arg($a1),
        ) }
    };
    (RD, $nr:ident, $a1:expr, $a2:expr) => {
        unsafe { crate::sys::call2_rd(crate::sys::$nr,
            crate::sys::IntoArg::into_arg($a1),
            crate::sys::IntoArg::into_arg($a2),
        ) }
    };
    ($nr:ident, $a1:expr, $a2:expr) => {
        unsafe { crate::sys::call2(crate::sys::$nr,
            crate::sys::IntoArg::into_arg($a1),
            crate::sys::IntoArg::into_arg($a2),
        ) }
    };
    (RD, $nr:ident, $a1:expr, $a2:expr, $a3:expr) => {
        unsafe { crate::sys::call3_rd(crate::sys::$nr,
            crate::sys::IntoArg::into_arg($a1),
            crate::sys::IntoArg::into_arg($a2),
            crate::sys::IntoArg::into_arg($a3),
        ) }
    };
    ($nr:ident, $a1:expr, $a2:expr, $a3:expr) => {
        unsafe { crate::sys::call3(crate::sys::$nr,
            crate::sys::IntoArg::into_arg($a1),
            crate::sys::IntoArg::into_arg($a2),
            crate::sys::IntoArg::into_arg($a3),
        ) }
    };
    (RD, $nr:ident, $a1:expr, $a2:expr, $a3:expr, $a4:expr) => {
        unsafe { crate::sys::call4_rd(crate::sys::$nr,
            crate::sys::IntoArg::into_arg($a1),
            crate::sys::IntoArg::into_arg($a2),
            crate::sys::IntoArg::into_arg($a3),
            crate::sys::IntoArg::into_arg($a4),
        ) }
    };
    ($nr:ident, $a1:expr, $a2:expr, $a3:expr, $a4:expr) => {
        unsafe { crate::sys::call4(crate::sys::$nr,
            crate::sys::IntoArg::into_arg($a1),
            crate::sys::IntoArg::into_arg($a2),
            crate::sys::IntoArg::into_arg($a3),
            crate::sys::IntoArg::into_arg($a4),
        ) }
    };
    (RD, $nr:ident, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr) => {
        unsafe { crate::sys::call5_rd(crate::sys::$nr,
            crate::sys::IntoArg::into_arg($a1),
            crate::sys::IntoArg::into_arg($a2),
            crate::sys::IntoArg::into_arg($a3),
            crate::sys::IntoArg::into_arg($a4),
            crate::sys::IntoArg::into_arg($a5),
        ) }
    };
    ($nr:ident, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr) => {
        unsafe { crate::sys::call5(crate::sys::$nr,
            crate::sys::IntoArg::into_arg($a1),
            crate::sys::IntoArg::into_arg($a2),
            crate::sys::IntoArg::into_arg($a3),
            crate::sys::IntoArg::into_arg($a4),
            crate::sys::IntoArg::into_arg($a5),
        ) }
    };
    (RD, $nr:ident, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr) => {
        unsafe { crate::sys::call6_rd(crate::sys::$nr,
            crate::sys::IntoArg::into_arg($a1),
            crate::sys::IntoArg::into_arg($a2),
            crate::sys::IntoArg::into_arg($a3),
            crate::sys::IntoArg::into_arg($a4),
            crate::sys::IntoArg::into_arg($a5),
            crate::sys::IntoArg::into_arg($a6),
        ) }
    };
    ($nr:ident, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr) => {
        unsafe { crate::sys::call6(crate::sys::$nr,
            crate::sys::IntoArg::into_arg($a1),
            crate::sys::IntoArg::into_arg($a2),
            crate::sys::IntoArg::into_arg($a3),
            crate::sys::IntoArg::into_arg($a4),
            crate::sys::IntoArg::into_arg($a5),
            crate::sys::IntoArg::into_arg($a6),
        ) }
    };
}
pub(crate) use call;

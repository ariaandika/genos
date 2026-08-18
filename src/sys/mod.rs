#[cfg_attr(target_arch = "x86_64", path = "x86_64.rs")]
pub(crate) mod arch;

#[cfg(not(target_arch = "x86_64"))]
compile_error!("this architecture is not yet supported");

mod types;

pub(crate) use types::IntoArg;

/// `call!(__NR_write, fd, ptr, len) -> isize`
macro_rules! call {
    (NORETURN, $nr:ident, $a1:expr) => {
        unsafe { crate::sys::arch::call1_noret(crate::sys::arch::$nr,
            crate::sys::IntoArg::into_arg($a1),
        ) }
    };
    ($nr:ident) => {
        unsafe { crate::sys::arch::call0(crate::sys::arch::$nr) }
    };
    (RD, $nr:ident, $a1:expr) => {
        unsafe { crate::sys::arch::call1_rd(crate::sys::arch::$nr,
            crate::sys::IntoArg::into_arg($a1),
        ) }
    };
    ($nr:ident, $a1:expr) => {
        unsafe { crate::sys::arch::call1(crate::sys::arch::$nr,
            crate::sys::IntoArg::into_arg($a1),
        ) }
    };
    (RD, $nr:ident, $a1:expr, $a2:expr) => {
        unsafe { crate::sys::arch::call2_rd(crate::sys::arch::$nr,
            crate::sys::IntoArg::into_arg($a1),
            crate::sys::IntoArg::into_arg($a2),
        ) }
    };
    ($nr:ident, $a1:expr, $a2:expr) => {
        unsafe { crate::sys::arch::call2(crate::sys::arch::$nr,
            crate::sys::IntoArg::into_arg($a1),
            crate::sys::IntoArg::into_arg($a2),
        ) }
    };
    (RD, $nr:ident, $a1:expr, $a2:expr, $a3:expr) => {
        unsafe { crate::sys::arch::call3_rd(crate::sys::arch::$nr,
            crate::sys::IntoArg::into_arg($a1),
            crate::sys::IntoArg::into_arg($a2),
            crate::sys::IntoArg::into_arg($a3),
        ) }
    };
    ($nr:ident, $a1:expr, $a2:expr, $a3:expr) => {
        unsafe { crate::sys::arch::call3(crate::sys::arch::$nr,
            crate::sys::IntoArg::into_arg($a1),
            crate::sys::IntoArg::into_arg($a2),
            crate::sys::IntoArg::into_arg($a3),
        ) }
    };
}
pub(crate) use call;

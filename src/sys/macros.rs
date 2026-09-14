/// Perform a syscall.
///
/// This is simple, function like macro, where the 1st argument is the syscall identifier, and the
/// rest is the syscall arguments.
macro_rules! call {
    ($nr:ident, $a1:expr) => {
        unsafe { crate::sys::call1(crate::sys::$nr,
            crate::sys::IntoArg::into_arg($a1),
        ) }
    };
    ($nr:ident, $a1:expr, $a2:expr) => {
        unsafe { crate::sys::call2(crate::sys::$nr,
            crate::sys::IntoArg::into_arg($a1),
            crate::sys::IntoArg::into_arg($a2),
        ) }
    };
    ($nr:ident, $a1:expr, $a2:expr, $a3:expr) => {
        unsafe { crate::sys::call3(crate::sys::$nr,
            crate::sys::IntoArg::into_arg($a1),
            crate::sys::IntoArg::into_arg($a2),
            crate::sys::IntoArg::into_arg($a3),
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
    ($nr:ident, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr) => {
        unsafe { crate::sys::call5(crate::sys::$nr,
            crate::sys::IntoArg::into_arg($a1),
            crate::sys::IntoArg::into_arg($a2),
            crate::sys::IntoArg::into_arg($a3),
            crate::sys::IntoArg::into_arg($a4),
            crate::sys::IntoArg::into_arg($a5),
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

/// Perform a readonly syscall.
///
/// This is simple, function like macro, where the 1st argument is the syscall identifier, and the
/// rest is the syscall arguments.
///
/// In contrast with [`call!`], this syscall should not mutate userspace memory.
macro_rules! call_rd {
    ($nr:ident) => { unsafe { crate::sys::call0_rd(crate::sys::$nr) } };
    ($nr:ident, $a1:expr) => {
        unsafe { crate::sys::call1_rd(crate::sys::$nr,
            crate::sys::IntoArg::into_arg($a1),
        ) }
    };
    ($nr:ident, $a1:expr, $a2:expr) => {
        unsafe { crate::sys::call2_rd(crate::sys::$nr,
            crate::sys::IntoArg::into_arg($a1),
            crate::sys::IntoArg::into_arg($a2),
        ) }
    };
    ($nr:ident, $a1:expr, $a2:expr, $a3:expr) => {
        unsafe { crate::sys::call3_rd(crate::sys::$nr,
            crate::sys::IntoArg::into_arg($a1),
            crate::sys::IntoArg::into_arg($a2),
            crate::sys::IntoArg::into_arg($a3),
        ) }
    };
    ($nr:ident, $a1:expr, $a2:expr, $a3:expr, $a4:expr) => {
        unsafe { crate::sys::call4_rd(crate::sys::$nr,
            crate::sys::IntoArg::into_arg($a1),
            crate::sys::IntoArg::into_arg($a2),
            crate::sys::IntoArg::into_arg($a3),
            crate::sys::IntoArg::into_arg($a4),
        ) }
    };
    ($nr:ident, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr) => {
        unsafe { crate::sys::call5_rd(crate::sys::$nr,
            crate::sys::IntoArg::into_arg($a1),
            crate::sys::IntoArg::into_arg($a2),
            crate::sys::IntoArg::into_arg($a3),
            crate::sys::IntoArg::into_arg($a4),
            crate::sys::IntoArg::into_arg($a5),
        ) }
    };
    ($nr:ident, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr) => {
        unsafe { crate::sys::call6_rd(crate::sys::$nr,
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
pub(crate) use call_rd;

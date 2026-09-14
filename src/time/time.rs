use crate::ffi::Char;
use crate::sys;

/// Symbol name for [`time`] exported by vDSO.
pub const TIME_VDSO_SYM: &Char = Char::new(c"__vdso_time");

/// Get time in seconds since Epoch, `1970-01-01 00:00:00 +0000 (UTC)`.
#[inline]
pub fn time() -> u64 {
    // Y2038: lmao
    sys::call_rd!(sys_time, 0).into_inner() as _
}

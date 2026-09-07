use core::{fmt, ops};

use crate::ffi::Char;

// ===== Env =====

/// Environment variable.
#[repr(transparent)]
pub struct Env(Char);

impl ops::Deref for Env {
    type Target = Char;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Debug for Env {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <Char as fmt::Debug>::fmt(&self.0, f)
    }
}

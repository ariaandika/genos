use core::fmt;

use crate::env::{Args, EnvIter, AuxvIter};

/// Initial stack memory.
///
/// This is intended to be used as `&Stack` that points to the initial stack memory.
#[repr(transparent)]
pub struct Stack(usize);

impl Stack {
    const fn as_ptr(&self) -> *const usize {
        self as *const Self as _
    }

    /// Returns the length of process arguments.
    #[inline]
    pub const fn argc(&self) -> i32 {
        self.0 as _
    }

    /// Returns the process arguments.
    #[inline]
    pub const fn args(&self) -> &Args<'_> {
        unsafe { Args::from_raw_parts(self.argc(), self.as_ptr().add(1).cast()) }
    }

    /// Returns the environment variables iterator.
    #[inline]
    pub const fn envs(&self) -> EnvIter<'_, '_> {
        EnvIter::new(unsafe { &*self.as_ptr().add(2 + self.0).cast() })
    }

    /// Returns the auxiliary vector iterator.
    ///
    /// Note that this will reiterate environment variables. The iterator can also be obtained from
    /// [`EnvIter::into_auxv`].
    #[inline]
    pub fn auxv(&self) -> AuxvIter<'_> {
        self.envs().into_auxv()
    }
}

impl fmt::Debug for Stack {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Stack")
            .field("argc", &(self.0 as i32))
            .finish_non_exhaustive()
    }
}

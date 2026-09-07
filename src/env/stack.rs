use crate::env::{Args, Vars};

/// Initial stack memory.
///
/// This is intended to be used as `&Stack` that points to the initial stack memory.
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

    /// Returns the environment variables.
    #[inline]
    pub const fn envs(&self) -> Vars {
        unsafe { Vars::from_raw(self.as_ptr().add(2 + self.0).cast()) }
    }

    /// Returns the auxiliary vector.
    #[inline]
    pub const fn auxv_ptr(&self) -> *const usize {
        unsafe {
            let mut envp = self.as_ptr().add(2 + self.0);
            while *envp != 0 {
                envp = envp.add(1);
            }
            envp.add(1)
        }
    }
}

impl core::fmt::Debug for Stack {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Stack")
            .field("argc", &(self.0 as i32))
            .finish_non_exhaustive()
    }
}

use core::{fmt, ops};

use crate::{env::aux::AuxvIter, ffi::Char};

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

// ===== EnvIter =====

/// Environment variables iterator.
#[derive(Debug)]
#[repr(transparent)]
pub struct EnvIter<'a, 's>(&'a Option<&'s Env>);

impl<'a, 's> EnvIter<'a, 's> {
    pub(crate) const fn new(env: &'a Option<&'s Env>) -> Self {
        Self(env)
    }

    /// Consume the iterator to create [`AuxvIter`].
    ///
    /// This takes advantage of how much the iterator advanced to get the auxv entry without
    /// reiterating the environment variables.
    pub fn into_auxv(mut self) -> AuxvIter<'a> {
        for _ in &mut self {}
        unsafe { AuxvIter::new(&*(self.0 as *const Option<&Env>).add(1).cast()) }
    }
}

impl<'a, 's> Iterator for EnvIter<'a, 's> {
    type Item = &'a Env;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let current = (*self.0)?;
        self.0 = unsafe { &*(self.0 as *const Option<&Env>).add(1) };
        Some(current)
    }
}

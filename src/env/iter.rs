use core::marker::PhantomData;

use crate::env::Env;

/// A null terminated array iterator.
#[derive(Debug)]
pub struct Iter<'a, T> {
    elem: *const *const T,
    _p: PhantomData<&'a Option<&'a T>>,
}

impl<T> Iter<'_, T> {
    pub(crate) const fn new(elem: *const *const T) -> Self {
        Self { elem, _p: PhantomData }
    }
}

impl<'a> Iterator for Iter<'a, Env> {
    type Item = &'a Env;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            let current = (*self.elem).as_ref()?;
            self.elem = self.elem.add(1);
            Some(current)
        }
    }
}

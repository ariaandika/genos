use core::mem::MaybeUninit;
use core::ptr::NonNull;
use core::{alloc, cell, fmt, slice};

/// Memory arena.
pub struct Arena {
    ptr: NonNull<u8>,
    off: cell::Cell<usize>,
    cap: usize,
}

impl Arena {
    /// Creates new [`Arena`] from given raw parts.
    ///
    /// Note that caller may needs to deallocate the memory manually.
    ///
    /// # Safety
    ///
    /// See safety docs from [`slice::from_raw_parts_mut`].
    ///
    /// [`slice::from_raw_parts_mut`]: core::slice::from_raw_parts_mut
    #[inline]
    pub const unsafe fn from_raw_parts(ptr: *mut u8, len: usize) -> Self {
        let ptr = unsafe { NonNull::new_unchecked(ptr) };
        Self { ptr, off: cell::Cell::new(0), cap: len }
    }

    /// Create new [`Arena`] with zero capacity.
    #[inline]
    pub const fn empty() -> Self {
        Self { ptr: NonNull::dangling(), off: cell::Cell::new(0), cap: 0 }
    }

    /// Returns the backing buffer pointer.
    #[inline]
    pub const fn as_ptr(&self) -> *mut u8 {
        self.ptr.as_ptr()
    }

    /// Returns the total capacity of the backing buffer.
    #[inline]
    pub const fn remaining(&self) -> usize {
        self.cap - self.off.get()
    }

    /// Returns the total capacity of the backing buffer.
    #[inline]
    pub const fn capacity(&self) -> usize {
        self.cap
    }

    /// Allocate `T`.
    ///
    /// Returns `None` if there is not enough remaining memory.
    ///
    /// Note that `T` drop will not be called.
    #[inline]
    pub fn allocate<'a, T>(&self) -> Option<&'a mut MaybeUninit<T>> {
        self.alloc_inner(alloc::Layout::new::<T>())
            .map(|ptr| unsafe { &mut *ptr.as_ptr().cast() })
    }

    /// Allocate `len` slice of `T`.
    ///
    /// Returns `None` if there is not enough remaining memory.
    ///
    /// Note that `T` drop will not be called.
    #[inline]
    pub fn allocate_slice<'a, T>(&self, len: usize) -> Option<&'a mut [MaybeUninit<T>]> {
        alloc::Layout::array::<T>(len)
            .ok()
            .and_then(|ly| self.alloc_inner(ly))
            .map(|ptr| unsafe { slice::from_raw_parts_mut(ptr.as_ptr().cast(), len) })
    }

    fn alloc_inner(&self, layout: alloc::Layout) -> Option<NonNull<u8>> {
        let off = self.off.get();
        let rem = self.cap - off;
        let ptr = unsafe { self.ptr.add(off) };

        let align_off = ptr.align_offset(layout.align());
        let aligned_size = align_off + layout.size();
        if aligned_size <= rem {
            self.off.set(off + aligned_size);
            unsafe { Some(ptr.add(align_off)) }
        } else {
            None
        }
    }

    /// Clear the memory, restoring original capacity.
    ///
    /// # Safety
    ///
    /// Caller must ensure that there should not be existing reference to this memory.
    #[inline]
    pub unsafe fn clear(&self) {
        self.off.set(0);
    }
}

impl fmt::Debug for Arena {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Arena").finish_non_exhaustive()
    }
}

//! ELF hash table.
use crate::ffi::Char;

/// Hash string for ELF hash table.
#[inline]
pub const fn elf_hash_cstr(mut string: &Char) -> u32 {
    let mut hash = 0u32;
    let mut x: u32;
    unsafe {
        while *string.as_ptr() != 0 {
            hash = (hash << 4).wrapping_add(*string.as_ptr() as u32);

            x = hash & 0xF000_0000;
            if x != 0 {
                hash ^= x >> 24;
            }

            hash &= !x;
            string = &*(string as *const Char).add(1);
        }
    }
    hash
}

/// ELF Hash Table.
#[derive(Debug)]
#[repr(C)]
pub struct ELFHashTable {
    nbuckets: u32,
    nchain: u32,
}

impl ELFHashTable {
    /// Creates new [`ELFHashTable`] as a header.
    ///
    /// # Safety
    ///
    /// `ptr` must point to valid structure of the ELF hash table.
    #[inline]
    pub unsafe fn from_raw<'a>(ptr: *const u32) -> &'a Self {
        unsafe { &*ptr.cast() }
    }

    /// Returns the hash table `nbuckets`.
    #[inline]
    pub const fn nbuckets(&self) -> u32 {
        self.nbuckets
    }

    /// Returns the hash table `nchain`.
    #[inline]
    pub const fn nchain(&self) -> u32 {
        self.nchain
    }
}

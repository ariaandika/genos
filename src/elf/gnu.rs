//! GNU specific ELF feature.
use core::slice;

use crate::ffi::Char;

/// Hash bytes for GNU hash table.
#[inline]
pub const fn gnu_hash(mut bytes: &[u8]) -> u32 {
    let mut h = 5381u32;
    while let [byte, rest @ ..] = bytes {
        h = h.wrapping_mul(33).wrapping_add(*byte as u32);
        bytes = rest;
    }
    h
}

/// Hash CStr for GNU hash table.
///
/// Source: [`bfd_elf_gnu_hash`][1] or [`dl_new_hash`][2].
///
/// [1]: <https://sourceware.org/git/gitweb.cgi?p=binutils-gdb.git;a=blob;f=bfd/elf.c;h=a08e0f8ea6197f103908364665ec6e5f6c89927d;hb=HEAD#l222>
/// [2]: <https://sourceware.org/git/?p=glibc.git;a=blob;f=elf/dl-lookup.c;h=3d2369dbf2b7ca219eaf80a820e2a8e1329fbf50;hb=HEAD#l569>
#[inline]
pub const fn gnu_hash_cstr(string: &Char) -> u32 {
    let mut ptr = string.as_ptr();
    let mut h = 5381u32;
    unsafe {
        while *ptr != 0 {
            h = h.wrapping_mul(33).wrapping_add(*ptr as u32);
            ptr = ptr.add(1);
        }
    }
    h
}

/// GNU Hash Table.
#[derive(Debug)]
#[repr(C)]
pub struct GNUHashTable {
    nbuckets: u32,
    symoffset: u32,
    bloom_size: u32,
    bloom_shift: u32,
    // bloom: [u64; bloom_size],
    // buckets: [u32; nbuckets],
    // chain: [u32],
}

impl GNUHashTable {
    /// Create new [`GNUHashTable`] as a header.
    ///
    /// # Safety
    ///
    /// `ptr` must point to valid structure of the GNU hash table.
    #[inline]
    pub unsafe fn from_ptr<'a>(ptr: *const u32) -> &'a GNUHashTable {
        unsafe { &*ptr.cast() }
    }

    const fn as_ptr(&self) -> *const u32 {
        self as *const Self as *const u32
    }

    /// Returns hash table `nbuckets`.
    #[inline]
    pub const fn nbuckets(&self) -> u32 {
        self.nbuckets
    }

    /// Returns hash table `symoffset`.
    #[inline]
    pub const fn symoffset(&self) -> u32 {
        self.symoffset
    }

    /// Returns hash table `bloom_size`.
    #[inline]
    pub const fn bloom_size(&self) -> u32 {
        self.bloom_size
    }

    /// Returns hash table `bloom_shift`.
    #[inline]
    pub const fn bloom_shift(&self) -> u32 {
        self.bloom_shift
    }
}

impl GNUHashTable {
    const fn blooms_ptr(&self) -> *const u64 {
        unsafe { self.as_ptr().add(4).cast() }
    }

    /// Returns the blooms as slice.
    #[inline]
    pub const fn blooms(&self) -> &[u64] {
        unsafe { slice::from_raw_parts(self.blooms_ptr(), self.bloom_size as usize) }
    }

    /// Returns the blooms as slice.
    #[inline]
    pub const fn buckets(&self) -> &[u32] {
        let off = 4 + (self.bloom_size * 2) as usize;
        unsafe { slice::from_raw_parts(self.as_ptr().add(off).cast(), self.nbuckets as usize) }
    }

    /// Returns reference to the chains first element.
    #[inline]
    pub const fn chains(&self) -> &u32 {
        let off = 4 + ((self.bloom_size * 2) + self.nbuckets) as usize;
        unsafe { &*self.as_ptr().add(off).cast() }
    }
}

impl GNUHashTable {
    /// Returns `true` if given hash may be contained in the table.
    ///
    /// Perform a bloom filter on given hash.
    ///
    /// The hash for a string can be obtained using [`gnu_hash_cstr`].
    #[inline]
    pub const fn bloom_filter(&self, hash: u32) -> bool {
        let bloom_idx = (hash / 64) % self.bloom_size();
        // SAFETY: `bloom_idx < self.bloom_size()`
        let word = unsafe { *self.blooms_ptr().add(bloom_idx as usize) };
        let bit1 = hash & 63;
        let bit2 = (hash >> self.bloom_shift()) & 63;
        let mask = (1u64 << bit1) | (1u64 << bit2);
        (word & mask) == mask
    }
}

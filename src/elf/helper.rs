use core::slice;

use crate::elf::gnu::{self, GNUHashTable};
use crate::elf::types::{Elf64_Dyn, Elf64_Ehdr, Elf64_Phdr, Elf64_Sym, PType};
use crate::env::{AuxType, Auxiliary};
use crate::ffi::Char;

// ===== ElfFile =====

/// ELF File helper.
///
/// This struct provide methods for accessing ELF structures.
#[derive(Debug)]
#[repr(transparent)]
pub struct ElfFile<'a>(&'a Elf64_Ehdr);

impl<'a> ElfFile<'a> {
    /// Creates new [`ElfFile`].
    ///
    /// # Safety
    ///
    /// This method requires that the memory are properly placed adhere to the ELF format.
    ///
    /// The safest way is to create the reference based on value of auxiliary type
    /// `AT_SYSINFO_EHDR`.
    #[inline]
    pub const unsafe fn new(header: &'a Elf64_Ehdr) -> Self {
        Self(header)
    }

    /// Creates new [`ElfFile`] from [`Auxiliary`].
    ///
    /// This can be used when iterating auxiliary vector.
    ///
    /// # Safety
    ///
    /// This method requires that the auxiliary value contains valid pointer to memory containing
    /// that represents ELF format.
    #[inline]
    pub const unsafe fn from_aux(aux: &'a Auxiliary) -> Option<ElfFile<'a>> {
        if matches!(aux.ty(), AuxType::SYSINFO_EHDR) {
            Some(unsafe { Self(&*(aux.value() as *const _)) })
        } else {
            None
        }
    }

    /// Returns the internal pointer.
    #[inline]
    pub const fn as_ptr(&self) -> *const Elf64_Ehdr {
        self.0
    }

    /// Returns slice of program headers.
    #[inline]
    pub const fn phdrs(&self) -> &[Elf64_Phdr] {
        unsafe {
            slice::from_raw_parts(
                self.as_ptr().byte_add(self.0.e_phoff as usize).cast(),
                self.0.e_phnum as usize,
            )
        }
    }

    /// Returns `Some` if the `phdr` contains the dynamic linking segment.
    #[inline]
    pub const fn as_dynamic(&self, phdr: &Elf64_Phdr) -> Option<&[Elf64_Dyn]> {
        match phdr.p_type {
            PType::DYNAMIC => unsafe {
                Some(slice::from_raw_parts(
                    self.as_ptr().byte_add(phdr.p_offset as usize).cast(),
                    phdr.p_filesz as usize / size_of::<Elf64_Dyn>(),
                ))
            },
            _ => None,
        }
    }
}

// ===== GNUHashLookup =====

/// GNU Hash Table Lookup.
#[derive(Debug)]
pub struct GNUHashLookup<'a> {
    symtab: &'a Elf64_Sym,
    strtab: &'a Char,
    hash_table: &'a GNUHashTable,
}

impl<'a> GNUHashLookup<'a> {
    /// Creates new [`GNUHashLookup`].
    ///
    /// # Safety
    ///
    /// This method requires that the memory are properly placed adhere to the ELF format.
    #[inline]
    pub unsafe fn new(
        symtab: &'a Elf64_Sym,
        strtab: &'a Char,
        hash_table: &'a GNUHashTable,
    ) -> Self {
        Self { symtab, strtab, hash_table }
    }

    /// Perform a symbol lookup by name.
    #[inline]
    pub fn symbol(&self, name: &Char) -> Option<&Elf64_Sym> {
        // hash table lookup
        let hash = gnu::gnu_hash_cstr(name);
        if !self.hash_table.bloom_filter(hash) {
            // fast filter
            return None;
        }

        // grab the bucket value, it contains index to symtab
        let bucket_i = hash % self.hash_table.nbuckets();
        let mut symtab_i = unsafe { *self.hash_table.bucket_ptr().add(bucket_i as usize) };

        // iterate hash entry chains linearly
        let chains = self.hash_table.chains();
        loop {
            let chain = nth(chains, symtab_i - self.hash_table.symoffset());

            // compare the hash integer before strcmp for fast filter
            if (*chain | 1) == (hash | 1) {
                let sym = nth(self.symtab, symtab_i);
                let entry_name = nth(self.strtab, sym.st_name);

                if strcmp(entry_name, name) {
                    return Some(sym);
                }
            }

            // chain least significant bit indicate the end of the chain
            if *chain & 1 != 0 {
                return None;
            }

            symtab_i += 1;
        }
    }
}

// ===== helper functions =====

fn nth<T>(elem: &T, n: u32) -> &T {
    unsafe { &*(elem as *const T).add(n as usize) }
}

fn strcmp(s1: &Char, s2: &Char) -> bool {
    unsafe {
        let mut s1 = s1.as_ptr();
        let mut s2 = s2.as_ptr();
        while *s1 == *s2 {
            if (*s1 & *s2) == 0 {
                return *s1 == *s2;
            }
            s1 = s1.add(1);
            s2 = s2.add(1);
        }
        false
    }
}

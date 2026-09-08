use core::slice;

use crate::elf::types::{Elf64_Dyn, Elf64_Ehdr, Elf64_Phdr, PType};
use crate::env::{AuxType, Auxiliary};

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

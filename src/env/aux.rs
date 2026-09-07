// ===== Auxiliary =====

/// Auxiliary entry.
#[derive(Debug, Clone)]
#[repr(C)]
pub struct Auxiliary {
    ty: AuxType,
    value: usize,
}

impl Auxiliary {
    /// Returns the auxiliary type.
    #[inline]
    pub const fn ty(&self) -> AuxType {
        self.ty
    }

    /// Returns the auxiliary value.
    #[inline]
    pub const fn value(&self) -> usize {
        self.value
    }

    /// Returns `true` if the auxiliary type is [`AuxType::NULL`].
    #[inline]
    pub const fn is_null(&self) -> bool {
        self.ty.0 == AuxType::NULL.0
    }
}

// ===== AuxType =====

// source: include/uapi/linux/auxvec.h
// source: arch/x86/include/uapi/asm/auxvec.h

/// Auxiliary type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct AuxType(usize);

impl AuxType {
    /// End of vector.
    pub const NULL: Self = Self(0);
    /// Entry should be ignored.
    pub const IGNORE: Self = Self(1);
    /// File descriptor of program.
    pub const EXECFD: Self = Self(2);
    /// Program headers for program.
    pub const PHDR: Self = Self(3);
    /// Size of program header entry.
    pub const PHENT: Self = Self(4);
    /// Number of program headers.
    pub const PHNUM: Self = Self(5);
    /// System page size.
    pub const PAGESZ: Self = Self(6);
    /// Base address of the program interpreter.
    pub const BASE: Self = Self(7);
    /// Flags.
    pub const FLAGS: Self = Self(8);
    /// Entry address of the executable.
    pub const ENTRY: Self = Self(9);
    /// Program is not ELF.
    pub const NOTELF: Self = Self(10);
    /// Real uid.
    pub const UID: Self = Self(11);
    /// Effective user ID of the thread.
    pub const EUID: Self = Self(12);
    /// Real group ID of the thread.
    pub const GID: Self = Self(13);
    /// Effective group ID of the thread.
    pub const EGID: Self = Self(14);
    /// String identifying CPU for optimizations.
    pub const PLATFORM: Self = Self(15);
    /// Arch dependent hints at CPU capabilities.
    pub const HWCAP: Self = Self(16);
    /// Frequency with which `times(2)` counts.
    pub const CLKTCK: Self = Self(17);
    /// Secure mode boolean.
    pub const SECURE: Self = Self(23);
    /// A pointer to a string (PowerPC and MIPS only), may differ from `AT_PLATFORM`.
    pub const BASE_PLATFORM: Self = Self(24);
    /// Address of 16 random bytes.
    pub const RANDOM: Self = Self(25);
    /// Extension of `AT_HWCAP`.
    pub const HWCAP2: Self = Self(26);
    /// Rseq supported feature size.
    pub const RSEQ_FEATURE_SIZE: Self = Self(27);
    /// Rseq allocation alignment.
    pub const RSEQ_ALIGN: Self = Self(28);
    /// Extension of HWCAP.
    pub const HWCAP3: Self = Self(29);
    /// Extension of HWCAP.
    pub const HWCAP4: Self = Self(30);
    /// Pointer to a string containing the pathname used to execute the program.
    pub const EXECFN: Self = Self(31);
    /// The entry point to the system call function in the vDSO.
    ///
    /// Not present/needed on all architectures (e.g., absent on x86-64).
    pub const SYSINFO: Self = Self(32);
    /// Address of a page containing the virtual Dynamic Shared Object (vDSO).
    pub const SYSINFO_EHDR: Self = Self(33);
    /// Minimal stack size for signal delivery.
    pub const MINSIGSTKSZ: Self = Self(51);
}

// ===== AuxvIter =====

/// Auxiliary vector iterator.
#[derive(Debug)]
pub struct AuxvIter<'a>(&'a Auxiliary);

impl<'a> AuxvIter<'a> {
    pub(crate) fn new(auxv: &'a Auxiliary) -> Self {
        Self(auxv)
    }
}

impl<'a> Iterator for AuxvIter<'a> {
    type Item = &'a Auxiliary;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let current = self.0;
        if current.is_null() {
            return None;
        }
        self.0 = unsafe { &*(current as *const Auxiliary).add(1) };
        Some(current)
    }
}

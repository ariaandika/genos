//! ELF Basic Types.
#![expect(non_camel_case_types)]

// source: include/uapi/linux/elf.h

/// Unsigned program address.
pub type Elf64_Addr = u64;

/// Unsigned section index.
pub type Elf64_SHalf = i16;

/// Unsigned file offset.
pub type Elf64_Off = u64;

/// Unsigned version symbol information.
pub type Elf64_Versym = u16;

/// `u16`
pub type Elf64_Half = u16;

/// `i32`
pub type Elf64_Sword = i32;

/// `u32`
pub type Elf64_Word = u32;

/// `u64`
pub type Elf64_Xword = u64;

/// `i64`
pub type Elf64_Sxword = i64;

// ===== Elf64_Ehdr =====

const EI_NIDENT: usize = 16;

/// ELF Header.
///
/// Reference: `elf(5)`
#[derive(Debug, Clone)]
#[repr(C)]
pub struct Elf64_Ehdr {
    /// Specifies how to interpret the file, independent of the processor or the file's remaining
    /// contents.
    pub e_ident: [u8; EI_NIDENT],
    /// Object file type.
    pub e_type: u16,
    /// Required architecture for an individual file.
    pub e_machine: u16,
    /// File version.
    pub e_version: u32,
    /// Virtual address to which the system first transfers control, thus starting the process.
    ///
    /// If the file has no associated entry point, this member holds zero.
    pub e_entry: Elf64_Addr,
    /// Program header table's file offset in bytes.
    ///
    /// If the file has no program header table, this member holds zero.
    pub e_phoff: Elf64_Off,
    /// Section header table's file offset in bytes.
    ///
    /// If the file has no section header table, this member holds zero.
    pub e_shoff: Elf64_Off,
    /// Processor-specific flags associated with the file.
    pub e_flags: u32,
    /// ELF Header's size in bytes.
    pub e_ehsize: u16,
    /// Size in bytes of one entry in the file's program header table.
    ///
    /// All entries are the same size.
    pub e_phentsize: u16,
    /// Number of entries in the program header table.
    ///
    /// Thus the product of `e_phentsize` and `e_phnum` gives the table's size in bytes. If a file
    /// has no program header, `e_phnum` holds the value zero.
    pub e_phnum: u16,
    /// Sections header's size in bytes.
    ///
    /// A section header is one entry in the section header table; all entries are the same size.
    pub e_shentsize: u16,
    /// Number of entries in the section header table.
    pub e_shnum: u16,
    /// Section header table index of the entry associated with the section name string table.
    pub e_shstrndx: u16,
}

// ===== Elf64_Phdr =====

/// Program Header.
///
/// Reference: `elf(5)`
#[derive(Debug, Clone)]
#[repr(C)]
pub struct Elf64_Phdr {
    /// Kind of segment this array element describes.
    pub p_type: PType,
    /// Bit mask of flags relevant to the segment.
    pub p_flags: u32,
    /// Offset from the beginning of the file at which the first byte of the segment resides.
    pub p_offset: Elf64_Off,
    /// Virtual address at which the first byte of the segment resides in memory.
    pub p_vaddr: Elf64_Addr,
    /// Reserved for segment's physical address.
    pub p_paddr: Elf64_Addr,
    /// Number of bytes in the file image of the segment.
    pub p_filesz: u64,
    /// Number of bytes in the memory image of the segment.
    pub p_memsz: u64,
    /// Value to which the segments are aligned in memory and in the file.
    pub p_align: u64,
}

// ===== PType =====

/// [`Elf64_Phdr`] type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct PType(u32);

impl From<PType> for u32 {
    #[inline]
    fn from(value: PType) -> Self {
        value.0
    }
}

impl PType {
    /// The array element is unused and the other members values are undefined.
    pub const NULL: Self = Self(0);
    /// The array element specifies a loadable segment, described by `p_filesz` and `p_memsz`.
    pub const LOAD: Self = Self(1);
    /// The array element specifies dynamic linking information.
    pub const DYNAMIC: Self = Self(2);
    /// The array element specifies the location and size of a null-terminated pathname to invoke
    /// as an interpreter.
    pub const INTERP: Self = Self(3);
    /// The array element specifies the location of notes (`Elf64_Nhdr`).
    pub const NOTE: Self = Self(4);
    /// This segment type is reserved but has unspecified semantics.
    pub const SHLIB: Self = Self(5);
    /// The array element, if present, specifies the location and size of the program header table
    /// itself, both in the file and in the memory image of the program.
    pub const PHDR: Self = Self(6);
    /// Thread local storage segment.
    pub const TLS: Self = Self(7);
    /// Values in the inclusive range [`PT_LOPROC`, `PT_HIPROC`] are  reserved  for  processor-
    /// specific semantics.
    pub const LOPROC: Self = Self(0x70000000);
    /// Values in the inclusive range [`PT_LOPROC`, `PT_HIPROC`] are  reserved  for  processor-
    /// specific semantics.
    pub const HIPROC: Self = Self(0x7fffffff);

    // /// OS-specific.
    // pub const LOOS: Self = Self(0x60000000);
    // /// OS-specific.
    // pub const HIOS: Self = Self(0x6fffffff);
    // pub const GNU_EH_FRAME: Self = Self(Self::LOOS.0 + 0x474e550);
    // pub const GNU_STACK: Self = Self(Self::LOOS.0 + 0x474e551);
    // pub const GNU_RELRO: Self = Self(Self::LOOS.0 + 0x474e552);
    // pub const GNU_PROPERTY: Self = Self(Self::LOOS.0 + 0x474e553);
}

// ===== Elf64_Dyn =====

/// Dynamic tags.
#[derive(Debug, Clone)]
#[repr(C)]
pub struct Elf64_Dyn {
    /// Tag representing the `d_un` interpretation.
    pub d_tag: DynTag,
    /// Tag value.
    pub d_un: Elf64_Addr,
}

// ===== DynTag =====

/// [`Elf64_Dyn`] tags.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct DynTag(Elf64_Sxword);

impl From<DynTag> for Elf64_Sxword {
    #[inline]
    fn from(value: DynTag) -> Self {
        value.0
    }
}

impl DynTag {
    /// Marks end of dynamic section.
    pub const NULL: Self = Self(0);
    /// String table offset to name of a needed library.
    pub const NEEDED: Self = Self(1);
    /// Size in bytes of PLT relocation entries.
    pub const PLTRELSZ: Self = Self(2);
    /// Address of PLT and/or GOT.
    pub const PLTGOT: Self = Self(3);
    /// Address of symbol hash table
    pub const HASH: Self = Self(4);
    /// Address of string table.
    pub const STRTAB: Self = Self(5);
    /// Address of symbol table.
    pub const SYMTAB: Self = Self(6);
    /// Address of Rela relocation table.
    pub const RELA: Self = Self(7);
    /// Size in bytes of the Rela relocation table.
    pub const RELASZ: Self = Self(8);
    /// Size in bytes of a Rela relocation table entry.
    pub const RELAENT: Self = Self(9);
    /// Size in bytes of string table.
    pub const STRSZ: Self = Self(10);
    /// Size in bytes of a symbol table entry.
    pub const SYMENT: Self = Self(11);
    /// Address of the initialization function
    pub const INIT: Self = Self(12);
    /// Address of the termination function.
    pub const FINI: Self = Self(13);
    /// String table offset to name of shared object.
    pub const SONAME: Self = Self(14);
    /// String table offset to search path for direct and indirect library dependencies.
    pub const RPATH: Self = Self(15);
    /// Alert linker to search this shared object before the executable for symbols.
    pub const SYMBOLIC: Self = Self(16);
    /// Address of Rel relocation table.
    pub const REL: Self = Self(17);
    /// Size in bytes of Rel relocation table.
    pub const RELSZ: Self = Self(18);
    /// Size in bytes of a Rel table entry.
    pub const RELENT: Self = Self(19);
    /// Type of relocation entry to which the PLT refers (Rela or Rel).
    pub const PLTREL: Self = Self(20);
    /// Undefined use for debugging.
    pub const DEBUG: Self = Self(21);
    /// Absence of this entry indicates that no relocation entries should apply to a nonwritable
    /// segment.
    pub const TEXTREL: Self = Self(22);
    /// Address of relocation entries associated solely with the PLT.
    pub const JMPREL: Self = Self(23);
    /// Values in the inclusive range [DT_LOPROC, DT_HIPROC] are reserved for processor-specific
    /// semantics.
    pub const DT_HIPROC: Self = Self(0x7fffffff);
    /// Values in the inclusive range [DT_LOPROC, DT_HIPROC] are reserved for processor-specific
    /// semantics.
    pub const DT_LOPROC: Self = Self(0x70000000);
    /// Better hash table for the ELF used by GNU systems in GNU-compatible software.
    pub const GNU_HASH: Self = Self(0x6ffffef5);

    // DT_BIND_NOW Instruct dynamic linker to process all relocations before transferring control to the executable
    // DT_RUNPATH  String table offset to search path for direct library dependencies

    // const DT_ENCODING	32
    // const OLD_DT_LOOS	0x60000000
    // const DT_LOOS		0x6000000d
    // const DT_HIOS		0x6ffff000
    // const DT_VALRNGLO	0x6ffffd00
    // const DT_VALRNGHI	0x6ffffdff
    // const DT_ADDRRNGLO	0x6ffffe00
    // const DT_ADDRRNGHI	0x6ffffeff
    // const DT_VERSYM	0x6ffffff0
    // const DT_RELACOUNT	0x6ffffff9
    // const DT_RELCOUNT	0x6ffffffa
    // const DT_FLAGS_1	0x6ffffffb
    // const DT_VERDEF	0x6ffffffc
    // const DT_VERDEFNUM	0x6ffffffd
    // const DT_VERNEED	0x6ffffffe
    // const DT_VERNEEDNUM	0x6fffffff
    // const OLD_DT_HIOS     0x6fffffff
}

// ===== Elf64_Sym =====

/// String and symbol tables.
#[derive(Debug, Clone)]
#[repr(C)]
pub struct Elf64_Sym {
    /// Index into the object files symbol string table.
    ///
    /// If the value is zero, the symbol has no name.
    pub st_name: u32,
    /// Symbol type.
    pub st_info: u8,
    /// Symbol visibility.
    pub st_other: u8,
    /// Relevant section header table index.
    pub st_shndx: u16,
    /// Value of the associated symbol.
    pub st_value: Elf64_Addr,
    /// Symbols associated sizes.
    pub st_size: u64,
}

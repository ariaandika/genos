#![no_std]
#![no_main]
#![allow(unsafe_op_in_unsafe_fn)]
use genos::elf::gnu::GNUHashTable;
use genos::elf::hash::ELFHashTable;
use genos::elf::types::{DynTag, Elf64_Dyn, Elf64_Sym, PType};
use genos::elf::{ElfFile, gnu};
use genos::env::Stack;
use genos::error::ErrCode;
use genos::ffi::Char;
use genos::process;

macro_rules! print {
    ($($tt:tt)*) => {{
        use core::fmt::Write;
        let _ = core::write!(genos::io::Stdout, $($tt)*);
    }};
}

macro_rules! println {
    ($($tt:tt)*) => {{
        use core::fmt::Write;
        let _ = core::writeln!(genos::io::Stdout, $($tt)*);
    }};
}

// naked function because compiler generate function prologue that pushes old base pointer
#[unsafe(naked)]
#[unsafe(no_mangle)]
unsafe extern "C" fn _start() -> ! {
    core::arch::naked_asm!(
        // `rsp` contains the initial stack pointer
        // `rdi` is the first argument of a function based on System V AMD64 ABI
        "mov rdi, rsp",
        // because `jmp` is used, `call` requirement must be satisfied
        //
        // x86-64 System V ABI states that the stack pointer (`rsp`) must be aligned to a
        // 16-byte boundary right before a `call` instruction is executed
        //
        // following instruction will shifts the stack alignment by 8 bytes, which satisfies the
        // `call` requirement
        "push rbp",
        // `jmp` instead of `call`
        "jmp {}",
        sym init,
    )
}

unsafe extern "C" fn init(stack: &Stack) -> ! {
    print!("$");
    for arg in stack.args() {
        print!(" {arg:?}");
    }
    println!();

    let mut envs = stack.envs();

    for var in &mut envs {
        println!("> {var:?}");
    }

    // ===== auxv =====

    let auxv = envs.into_auxv();
    let mut elf = None;
    for aux in auxv {
        println!("{aux:?}");
        if let Some(vdso) = ElfFile::from_aux(aux) {
            elf = Some(vdso);
        }
    }

    // ===== vdso =====

    match elf {
        Some(elf) => {
            if let Err(err) = vdso(elf) {
                println!("vdso inspection failed: {err}")
            }
        }
        _ => println!("vdso is not available"),
    }

    process::_exit(0)
}

type VdsoGetRandom = extern "C" fn(*mut u8, usize, u32) -> isize;

unsafe fn vdso(elf: ElfFile) -> Result<(), &'static str> {
    let load = elf
        .phdrs()
        .iter()
        .find(|phdr| phdr.p_type == PType::LOAD)
        .ok_or("no `phdr` with PT_LOAD")?;
    assert_eq!(load.p_vaddr, 0);

    // search phdr for the dynamic linking segment (PT_DYNAMIC)
    let dynamics = elf
        .phdrs()
        .iter()
        .find_map(|phdr| elf.as_dynamic(phdr))
        .ok_or("no `phdr` with dynamic section")?;

    // search for dynamic linking symtab, strtab, and gnu hash table
    let mut symtab = None::<&Elf64_Sym>;
    let mut strtab = None::<&Char>;
    let mut sym_hash = None::<&u32>;
    let mut hash_table = None::<&u32>;

    for Elf64_Dyn { d_tag, d_un } in dynamics {
        let value = *d_un as usize;
        match *d_tag {
            DynTag::NULL => break,
            DynTag::HASH => sym_hash = Some(&*elf.as_ptr().byte_add(value).cast()),
            DynTag::STRTAB => strtab = Some(&*elf.as_ptr().byte_add(value).cast()),
            DynTag::SYMTAB => symtab = Some(&*elf.as_ptr().byte_add(value).cast()),
            DynTag::GNU_HASH => hash_table = Some(&*elf.as_ptr().byte_add(value).cast()),
            _ => {}
        }
    }

    let symtab = symtab.ok_or("no symtab")?;
    let strtab = strtab.ok_or("no strtab")?;
    let hash_table = GNUHashTable::from_ptr(hash_table.ok_or("no GNU hash table")?);

    // `DT_HASH` is not always present, in favor of `DT_GNU_HASH`
    if let Some(sym_hash) = sym_hash {
        let sym_hash = ELFHashTable::from_raw(sym_hash);
        let syms = core::slice::from_raw_parts(symtab, sym_hash.nchain() as usize);
        for sym in syms {
            let entry_name = nth(strtab, sym.st_name);
            println!("[SYMBOL]: {entry_name:?}");
        }
    }

    let search = VdsoSearch { symtab, strtab, hash_table };
    let Some(sym) = search.search_symbol(c"__vdso_getrandom".into()) else {
        return Err("`getrandom` vdso not available");
    };

    // ===== use the vdso =====

    let getrandom = elf.as_ptr().byte_add(sym.st_value as _);
    let getrandom = core::mem::transmute::<*const _, VdsoGetRandom>(getrandom);
    let mut buf = [0; 8];
    let res = getrandom(buf.as_mut_ptr(), buf.len(), 0);
    if res < 0 {
        panic!("cannot perform `getrandom`: {}", ErrCode::new(-res as _));
    }
    println!("RANDOM: {buf:?}");
    Ok(())
}

struct VdsoSearch<'a> {
    symtab: &'a Elf64_Sym,
    strtab: &'a Char,
    hash_table: &'a GNUHashTable,
}

impl<'a> VdsoSearch<'a> {
    fn search_symbol(&self, name: &Char) -> Option<&Elf64_Sym> {
        // hash table lookup
        let hash = gnu::gnu_hash_cstr(name);
        if !self.hash_table.bloom_filter(hash) {
            // fast filter
            return None;
        }

        // grab the bucket value, it contains index to symtab
        let bucket_i = hash % self.hash_table.nbuckets();
        let mut symtab_i = self.hash_table.buckets()[bucket_i as usize];

        // iterate hash entry chains linearly
        let chains = self.hash_table.chains();
        loop {
            let chain = nth(chains, symtab_i - self.hash_table.symoffset());

            // chain least significant bit indicate the end of the chain
            if *chain & 1 != 0 {
                return None;
            }

            // compare the hash integer before strcmp for fast filter
            if (*chain | 1) == (hash | 1) {
                let sym = nth(self.symtab, symtab_i);
                let entry_name = nth(self.strtab, sym.st_name);

                if strcmp(entry_name, name) {
                    return Some(sym);
                }
            };

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

// ===== extern =====

#[cfg(not(test))]
#[panic_handler]
fn panic_me(info: &core::panic::PanicInfo) -> ! {
    let at = core::fmt::from_fn(|f| info.location().map_or(Ok(()), |l| write!(f, " at {l}")));
    println!("Thread panicked{at}: {}", info.message());
    process::_exit(101)
}

#[unsafe(no_mangle)]
extern "C" fn rust_eh_personality() {}

#[cfg(debug_assertions)] // `--release` seems work fine
#[unsafe(no_mangle)]
unsafe extern "C" fn memset(
    ptr: *mut core::ffi::c_void,
    val: i32,
    n: usize,
) -> *mut core::ffi::c_void {
    let mut p = ptr.cast::<u8>();
    for _ in 0..n {
        unsafe {
            p.write(val as u8);
            p = p.add(1);
        }
    }
    ptr
}

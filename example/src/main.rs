#![no_std]
#![no_main]
#![allow(unsafe_op_in_unsafe_fn)]
use genos::elf::gnu::GNUHashTable;
use genos::elf::hash::ELFHashTable;
use genos::elf::types::{DynTag, Elf64_Dyn, Elf64_Sym, PType};
use genos::elf::{ElfFile, GNUHashLookup};
use genos::env::Stack;
use genos::ffi::Char;
use genos::process;
use genos::time::TIME_VDSO_SYM;

mod clone;

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

use println;

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

    if let Some(elf) = elf {
        if let Err(err) = vdso(elf) {
            println!("vdso inspection failed: {err}")
        }
    } else {
        println!("vdso is not available")
    }

    clone::clone3_example();
    process::exit(0)
}

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
            let name = unsafe { &*(strtab as *const Char).add(sym.st_name as usize) };
            println!("[SYMBOL]: {name:?}");
        }
    }

    let table = GNUHashLookup::new(symtab, strtab, hash_table);
    let Some(sym) = table.symbol(TIME_VDSO_SYM) else {
        return Err("`getrandom` vdso not available");
    };

    // ===== use the vdso =====

    let ffi = elf.as_ptr().byte_add(sym.st_value as _);
    let time = core::mem::transmute::<*const _, extern "C" fn(usize) -> usize>(ffi);
    let time = time(0);

    println!("[TIME]: {time}");

    Ok(())
}

// ===== extern =====

#[cfg(not(test))]
#[panic_handler]
fn panic_me(info: &core::panic::PanicInfo) -> ! {
    let at = core::fmt::from_fn(|f| info.location().map_or(Ok(()), |l| write!(f, " at {l}")));
    println!("Thread panicked{at}: {}", info.message());
    process::exit(101)
}

#[unsafe(no_mangle)]
extern "C" fn rust_eh_personality() {}

// `compiler_builtins`

#[unsafe(no_mangle)]
unsafe extern "C" fn memset(
    dest: *mut core::ffi::c_void,
    c: core::ffi::c_int,
    count: usize,
) -> *mut core::ffi::c_void {
    core::arch::asm!(
        "repe stosb %al, (%rdi)",
        inout("rcx") count => _,
        inout("rdi") dest => _,
        inout("al") c as u8 => _,
        options(att_syntax, nostack, preserves_flags)
    );
    dest
}

#![no_std]
#![no_main]
#![allow(unsafe_op_in_unsafe_fn)]
use genos::elf::types::{DynTag, Elf64_Dyn, Elf64_Sym, PType};
use genos::elf::{ElfFile, gnu};
use genos::env::{AuxType, Stack};
use genos::ffi::{self, Char};
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

    let auxv = envs.into_auxv();
    let mut getrandom = None;

    for aux in auxv {
        println!("{aux:?}");
        if aux.ty() == AuxType::SYSINFO_EHDR {
            let vdso_base = aux.value() as _;
            let sym = match find_vdso(vdso_base, c"__vdso_getrandom".into()) {
                Ok(ok) => ok,
                Err(err) => {
                    println!("Lmao {err}");
                    process::_exit(2);
                }
            };
            getrandom = Some(core::mem::transmute::<usize, VdsoGetRandom>(
                sym.st_value as usize + vdso_base as usize,
            ));
        }
    }

    if let Some(getrandom) = getrandom {
        let mut buf = [0; 8];
        println!("INIT: {buf:?}");
        let res = getrandom(buf.as_mut_ptr(), buf.len(), 0);
        assert_ne!(res, -1);
        println!("RANDOM: {buf:?}");
    } else {
        println!("`getrandom` vdso is not available");
    }

    process::_exit(0)
}

type VdsoGetRandom = extern "C" fn(*mut u8, usize, u32) -> isize;

unsafe fn find_vdso(vdso_base: *const u64, target: &ffi::Char) -> Result<Elf64_Sym, &'static str> {
    // the base pointer
    let elf = ElfFile::new(&*vdso_base.cast());

    // search phdr that contains dynamic linking
    let load = elf
        .phdrs()
        .iter()
        .find(|phdr| phdr.p_type == PType::LOAD)
        .ok_or("no `phdr` with PT_LOAD")?;
    assert_eq!(load.p_vaddr, 0);

    // search phdr that contains dynamic linking
    let dynamics = elf
        .phdrs()
        .iter()
        .find_map(|phdr| elf.as_dynamic(phdr))
        .ok_or("no `phdr` with dynamic section")?;

    // search for dynamic linking symtab and strtab
    let mut symtab: Option<&Elf64_Sym> = None;
    let mut strtab: Option<&Char> = None;
    let mut hash: Option<&u32> = None;

    for Elf64_Dyn { d_tag, d_un } in dynamics {
        let value = *d_un as usize;
        match *d_tag {
            DynTag::NULL => break,
            DynTag::STRTAB => strtab = Some(&*elf.as_ptr().byte_add(value).cast()),
            DynTag::SYMTAB => symtab = Some(&*elf.as_ptr().byte_add(value).cast()),
            DynTag::GNU_HASH => hash = Some(&*elf.as_ptr().byte_add(value).cast()),
            _ => {}
        }
    }

    let strtab = strtab.ok_or("no strtab")?;
    let symtab = symtab.ok_or("no symtab")?;

    let hash_table = gnu::GNUHashTable::from_ptr(hash.ok_or("no GNU hash")?);
    let buckets = hash_table.buckets();
    let hash = gnu::gnu_hash_cstr(target);
    if !hash_table.bloom_filter(hash) {
        return Err("symbol bloom filter false");
    }

    unsafe fn next<T>(elem: &T, n: usize) -> &T {
        &*(elem as *const T).add(n)
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

    let chains = hash_table.chain_ptr();

    let bucket_i = hash % hash_table.nbuckets();
    let mut symtab_i = *buckets.as_ptr().add(bucket_i as usize);

    loop {
        let sym = next(symtab, symtab_i as _);
        let name = next(strtab, sym.st_name as usize);
        let chain = chains.add((symtab_i - hash_table.symoffset()) as usize);

        if (*chain | 1) == (hash | 1) && strcmp(name, target) {
            return Ok(sym.clone());
        };

        if *chain & 1 != 0 {
            return Err("target symbol not found");
        }
        symtab_i += 1;
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

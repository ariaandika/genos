use genos::ffi::Char;
use genos::process::{CloneArgs, clone, exit};

use crate::println;

extern "C" fn thread(name: &Char) -> ! {
    println!("{:?}", name);
    exit(0);
}

static NAME: &Char = Char::new(c"welcome");

pub fn clone3_example() {
    use clone::Flags as C;

    let flags = C::VM | C::FS | C::FILES | C::SIGHAND | C::THREAD;

    let mut stack = [0usize; 512];
    let (stack, args) = stack.split_last_chunk_mut().unwrap();

    *args = [NAME.as_ptr() as _, thread as *const () as _];

    println!("[P] {}", args.as_ptr() as usize);

    let clone = CloneArgs {
        flags,
        pidfd: 0,
        child_tid: 0,
        parent_tid: 0,
        exit_signal: 0,
        stack: stack.as_mut_ptr() as u64,
        stack_size: size_of_val(stack) as u64,
        tls: 0,
        set_tid: 0,
        set_tid_size: 0,
        cgroup: 0,
    };
    unsafe { clone.clone3().unwrap() };

    nanosleep(1);
    println!("[P] {}", args.as_ptr() as usize);
}

fn nanosleep(sec: usize) -> isize {
    let s = [sec, 0];
    unsafe {
        let ret;
        core::arch::asm!(
            "syscall",
            inlateout("rax") 35usize => ret,
            in("rdi") s.as_ptr(),
            in("rsi") 0usize,
            lateout("rcx") _,
            lateout("r11") _,
            options(nostack, preserves_flags)
        );
        ret
    }
}

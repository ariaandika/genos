use core::mem::MaybeUninit;

use genos::event::Eventfd;
use genos::event::poll::Pollfd;
use genos::fd::{AsFd, BorrowedFd};
use genos::ffi::Char;
use genos::process::{CloneArgs, clone, exit};
use genos::{arch, io};

use crate::println;

extern "C" fn thread(data: &[usize; 2]) -> ! {
    let [fd, name] = data;
    let tls = unsafe { arch::rdfsbase() } as *mut u8;
    let mut fs_base = 0 as _;

    arch::ArchOp::GET_FS.arch_prctl(&raw mut fs_base as _).unwrap();

    let name = unsafe { &*(*name as *const Char) };
    println!("[C] {:?}({:?})", name, name as *const _);
    println!("[C] tls: {:?}", tls);

    assert_eq!(tls, fs_base);

    let fd = unsafe { BorrowedFd::borrow_raw(*fd as i32) };
    let val = 4usize.to_ne_bytes();
    io::write(&fd, &val).unwrap();

    exit(0);
}

static NAME: &Char = Char::new(c"welcome");

pub fn clone3_example() {
    use clone::Flags as C;

    let fd = Eventfd::create(0, Eventfd::NONBLOCK).unwrap();

    let flags = C::VM | C::FS | C::FILES | C::SIGHAND | C::THREAD | C::SETTLS;

    let mut stack = [0usize; 512];
    let (stack, data) = stack.split_last_chunk_mut().unwrap();
    let (stack, args) = stack.split_last_chunk_mut().unwrap();

    *data = [fd.as_raw_fd() as _, NAME.as_ptr() as _];
    *args = [data.as_ptr() as _, thread as *const () as _];

    println!("[P] child stack: {:?}", args.as_ptr());
    println!("[P] child tls: {:?}", NAME.as_ptr());

    let clone = CloneArgs {
        flags,
        pidfd: 0,
        child_tid: 0,
        parent_tid: 0,
        exit_signal: 0,
        stack: stack.as_mut_ptr() as u64,
        stack_size: size_of_val(stack) as u64,
        tls: NAME.as_ptr() as u64,
        set_tid: 0,
        set_tid_size: 0,
        cgroup: 0,
    };
    unsafe { clone.clone3().unwrap() };

    Pollfd::new(&fd, Pollfd::IN).poll(-1).unwrap();
    let mut buf = [const { MaybeUninit::uninit() }; size_of::<usize>()];
    io::read(&fd, &mut buf).unwrap();
    let value = usize::from_ne_bytes(unsafe { MaybeUninit::from(buf).assume_init() });

    println!("[P] child notify: {}", value);
}

#![no_std]
#![no_main]
use core::fmt;
use core::mem::MaybeUninit;

use genos::net::addr::SockaddrUn;
use genos::net::socket::RecvFlags;
use genos::net::{OpenFlag, Socket};
use genos::println;
use genos::process::Args;

genos::main!(|args| start(args).is_err() as _);

fn start(mut args: Args) -> Result<(), Error> {
    let path = args.nth(1).ok_or("path argument required")?;

    let socket = Socket::unix_stream(<_>::CLOEXEC)?;
    socket.connect(&SockaddrUn::from_path(path)?)?;

    if let Ok(addr) = socket.peer_addr::<SockaddrUn>()
        && let Some(path) = addr.as_pathname()
    {
        println!("connected to {path:?}");
    }

    socket.send(b"Hello World!", <_>::default())?;

    let mut buf = [MaybeUninit::uninit(); 128];
    let len = socket.recv(&mut buf, RecvFlags::PEEK)?;
    let read = unsafe { buf[..len].assume_init_ref() };
    let read = str::from_utf8(read).unwrap_or("<non-utf8>");
    println!("read: {read:?}");
    assert_eq!(socket.recv(&mut buf, <_>::default())?, len);
    Ok(())
}

struct Error;

impl<E: fmt::Display> From<E> for Error {
    fn from(value: E) -> Self {
        println!("{value}");
        Self
    }
}

use core::fmt;
use core::mem::MaybeUninit;

use genos::net::addr::SockaddrUn;
use genos::net::socket::RecvFlags;
use genos::net::{OpenFlag, Socket};
use genos::process;

fn main() -> Result<(), Error> {
    let Some(path) = process::args().nth(1) else {
        return Err("path argument required".to_string().into());
    };

    let socket = Socket::unix_stream(<_>::CLOEXEC)?;
    socket.connect(&SockaddrUn::from_path(path)?)?;

    if let Ok(addr) = socket.peer_addr::<SockaddrUn>()
        && let Some(path) = addr.as_pathname()
    {
        println!("connected to {:?}", path);
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

struct Error(String);

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<E: fmt::Display> From<E> for Error {
    fn from(value: E) -> Self {
        Self(value.to_string())
    }
}

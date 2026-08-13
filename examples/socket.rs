use std::env;
use std::ffi::CString;
use std::fmt;

use genos::net::{OpenFlag, SockaddrUn, Socket};

fn main() -> Result<(), Error> {
    let Some(path) = env::args().nth(1) else {
        return Err("path argument required".to_string().into());
    };
    let path = CString::new(path)?;

    let socket = Socket::unix_stream(<_>::CLOEXEC)?;
    socket.connect(&SockaddrUn::from_path(&path)?)?;

    if let Ok(addr) = socket.peer_addr::<SockaddrUn>()
        && let Some(path) = addr.as_pathname()
    {
        println!("connected to {:?}", path);
    }

    socket.write(b"Hello World!")?;

    let mut buf = [0u8; 128];
    let len = socket.read(&mut buf)?;
    let read = str::from_utf8(&buf[..len]).unwrap_or("<non-utf8>");
    println!("read: {read:?}");
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

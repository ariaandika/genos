//! Networking primitives.
pub use addr::SockaddrUn;
pub use socket::Socket;

pub use crate::flags::OpenFlag;

pub mod addr;
pub mod msg;
pub mod socket;

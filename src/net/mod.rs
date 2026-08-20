//! Networking primitives.
pub use unix::SockAddrUn;

#[doc(inline)]
pub use socket::Socket;

pub use crate::flags::OpenFlag;

// ===== mods =====

pub mod addr;
mod unix;

pub mod iovec;
pub mod msg;
pub mod cmsg;

pub mod socket;

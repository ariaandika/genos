//! Networking primitives.
pub use socket::Socket;

pub use crate::flags::OpenFlag;

pub mod addr;
pub mod socket;

pub mod iovec;
pub mod msg;
pub mod cmsg;

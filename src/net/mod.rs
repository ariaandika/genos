//! Networking primitives.
#[doc(inline)]
pub use socket::Socket;

pub use crate::flags::OpenFlag;

// ===== mods =====

pub mod iovec;

pub mod addr;
pub mod msg;
pub mod cmsg;

pub mod unix;

pub mod socket;

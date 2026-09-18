//! Networking primitives.
#[doc(inline)]
pub use socket::Socket;
pub use raw::SaFamily;

pub use crate::flags::OpenFlag;

// ===== mods =====

mod raw;

// net abstraction

pub mod addr;
pub mod msg;
pub mod cmsg;

// net implementation

pub mod unix;

// `socket(2)`

pub mod socket;

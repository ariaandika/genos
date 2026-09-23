//! Networking primitives.
#[doc(inline)]
pub use socket::Socket;
pub use raw::SaFamily;

// ===== mods =====

mod raw;

// net abstraction

pub mod addr;
pub mod msg;
pub mod cmsg;

// net implementation

pub mod unix;
pub mod ip;

// `socket(2)`

pub mod socket;

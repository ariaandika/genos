//! Networking primitives.
pub use raw::{SaFamily, Socklen};

pub use addr::{Family, SockAddr};
pub use unix::SockAddrUn;
pub use ip::SockAddrIn;

pub use option::{OptInt, OptName, OptValue};

#[doc(inline)]
pub use socket::Socket;

// ===== mods =====

mod raw;

// socket address abstractions

mod addr;
mod msg;
mod cmsg;

// socket message abstraction

pub mod message {
    //! Socket message.
    pub use super::msg::{MsgHdrFlags, MsgHdr, MsgHdrMut};
    pub use super::cmsg::{CMsgArray, CMsgHdr, CMsgKind, CMsgType};

    pub use super::unix::SCMRights;
}

// socket address implementation

mod unix;
mod ip;

// `socket(2)`

mod option;
pub mod socket;

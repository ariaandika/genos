//! Networking primitives.
pub use raw::{SaFamily, Socklen};

pub use addr::{Family, SockAddr};
pub use unix::SockAddrUn;
pub use ip::SockAddrIn;

pub use option::{OptInt, OptLevel, OptName, OptValue};

#[doc(inline)]
pub use socket::Socket;

// ===== mods =====

mod raw;

mod option;
mod addr;
mod msg;
mod cmsg;

pub mod message {
    //! Socket message.
    pub use super::msg::{MsgHdrFlags, MsgHdr, MsgHdrMut};
    pub use super::cmsg::{CMsgArray, CMsgHdr, CMsgKind, CMsgType};

    pub use super::unix::SCMRights;
}

mod unix;
mod ip;

pub mod socket;

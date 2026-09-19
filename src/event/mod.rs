//! Non-blocking operations.
#[doc(inline)]
pub use epoll::Epoll;
#[doc(inline)]
pub use eventfd::Eventfd;

pub mod epoll;
pub mod eventfd;

//! Linux Feature Bindings.
#![warn(
    missing_docs,
    missing_debug_implementations,
    // for longterm use `unimplemented!()`
    clippy::todo,
    clippy::use_debug,
    clippy::dbg_macro
)]
#![allow(clippy::module_inception, clippy::new_without_default, clippy::len_without_is_empty)]

pub mod error;
pub mod fd;
pub mod flags;

pub mod alloc;
pub mod event;
pub mod io;
pub mod net;
pub mod process;
pub mod signal;

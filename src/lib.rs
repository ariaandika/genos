//! Linux Feature Bindings.
#![no_std]
#![warn(
    missing_docs,
    missing_debug_implementations,
    // for longterm use `unimplemented!()`
    clippy::todo,
    clippy::use_debug,
    clippy::dbg_macro
)]
#![allow(clippy::module_inception, clippy::new_without_default, clippy::len_without_is_empty)]

mod sys;

pub mod error;
pub mod fd;
pub mod flags;

pub mod env;
pub mod event;
pub mod fs;
pub mod io;
pub mod mem;
pub mod net;
pub mod process;
pub mod rand;
pub mod signal;
pub mod time;

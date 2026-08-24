//! Linux Feature Bindings.
#![no_std]
#![warn(
    missing_docs,
    missing_debug_implementations,
    clippy::todo, // for longterm use `unimplemented!()`
    clippy::use_debug,
    clippy::dbg_macro,
    clippy::explicit_auto_deref,
)]
#![allow(clippy::module_inception, clippy::new_without_default, clippy::len_without_is_empty)]

mod sys;

pub mod error;
pub mod fd;
pub mod ffi;
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

//! Linux Feature Bindings.
#![no_std]
#![warn(
    missing_docs,
    missing_debug_implementations,
    clippy::dbg_macro,
    clippy::expect_used,
    clippy::indexing_slicing,
    clippy::panic,
    clippy::string_slice,
    clippy::todo,
    clippy::unwrap_used,
    clippy::use_debug
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

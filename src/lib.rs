//! All things platform.
//!
//! # Standards
//!
//! Standards is an API that will be shared throughout other APIs.
//!
//! [`OwnedFd`] is an owned file descriptor that will be closed when dropped. This has the same API
//! as the rust standard library `OwnedFd`, redefined here because this crate is `#![no_std]`.
//!
//! [`Char`], mainly used as `&Char`, represent a C string that is guaranteed to be null terminated.
//! This crate cannot use [`CStr`] because its representation is not equal to C `*const char`.
//!
//! [`OwnedFd`]: fd::OwnedFd
//! [`Char`]: ffi::Char
//! [`CStr`]: core::ffi::CStr
//!
//! # Syscalls
//!
//! This crate provide syscalls as a function call. The syscalls are organized into predefined
//! categories:
//!
//! - [`event`], Non-blocking operations
//! - [`fs`], Filesystem manipulation
//! - [`io`], Input/Output operations
//! - [`mem`], Memory allocation
//! - [`net`], Networking primitives
//! - [`process`], Process management
//! - [`rand`], Random number generator
//! - [`signal`], Signal management
//! - [`time`], Time management
//!
//! # Other
//!
//! - [`elf`], Executable and Linkable Format (ELF)
//! - [`mod@env`], Argument and Environment variables
#![no_std]
#![warn(
    // quality of API
    missing_docs,
    missing_debug_implementations,
    // debugging remnant
    clippy::dbg_macro,
    clippy::use_debug,
    // forbid panic
    clippy::expect_used,
    clippy::indexing_slicing,
    clippy::panic,
    clippy::string_slice,
    clippy::todo,
    clippy::unwrap_used,
)]
#![allow(clippy::module_inception, clippy::new_without_default, clippy::len_without_is_empty)]

pub mod sys;

// ===== standards =====

pub mod fd;
pub mod ffi;
pub mod flags;

// ===== syscalls =====

pub mod alloc;
pub mod event;
pub mod fs;
pub mod io;
pub mod mem;
pub mod net;
pub mod process;
pub mod rand;
pub mod signal;
pub mod time;

// ===== environments =====

pub mod elf;
pub mod env;

//! Environment variables.
pub use args::Args;
pub use vars::Vars;
pub use libc::{Error, clear_vars, set_var, unset_var, var};

pub mod args;
pub mod vars;
mod libc;

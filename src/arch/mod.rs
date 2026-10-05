//! Architecture intrinsics.
pub use arch::*;

// ===== architecture =====

#[cfg_attr(target_arch = "x86_64", path = "x86_64.rs")]
mod arch;

#[cfg(not(target_arch = "x86_64"))]
compile_error!("current architecture is not yet supported");

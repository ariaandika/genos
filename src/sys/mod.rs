pub(crate) use arch::*;
pub(crate) use shared::*;
pub(crate) use types::*;

pub(crate) use macros::{call_rd, call};

// default fallback for arch specific definitions
mod asm_generic;

// architecture independent definitions
mod shared;

// `IntoArg` and `SysRes`
mod types;

// syscall, ioctl, fcntl
mod macros;

// ===== architecture =====

#[cfg_attr(target_arch = "x86_64", path = "x86_64.rs")]
mod arch;

#[cfg(not(target_arch = "x86_64"))]
compile_error!("this architecture is not yet supported");

use core::fmt;

use crate::sys;

/// Signal number.
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Signo(i32);

// `signal(7)`
impl Signo {
    /// Hangup detected on controlling terminal or death of controlling process.
    pub const HUP: Self = Self(sys::SIGHUP);
    /// Interrupt from keyboard.
    pub const INT: Self = Self(sys::SIGINT);
    /// Quit from keyboard.
    pub const QUIT: Self = Self(sys::SIGQUIT);
    /// Illegal Instruction.
    pub const ILL: Self = Self(sys::SIGILL);
    /// Abort signal from `abort(3)`.
    pub const ABRT: Self = Self(sys::SIGABRT);
    /// IOT trap. A synonym for [`Signo::ABRT`].
    pub const IOT: Self = Self(sys::SIGIOT);
    /// Erroneous arithmetic operation.
    pub const FPE: Self = Self(sys::SIGFPE);
    /// Kill signal.
    pub const KILL: Self = Self(sys::SIGKILL);
    /// User-defined signal 1.
    pub const USR1: Self = Self(sys::SIGUSR1);
    /// User-defined signal 2.
    pub const USR2: Self = Self(sys::SIGUSR2);
    /// Invalid memory reference.
    pub const SEGV: Self = Self(sys::SIGSEGV);
    /// Broken pipe: write to pipe with no readers; see `pipe(7)`.
    pub const PIPE: Self = Self(sys::SIGPIPE);
    /// Timer signal from `alarm(2)`.
    pub const ALRM: Self = Self(sys::SIGALRM);
    /// Termination signal.
    pub const TERM: Self = Self(sys::SIGTERM);
    /// Child stopped, terminated, or continued.
    pub const CHLD: Self = Self(sys::SIGCHLD);
    /// Continue if stopped.
    pub const CONT: Self = Self(sys::SIGCONT);

    pub(crate) fn from_raw(ssi_signo: u32) -> Signo {
        Self(ssi_signo as _)
    }

    /// Returns the raw signal number.
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl fmt::Debug for Signo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match match *self {
            Self::HUP => Some("SIGHUP"),
            Self::INT => Some("SIGINT"),
            Self::QUIT => Some("SIGQUIT"),
            Self::ILL => Some("SIGILL"),
            Self::IOT => Some("SIGIOT"),
            Self::FPE => Some("SIGFPE"),
            Self::KILL => Some("SIGKILL"),
            Self::USR1 => Some("SIGUSR1"),
            Self::USR2 => Some("SIGUSR2"),
            Self::SEGV => Some("SIGSEGV"),
            Self::PIPE => Some("SIGPIPE"),
            Self::ALRM => Some("SIGALRM"),
            Self::TERM => Some("SIGTERM"),
            Self::CHLD => Some("SIGCHLD"),
            Self::CONT => Some("SIGCONT"),
            _ => None,
        } {
            Some(name) => f.write_str(name),
            None => f.debug_tuple("Sig").field(&self.0).finish(),
        }
    }
}

impl From<Signo> for i32 {
    #[inline]
    fn from(value: Signo) -> Self {
        value.0
    }
}

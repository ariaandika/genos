use core::fmt;

/// Signal number.
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Signo(i32);

// `man 7 signal`
impl Signo {
    /// Hangup detected on controlling terminal or death of controlling process.
    pub const HUP: Self = Self(libc::SIGHUP);
    /// Interrupt from keyboard.
    pub const INT: Self = Self(libc::SIGINT);
    /// Quit from keyboard.
    pub const QUIT: Self = Self(libc::SIGQUIT);
    /// Illegal Instruction.
    pub const ILL: Self = Self(libc::SIGILL);
    /// Abort signal from `abort(3)`.
    pub const ABRT: Self = Self(libc::SIGABRT);
    /// IOT trap. A synonym for [`Signo::ABRT`].
    pub const IOT: Self = Self(libc::SIGIOT);
    /// Erroneous arithmetic operation.
    pub const FPE: Self = Self(libc::SIGFPE);
    /// Kill signal.
    pub const KILL: Self = Self(libc::SIGKILL);
    /// Invalid memory reference.
    pub const SEGV: Self = Self(libc::SIGSEGV);
    /// Broken pipe: write to pipe with no readers; see `pipe(7)`.
    pub const PIPE: Self = Self(libc::SIGPIPE);
    /// Timer signal from `alarm(2)`.
    pub const ALRM: Self = Self(libc::SIGALRM);
    /// Termination signal.
    pub const TERM: Self = Self(libc::SIGTERM);

    pub(crate) fn from_raw(ssi_signo: u32) -> Signo {
        Self(ssi_signo as _)
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
            Self::SEGV => Some("SIGSEGV"),
            Self::PIPE => Some("SIGPIPE"),
            Self::ALRM => Some("SIGALRM"),
            Self::TERM => Some("SIGTERM"),
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

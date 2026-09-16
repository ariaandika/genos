/// Signal number (`signal(7)`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Signo(i32);

impl Signo {
    /// `SIGHUP`
    pub const HUP: Self = Self(SIGHUP);
    /// `SIGINT`
    pub const INT: Self = Self(SIGINT);
    /// `SIGQUIT`
    pub const QUIT: Self = Self(SIGQUIT);
    /// `SIGILL`
    pub const ILL: Self = Self(SIGILL);
    /// `SIGTRAP`
    pub const TRAP: Self = Self(SIGTRAP);
    /// `SIGABRT`
    pub const ABRT: Self = Self(SIGABRT);
    /// `SIGIOT`
    pub const IOT: Self = Self(SIGIOT);
    /// `SIGBUS`
    pub const BUS: Self = Self(SIGBUS);
    /// `SIGFPE`
    pub const FPE: Self = Self(SIGFPE);
    /// `SIGKILL`
    pub const KILL: Self = Self(SIGKILL);
    /// `SIGUSR1`
    pub const USR1: Self = Self(SIGUSR1);
    /// `SIGUSR2`
    pub const USR2: Self = Self(SIGUSR2);
    /// `SIGSEGV`
    pub const SEGV: Self = Self(SIGSEGV);
    /// `SIGPIPE`
    pub const PIPE: Self = Self(SIGPIPE);
    /// `SIGALRM`
    pub const ALRM: Self = Self(SIGALRM);
    /// `SIGTERM`
    pub const TERM: Self = Self(SIGTERM);
    /// `SIGSTKFLT`
    pub const STKFLT: Self = Self(SIGSTKFLT);
    /// `SIGCHLD`
    pub const CHLD: Self = Self(SIGCHLD);
    /// `SIGCONT`
    pub const CONT: Self = Self(SIGCONT);

    pub(crate) fn from_raw(ssi_signo: u32) -> Signo {
        Self(ssi_signo as i32)
    }

    /// Returns the raw signal number.
    #[inline]
    pub const fn as_raw(self) -> i32 {
        self.0
    }
}

impl From<Signo> for i32 {
    #[inline]
    fn from(value: Signo) -> Self {
        value.0
    }
}

// ===== extern =====

// arch/x86/include/uapi/asm/signal.h

const SIGHUP: i32 = 1;
const SIGINT: i32 = 2;
const SIGQUIT: i32 = 3;
const SIGILL: i32 = 4;
const SIGTRAP: i32 = 5;
const SIGABRT: i32 = 6;
const SIGIOT: i32 = 6;
const SIGBUS: i32 = 7;
const SIGFPE: i32 = 8;
const SIGKILL: i32 = 9;
const SIGUSR1: i32 = 10;
const SIGSEGV: i32 = 11;
const SIGUSR2: i32 = 12;
const SIGPIPE: i32 = 13;
const SIGALRM: i32 = 14;
const SIGTERM: i32 = 15;
const SIGSTKFLT: i32 = 16;
const SIGCHLD: i32 = 17;
const SIGCONT: i32 = 18;
// const SIGSTOP: i32 = 19;
// const SIGTSTP: i32 = 20;
// const SIGTTIN: i32 = 21;
// const SIGTTOU: i32 = 22;
// const SIGURG: i32 = 23;
// const SIGXCPU: i32 = 24;
// const SIGXFSZ: i32 = 25;
// const SIGVTALRM: i32 = 26;
// const SIGPROF: i32 = 27;
// const SIGWINCH: i32 = 28;
// const SIGIO: i32 = 29;
// const SIGPOLL: i32 = SIGIO;

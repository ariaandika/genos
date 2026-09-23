use core::fmt;
use core::num::NonZeroI16;

use crate::sys::SysRaw;

// ===== ErrCode =====

/// Error Code.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ErrCode(NonZeroI16);

impl ErrCode {
    /// # Safety
    ///
    /// `code < 0`.
    pub(crate) const unsafe fn new(code: i16) -> Self {
        debug_assert!(code.is_negative());
        unsafe { Self(NonZeroI16::new_unchecked(code)) }
    }

    pub(crate) const fn from_sys(code: SysRaw) -> Option<Self> {
        if code >= 0 { None } else { unsafe { Some(Self(NonZeroI16::new_unchecked(code as _))) } }
    }

    /// Returns the raw error code.
    #[inline]
    pub const fn code(self) -> i32 {
        self.0.get().wrapping_neg() as i32
    }

    /// Returns `true` if error code is `EINTR`.
    #[inline]
    pub const fn is_interrupt(self) -> bool {
        matches!(self, Self::EINTR)
    }

    /// Returns `true` if error code is `EAGAIN`.
    #[inline]
    pub const fn is_retry(self) -> bool {
        matches!(self, Self::EAGAIN)
    }
}

// ===== core traits =====

impl fmt::Debug for ErrCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("ErrCode").field(&self.code()).finish()
    }
}

// ===== constants =====

macro_rules! def_errno {
    ($($vis:vis const $name:ident: $ty:ty = $val:expr;)*) => {
        impl ErrCode {$(
            #[doc = concat!(" `", stringify!($name), "`")]
            $vis const $name: $ty = Self(NonZeroI16::new($val as i16).unwrap().wrapping_neg());
        )*}
    };
}
def_errno! {
    // include/uapi/asm-generic/errno-base.h
    pub const EPERM: Self   = 1;
    pub const ENOENT: Self  = 2;
    pub const ESRCH : Self  = 3;
    pub const EINTR: Self   = 4;
    pub const EIO: Self     = 5;
    pub const ENXIO: Self   = 6;
    pub const E2BIG: Self   = 7;
    pub const ENOEXEC: Self = 8;
    pub const EBADF: Self   = 9;
    pub const ECHILD: Self  = 10;
    pub const EAGAIN: Self  = 11;
    pub const ENOMEM: Self  = 12;
    pub const EACCES: Self  = 13;
    pub const EFAULT: Self  = 14;
    pub const ENOTBLK: Self = 15;
    pub const EBUSY: Self   = 16;
    pub const EEXIST: Self  = 17;
    pub const EXDEV: Self   = 18;
    pub const ENODEV: Self  = 19;
    pub const ENOTDIR: Self = 20;
    pub const EISDIR: Self  = 21;
    pub const EINVAL: Self  = 22;
    pub const ENFILE: Self  = 23;
    pub const EMFILE: Self  = 24;
    pub const ENOTTY: Self  = 25;
    pub const ETXTBSY: Self = 26;
    pub const EFBIG: Self   = 27;
    pub const ENOSPC: Self  = 28;
    pub const ESPIPE: Self  = 29;
    pub const EROFS: Self   = 30;
    pub const EMLINK: Self  = 31;
    pub const EPIPE: Self   = 32;
    pub const EDOM: Self    = 33;
    pub const ERANGE: Self  = 34;

    // include/uapi/asm-generic/errno.h
    pub const EDEADLK: Self     = 35;
    pub const ENAMETOOLONG: Self    = 36;
    pub const ENOLCK: Self      = 37;
    pub const ENOSYS: Self      = 38;
    pub const ENOTEMPTY: Self   = 39;
    pub const ELOOP: Self       = 40;
    pub const EWOULDBLOCK: Self = Self::EAGAIN.code();
    pub const ENOMSG: Self      = 42;
    pub const EIDRM: Self       = 43;
    pub const ECHRNG: Self      = 44;
    pub const EL2NSYNC: Self    = 45;
    pub const EL3HLT: Self      = 46;
    pub const EL3RST: Self      = 47;
    pub const ELNRNG: Self      = 48;
    pub const EUNATCH: Self     = 49;
    pub const ENOCSI: Self      = 50;
    pub const EL2HLT: Self      = 51;
    pub const EBADE: Self       = 52;
    pub const EBADR: Self       = 53;
    pub const EXFULL: Self      = 54;
    pub const ENOANO: Self      = 55;
    pub const EBADRQC: Self     = 56;
    pub const EBADSLT: Self     = 57;
    pub const EDEADLOCK: Self   = Self::EDEADLK.code();
    pub const EBFONT: Self      = 59;
    pub const ENOSTR: Self      = 60;
    pub const ENODATA: Self     = 61;
    pub const ETIME: Self       = 62;
    pub const ENOSR: Self       = 63;
    pub const ENONET: Self      = 64;
    pub const ENOPKG: Self      = 65;
    pub const EREMOTE: Self     = 66;
    pub const ENOLINK: Self     = 67;
    pub const EADV: Self        = 68;
    pub const ESRMNT: Self      = 69;
    pub const ECOMM: Self       = 70;
    pub const EPROTO: Self      = 71;
    pub const EMULTIHOP: Self   = 72;
    pub const EDOTDOT: Self     = 73;
    pub const EBADMSG: Self     = 74;
    pub const EFSBADCRC: Self   = Self::EBADMSG.code();
    pub const EOVERFLOW: Self   = 75;
    pub const ENOTUNIQ: Self    = 76;
    pub const EBADFD: Self      = 77;
    pub const EREMCHG: Self     = 78;
    pub const ELIBACC: Self     = 79;
    pub const ELIBBAD: Self     = 80;
    pub const ELIBSCN: Self     = 81;
    pub const ELIBMAX: Self     = 82;
    pub const ELIBEXEC: Self    = 83;
    pub const EILSEQ: Self      = 84;
    pub const ERESTART: Self    = 85;
    pub const ESTRPIPE: Self    = 86;
    pub const EUSERS: Self      = 87;
    pub const ENOTSOCK: Self    = 88;
    pub const EDESTADDRREQ: Self    = 89;
    pub const EMSGSIZE: Self    = 90;
    pub const EPROTOTYPE: Self  = 91;
    pub const ENOPROTOOPT: Self     = 92;
    pub const EPROTONOSUPPORT: Self = 93;
    pub const ESOCKTNOSUPPORT: Self = 94;
    pub const EOPNOTSUPP: Self      = 95;
    pub const EPFNOSUPPORT: Self    = 96;
    pub const EAFNOSUPPORT: Self    = 97;
    pub const EADDRINUSE: Self      = 98;
    pub const EADDRNOTAVAIL: Self   = 99;
    pub const ENETDOWN: Self        = 100;
    pub const ENETUNREACH: Self     = 101;
    pub const ENETRESET: Self       = 102;
    pub const ECONNABORTED: Self    = 103;
    pub const ECONNRESET: Self      = 104;
    pub const ENOBUFS: Self         = 105;
    pub const EISCONN: Self         = 106;
    pub const ENOTCONN: Self        = 107;
    pub const ESHUTDOWN: Self       = 108;
    pub const ETOOMANYREFS: Self    = 109;
    pub const ETIMEDOUT: Self       = 110;
    pub const ECONNREFUSED: Self    = 111;
    pub const EHOSTDOWN: Self       = 112;
    pub const EHOSTUNREACH: Self    = 113;
    pub const EALREADY: Self        = 114;
    pub const EINPROGRESS: Self     = 115;
    pub const ESTALE: Self          = 116;
    pub const EUCLEAN: Self         = 117;
    pub const EFSCORRUPTED: Self    = Self::EUCLEAN.code();
    pub const ENOTNAM: Self         = 118;
    pub const ENAVAIL: Self         = 119;
    pub const EISNAM: Self          = 120;
    pub const EREMOTEIO: Self       = 121;
    pub const EDQUOT: Self          = 122;
    pub const ENOMEDIUM: Self       = 123;
    pub const EMEDIUMTYPE: Self     = 124;
    pub const ECANCELED: Self       = 125;
    pub const ENOKEY: Self          = 126;
    pub const EKEYEXPIRED: Self     = 127;
    pub const EKEYREVOKED: Self     = 128;
    pub const EKEYREJECTED: Self    = 129;
    pub const EOWNERDEAD: Self      = 130;
    pub const ENOTRECOVERABLE: Self = 131;
    pub const ERFKILL: Self         = 132;
    pub const EHWPOISON: Self       = 133;
    pub const EFTYPE: Self          = 134;
}

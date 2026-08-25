use core::num::NonZeroU8;
use core::{error, fmt};

// ===== ErrCode =====

/// Error Code.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ErrCode(NonZeroU8);

impl ErrCode {
    /// Creates [`ErrCode`] with given error code.
    #[inline]
    pub const fn new(code: i32) -> Self {
        Self(match NonZeroU8::new(code as _) {
            Some(x) => x,
            None => NonZeroU8::MAX,
        })
    }

    /// Convert error code from syscall result.
    pub(crate) const fn sys(res: isize) -> Self {
        Self(match NonZeroU8::new((res as u8).wrapping_neg()) {
            Some(x) => x,
            None => NonZeroU8::MAX,
        })
    }

    /// Returns the contained raw error code.
    #[inline]
    pub const fn code(self) -> i32 {
        self.0.get() as i32
    }

    /// Returns `true` if error code is `EINTR`.
    #[inline]
    pub const fn is_interrupt(self) -> bool {
        matches!(self, Self::EINTR)
    }

    /// Returns `true` if error code is `EWOULDBLOCK`.
    #[inline]
    pub const fn would_block(self) -> bool {
        matches!(self, Self::EWOULDBLOCK)
    }
}

// ===== core traits =====

impl error::Error for ErrCode {}

impl fmt::Display for ErrCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let code = self.0.get() as usize;
        let msg = match ERRMSG.get(code) {
            Some(msg) => msg.trim_ascii_start(),
            None => "Unknown error code",
        };
        write!(f, "{msg} (os error {code})",)
    }
}

// ===== extern =====

// source: include/uapi/asm-generic/errno-base.h
// source: include/uapi/asm-generic/errno.h

// "Success (0)" + last code value
const LEN: usize = 1 + ErrCode::EFTYPE.0.get() as usize;

// excluded from macro to not break message table
impl ErrCode {
    /// Bad CRC detected
    pub const EFSBADCRC: Self = Self::EBADMSG;
    /// Filesystem is corrupted
    pub const EFSCORRUPTED: Self = Self::EUCLEAN;
}

macro_rules! def_errno {
    ($($vis:vis const $name:ident: $ty:ty = $val:expr;#[doc = $m:literal])*) => {
        const ERRMSG: [&str; LEN] = [" Success", $($m),*];
        impl ErrCode { $(#[doc = $m]$vis const $name: $ty = <$ty>::new($val);)* }
    };
}
def_errno! {
    pub const EPERM: Self = 1; /// Operation not permitted
    pub const ENOENT: Self = 2; /// No such file or directory
    pub const ESRCH: Self = 3; /// No such process
    pub const EINTR: Self = 4; /// Interrupted system call
    pub const EIO: Self = 5; /// I/O error
    pub const ENXIO: Self = 6; /// No such device or address
    pub const E2BIG: Self = 7; /// Argument list too long
    pub const ENOEXEC: Self = 8; /// Exec format error
    pub const EBADF: Self = 9; /// Bad file number
    pub const ECHILD: Self = 10; /// No child processes
    pub const EAGAIN: Self = 11; /// Try again
    pub const ENOMEM: Self = 12; /// Out of memory
    pub const EACCES: Self = 13; /// Permission denied
    pub const EFAULT: Self = 14; /// Bad address
    pub const ENOTBLK: Self = 15; /// Block device required
    pub const EBUSY: Self = 16; /// Device or resource busy
    pub const EEXIST: Self = 17; /// File exists
    pub const EXDEV: Self = 18; /// Cross-device link
    pub const ENODEV: Self = 19; /// No such device
    pub const ENOTDIR: Self = 20; /// Not a directory
    pub const EISDIR: Self = 21; /// Is a directory
    pub const EINVAL: Self = 22; /// Invalid argument
    pub const ENFILE: Self = 23; /// File table overflow
    pub const EMFILE: Self = 24; /// Too many open files
    pub const ENOTTY: Self = 25; /// Not a typewriter
    pub const ETXTBSY: Self = 26; /// Text file busy
    pub const EFBIG: Self = 27; /// File too large
    pub const ENOSPC: Self = 28; /// No space left on device
    pub const ESPIPE: Self = 29; /// Illegal seek
    pub const EROFS: Self = 30; /// Read-only file system
    pub const EMLINK: Self = 31; /// Too many links
    pub const EPIPE: Self = 32; /// Broken pipe
    pub const EDOM: Self = 33; /// Math argument out of domain of func
    pub const ERANGE: Self = 34; /// Math result not representable
    pub const EDEADLK: Self = 35; /// Resource deadlock would occur
    pub const ENAMETOOLONG: Self = 36; /// File name too long
    pub const ENOLCK: Self = 37; /// No record locks available
    pub const ENOSYS: Self = 38; /// Invalid system call number
    pub const ENOTEMPTY: Self = 39; /// Directory not empty
    pub const ELOOP: Self = 40; /// Too many symbolic links encountered
    pub const EWOULDBLOCK: Self = Self::EAGAIN.code(); /// Operation would block
    pub const ENOMSG: Self = 42; /// No message of desired type
    pub const EIDRM: Self = 43; /// Identifier removed
    pub const ECHRNG: Self = 44; /// Channel number out of range
    pub const EL2NSYNC: Self = 45; /// Level 2 not synchronized
    pub const EL3HLT: Self = 46; /// Level 3 halted
    pub const EL3RST: Self = 47; /// Level 3 reset
    pub const ELNRNG: Self = 48; /// Link number out of range
    pub const EUNATCH: Self = 49; /// Protocol driver not attached
    pub const ENOCSI: Self = 50; /// No CSI structure available
    pub const EL2HLT: Self = 51; /// Level 2 halted
    pub const EBADE: Self = 52; /// Invalid exchange
    pub const EBADR: Self = 53; /// Invalid request descriptor
    pub const EXFULL: Self = 54; /// Exchange full
    pub const ENOANO: Self = 55; /// No anode
    pub const EBADRQC: Self = 56; /// Invalid request code
    pub const EBADSLT: Self = 57; /// Invalid slot
    pub const EDEADLOCK: Self = Self::EDEADLK.code(); /// Deadlocked
    pub const EBFONT: Self = 59; /// Bad font file format
    pub const ENOSTR: Self = 60; /// Device not a stream
    pub const ENODATA: Self = 61; /// No data available
    pub const ETIME: Self = 62; /// Timer expired
    pub const ENOSR: Self = 63; /// Out of streams resources
    pub const ENONET: Self = 64; /// Machine is not on the network
    pub const ENOPKG: Self = 65; /// Package not installed
    pub const EREMOTE: Self = 66; /// Object is remote
    pub const ENOLINK: Self = 67; /// Link has been severed
    pub const EADV: Self = 68; /// Advertise error
    pub const ESRMNT: Self = 69; /// Srmount error
    pub const ECOMM: Self = 70; /// Communication error on send
    pub const EPROTO: Self = 71; /// Protocol error
    pub const EMULTIHOP: Self = 72; /// Multihop attempted
    pub const EDOTDOT: Self = 73; /// RFS specific error
    pub const EBADMSG: Self = 74; /// Not a data message
    pub const EOVERFLOW: Self = 75; /// Value too large for defined data type
    pub const ENOTUNIQ: Self = 76; /// Name not unique on network
    pub const EBADFD: Self = 77; /// File descriptor in bad state
    pub const EREMCHG: Self = 78; /// Remote address changed
    pub const ELIBACC: Self = 79; /// Can not access a needed shared library
    pub const ELIBBAD: Self = 80; /// Accessing a corrupted shared library
    pub const ELIBSCN: Self = 81; /// .lib section in a.out corrupted
    pub const ELIBMAX: Self = 82; /// Attempting to link in too many shared libraries
    pub const ELIBEXEC: Self = 83; /// Cannot exec a shared library directly
    pub const EILSEQ: Self = 84; /// Illegal byte sequence
    pub const ERESTART: Self = 85; /// Interrupted system call should be restarted
    pub const ESTRPIPE: Self = 86; /// Streams pipe error
    pub const EUSERS: Self = 87; /// Too many users
    pub const ENOTSOCK: Self = 88; /// Socket operation on non-socket
    pub const EDESTADDRREQ: Self = 89; /// Destination address required
    pub const EMSGSIZE: Self = 90; /// Message too long
    pub const EPROTOTYPE: Self = 91; /// Protocol wrong type for socket
    pub const ENOPROTOOPT: Self = 92; /// Protocol not available
    pub const EPROTONOSUPPORT: Self = 93; /// Protocol not supported
    pub const ESOCKTNOSUPPORT: Self = 94; /// Socket type not supported
    pub const EOPNOTSUPP: Self = 95; /// Operation not supported on transport endpoint
    pub const EPFNOSUPPORT: Self = 96; /// Protocol family not supported
    pub const EAFNOSUPPORT: Self = 97; /// Address family not supported by protocol
    pub const EADDRINUSE: Self = 98; /// Address already in use
    pub const EADDRNOTAVAIL: Self = 99; /// Cannot assign requested address
    pub const ENETDOWN: Self = 100; /// Network is down
    pub const ENETUNREACH: Self = 101; /// Network is unreachable
    pub const ENETRESET: Self = 102; /// Network dropped connection because of reset
    pub const ECONNABORTED: Self = 103; /// Software caused connection abort
    pub const ECONNRESET: Self = 104; /// Connection reset by peer
    pub const ENOBUFS: Self = 105; /// No buffer space available
    pub const EISCONN: Self = 106; /// Transport endpoint is already connected
    pub const ENOTCONN: Self = 107; /// Transport endpoint is not connected
    pub const ESHUTDOWN: Self = 108; /// Cannot send after transport endpoint shutdown
    pub const ETOOMANYREFS: Self = 109; /// Too many references: cannot splice
    pub const ETIMEDOUT: Self = 110; /// Connection timed out
    pub const ECONNREFUSED: Self = 111; /// Connection refused
    pub const EHOSTDOWN: Self = 112; /// Host is down
    pub const EHOSTUNREACH: Self = 113; /// No route to host
    pub const EALREADY: Self = 114; /// Operation already in progress
    pub const EINPROGRESS: Self = 115; /// Operation now in progress
    pub const ESTALE: Self = 116; /// Stale file handle
    pub const EUCLEAN: Self = 117; /// Structure needs cleaning
    pub const ENOTNAM: Self = 118; /// Not a XENIX named type file
    pub const ENAVAIL: Self = 119; /// No XENIX semaphores available
    pub const EISNAM: Self = 120; /// Is a named type file
    pub const EREMOTEIO: Self = 121; /// Remote I/O error
    pub const EDQUOT: Self = 122; /// Quota exceeded
    pub const ENOMEDIUM: Self = 123; /// No medium found
    pub const EMEDIUMTYPE: Self = 124; /// Wrong medium type
    pub const ECANCELED: Self = 125; /// Operation Canceled
    pub const ENOKEY: Self = 126; /// Required key not available
    pub const EKEYEXPIRED: Self = 127; /// Key has expired
    pub const EKEYREVOKED: Self = 128; /// Key has been revoked
    pub const EKEYREJECTED: Self = 129; /// Key was rejected by service
    pub const EOWNERDEAD: Self = 130; /// Owner died
    pub const ENOTRECOVERABLE: Self = 131; /// State not recoverable
    pub const ERFKILL: Self = 132; /// Operation not possible due to RF-kill
    pub const EHWPOISON: Self = 133; /// Memory page has hardware error
    pub const EFTYPE: Self = 134; /// Wrong file type for the intended operation
}

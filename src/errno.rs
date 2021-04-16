use cfg_if::cfg_if;
use libc::c_int;
use std::{fmt, result};

pub type Result<T> = result::Result<T, Error>;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Error {
    Sys(Errno),
    UnsupportedOperation,
}

impl Error {
    pub fn as_errno(self) -> Option<Errno> {
        if let Error::Sys(e) = self {
            Some(e)
        } else {
            None
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Error::Sys(e) = self {
            write!(f, "{:?}: {}", self, e.desc())
        } else {
            write!(f, "{:?}: {}", self, "UnsupportedOperation")
        }
    }
}

cfg_if! {
    if #[cfg(any(target_os = "ios",
                 target_os = "macos"))] {
        unsafe fn errno_location() -> *mut c_int {
            libc::__error()
        }
    } else if #[cfg(any(target_os = "android"))] {
        unsafe fn errno_location() -> *mut c_int {
            libc::__errno()
        }
    } else if #[cfg(any(target_os = "linux",
                        target_os = "fuchsia"))] {
        unsafe fn errno_location() -> *mut c_int {
            libc::__errno_location()
        }
    }
}

pub fn errno() -> i32 {
    unsafe { (*errno_location()) as i32 }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Errno(i32);

impl Errno {
    pub fn last() -> Self {
        last()
    }

    pub fn desc(self) -> &'static str {
        desc(self)
    }

    pub fn from_i32(err: i32) -> Errno {
        from_i32(err)
    }

    pub fn result<S: ErrnoBasis + PartialEq<S>>(value: S) -> Result<S> {
        if value == S::basis() {
            Err(Error::Sys(Self::last()))
        } else {
            Ok(value)
        }
    }
}

pub trait ErrnoBasis: Sized {
    fn basis() -> Self;
}

impl ErrnoBasis for i32 {
    fn basis() -> Self {
        -1
    }
}

fn last() -> Errno {
    Errno(errno())
}

fn from_i32(e: i32) -> Errno {
    Errno(e)
}

impl fmt::Display for Errno {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}: {}", self, self.desc())
    }
}

fn desc(errno: Errno) -> &'static str {
    match errno.0 {
        libc::EPERM => "Operation not permitted",
        libc::ENOENT => "No such file or directory",
        libc::ESRCH => "No such process",
        libc::EINTR => "Interrupted system call",
        libc::EIO => "I/O error",
        libc::ENXIO => "No such device or address",
        libc::E2BIG => "Argument list too long",
        libc::ENOEXEC => "Exec format error",
        libc::EBADF => "Bad file number",
        libc::ECHILD => "No child processes",
        libc::EAGAIN => "Try again",
        libc::ENOMEM => "Out of memory",
        libc::EACCES => "Permission denied",
        libc::EFAULT => "Bad address",
        libc::ENOTBLK => "Block device required",
        libc::EBUSY => "Device or resource busy",
        libc::EEXIST => "File exists",
        libc::EXDEV => "Cross-device link",
        libc::ENODEV => "No such device",
        libc::ENOTDIR => "Not a directory",
        libc::EISDIR => "Is a directory",
        libc::EINVAL => "Invalid argument",
        libc::ENFILE => "File table overflow",
        libc::EMFILE => "Too many open files",
        libc::ENOTTY => "Not a typewriter",
        libc::ETXTBSY => "Text file busy",
        libc::EFBIG => "File too large",
        libc::ENOSPC => "No space left on device",
        libc::ESPIPE => "Illegal seek",
        libc::EROFS => "Read-only file system",
        libc::EMLINK => "Too many links",
        libc::EPIPE => "Broken pipe",
        libc::EDOM => "Math argument out of domain of func",
        libc::ERANGE => "Math result not representable",
        libc::EDEADLK => "Resource deadlock would occur",
        libc::ENAMETOOLONG => "File name too long",
        libc::ENOLCK => "No record locks available",
        libc::ENOSYS => "Function not implemented",
        libc::ENOTEMPTY => "Directory not empty",
        libc::ELOOP => "Too many symbolic links encountered",
        libc::ENOMSG => "No message of desired type",
        libc::EIDRM => "Identifier removed",
        libc::EINPROGRESS => "Operation now in progress",
        libc::EALREADY => "Operation already in progress",
        libc::ENOTSOCK => "Socket operation on non-socket",
        libc::EDESTADDRREQ => "Destination address required",
        libc::EMSGSIZE => "Message too long",
        libc::EPROTOTYPE => "Protocol wrong type for socket",
        libc::ENOPROTOOPT => "Protocol not available",
        libc::EPROTONOSUPPORT => "Protocol not supported",
        libc::ESOCKTNOSUPPORT => "Socket type not supported",
        libc::EPFNOSUPPORT => "Protocol family not supported",
        libc::EAFNOSUPPORT => "Address family not supported by protocol",
        libc::EADDRINUSE => "Address already in use",
        libc::EADDRNOTAVAIL => "Cannot assign requested address",
        libc::ENETDOWN => "Network is down",
        libc::ENETUNREACH => "Network is unreachable",
        libc::ENETRESET => "Network dropped connection because of reset",
        libc::ECONNABORTED => "Software caused connection abort",
        libc::ECONNRESET => "Connection reset by peer",
        libc::ENOBUFS => "No buffer space available",
        libc::EISCONN => "Transport endpoint is already connected",
        libc::ENOTCONN => "Transport endpoint is not connected",
        libc::ESHUTDOWN => "Cannot send after transport endpoint shutdown",
        libc::ETOOMANYREFS => "Too many references: cannot splice",
        libc::ETIMEDOUT => "Connection timed out",
        libc::ECONNREFUSED => "Connection refused",
        libc::EHOSTDOWN => "Host is down",
        libc::EHOSTUNREACH => "No route to host",

        #[cfg(any(target_os = "linux", target_os = "android", target_os = "fuchsia"))]
        libc::ECHRNG => "Channel number out of range",

        #[cfg(any(target_os = "linux", target_os = "android", target_os = "fuchsia"))]
        libc::EL2NSYNC => "Level 2 not synchronized",

        #[cfg(any(target_os = "linux", target_os = "android", target_os = "fuchsia"))]
        libc::EL3HLT => "Level 3 halted",

        #[cfg(any(target_os = "linux", target_os = "android", target_os = "fuchsia"))]
        libc::EL3RST => "Level 3 reset",

        #[cfg(any(target_os = "linux", target_os = "android", target_os = "fuchsia"))]
        libc::ELNRNG => "Link number out of range",

        #[cfg(any(target_os = "linux", target_os = "android", target_os = "fuchsia"))]
        libc::EUNATCH => "Protocol driver not attached",

        #[cfg(any(target_os = "linux", target_os = "android", target_os = "fuchsia"))]
        libc::ENOCSI => "No CSI structure available",

        #[cfg(any(
            target_os = "linux",
            target_os = "android",
            target_os = "illumos",
            target_os = "solaris",
            target_os = "fuchsia"
        ))]
        libc::EL2HLT => "Level 2 halted",

        #[cfg(any(target_os = "linux", target_os = "android", target_os = "fuchsia"))]
        libc::EBADE => "Invalid exchange",

        #[cfg(any(target_os = "linux", target_os = "android", target_os = "fuchsia"))]
        libc::EBADR => "Invalid request descriptor",

        #[cfg(any(target_os = "linux", target_os = "android", target_os = "fuchsia"))]
        libc::EXFULL => "Exchange full",

        #[cfg(any(target_os = "linux", target_os = "android", target_os = "fuchsia"))]
        libc::ENOANO => "No anode",

        #[cfg(any(target_os = "linux", target_os = "android", target_os = "fuchsia"))]
        libc::EBADRQC => "Invalid request code",

        #[cfg(any(target_os = "linux", target_os = "android", target_os = "fuchsia"))]
        libc::EBADSLT => "Invalid slot",

        #[cfg(any(target_os = "linux", target_os = "android", target_os = "fuchsia"))]
        libc::EBFONT => "Bad font file format",

        #[cfg(any(target_os = "linux", target_os = "android", target_os = "fuchsia"))]
        libc::ENOSTR => "Device not a stream",

        #[cfg(any(target_os = "linux", target_os = "android", target_os = "fuchsia"))]
        libc::ENODATA => "No data available",

        #[cfg(any(target_os = "linux", target_os = "android", target_os = "fuchsia"))]
        libc::ETIME => "Timer expired",

        #[cfg(any(target_os = "linux", target_os = "android", target_os = "fuchsia"))]
        libc::ENOSR => "Out of streams resources",

        #[cfg(any(target_os = "linux", target_os = "android", target_os = "fuchsia"))]
        libc::ENONET => "Machine is not on the network",

        #[cfg(any(target_os = "linux", target_os = "android", target_os = "fuchsia"))]
        libc::ENOPKG => "Package not installed",

        #[cfg(any(target_os = "linux", target_os = "android", target_os = "fuchsia"))]
        libc::EREMOTE => "Object is remote",

        #[cfg(any(
            target_os = "linux",
            target_os = "android",
            target_os = "illumos",
            target_os = "solaris",
            target_os = "fuchsia"
        ))]
        libc::ENOLINK => "Link has been severed",

        #[cfg(any(target_os = "linux", target_os = "android", target_os = "fuchsia"))]
        libc::EADV => "Advertise error",

        #[cfg(any(target_os = "linux", target_os = "android", target_os = "fuchsia"))]
        libc::ESRMNT => "Srmount error",

        #[cfg(any(target_os = "linux", target_os = "android", target_os = "fuchsia"))]
        libc::ECOMM => "Communication error on send",

        #[cfg(any(target_os = "linux", target_os = "android", target_os = "fuchsia"))]
        libc::EPROTO => "Protocol error",

        #[cfg(any(target_os = "linux", target_os = "android", target_os = "fuchsia"))]
        libc::EMULTIHOP => "Multihop attempted",

        #[cfg(any(target_os = "linux", target_os = "android", target_os = "fuchsia"))]
        libc::EDOTDOT => "RFS specific error",

        #[cfg(any(target_os = "linux", target_os = "android", target_os = "fuchsia"))]
        libc::EBADMSG => "Not a data message",

        #[cfg(any(target_os = "linux", target_os = "android", target_os = "fuchsia"))]
        libc::EOVERFLOW => "Value too large for defined data type",

        #[cfg(any(target_os = "linux", target_os = "android", target_os = "fuchsia"))]
        libc::ENOTUNIQ => "Name not unique on network",

        #[cfg(any(target_os = "linux", target_os = "android", target_os = "fuchsia"))]
        libc::EBADFD => "File descriptor in bad state",

        #[cfg(any(target_os = "linux", target_os = "android", target_os = "fuchsia"))]
        libc::EREMCHG => "Remote address changed",

        #[cfg(any(target_os = "linux", target_os = "android", target_os = "fuchsia"))]
        libc::ELIBACC => "Can not access a needed shared library",

        #[cfg(any(target_os = "linux", target_os = "android", target_os = "fuchsia"))]
        libc::ELIBBAD => "Accessing a corrupted shared library",

        #[cfg(any(target_os = "linux", target_os = "android", target_os = "fuchsia"))]
        libc::ELIBSCN => ".lib section in a.out corrupted",

        #[cfg(any(target_os = "linux", target_os = "android", target_os = "fuchsia"))]
        libc::ELIBMAX => "Attempting to link in too many shared libraries",

        #[cfg(any(target_os = "linux", target_os = "android", target_os = "fuchsia"))]
        libc::ELIBEXEC => "Cannot exec a shared library directly",

        #[cfg(any(target_os = "linux", target_os = "android", target_os = "fuchsia",))]
        libc::EILSEQ => "Illegal byte sequence",

        #[cfg(any(target_os = "linux", target_os = "android", target_os = "fuchsia"))]
        libc::ERESTART => "Interrupted system call should be restarted",

        #[cfg(any(target_os = "linux", target_os = "android", target_os = "fuchsia"))]
        libc::ESTRPIPE => "Streams pipe error",

        #[cfg(any(target_os = "linux", target_os = "android", target_os = "fuchsia"))]
        libc::EUSERS => "Too many users",

        #[cfg(any(target_os = "linux", target_os = "android", target_os = "fuchsia",))]
        libc::EOPNOTSUPP => "Operation not supported on transport endpoint",

        #[cfg(any(target_os = "linux", target_os = "android", target_os = "fuchsia"))]
        libc::ESTALE => "Stale file handle",

        #[cfg(any(target_os = "linux", target_os = "android", target_os = "fuchsia"))]
        libc::EUCLEAN => "Structure needs cleaning",

        #[cfg(any(target_os = "linux", target_os = "android", target_os = "fuchsia"))]
        libc::ENOTNAM => "Not a XENIX named type file",

        #[cfg(any(target_os = "linux", target_os = "android", target_os = "fuchsia"))]
        libc::ENAVAIL => "No XENIX semaphores available",

        #[cfg(any(target_os = "linux", target_os = "android", target_os = "fuchsia"))]
        libc::EISNAM => "Is a named type file",

        #[cfg(any(target_os = "linux", target_os = "android", target_os = "fuchsia"))]
        libc::EREMOTEIO => "Remote I/O error",

        #[cfg(any(target_os = "linux", target_os = "android", target_os = "fuchsia"))]
        libc::EDQUOT => "Quota exceeded",

        #[cfg(any(target_os = "linux", target_os = "android", target_os = "fuchsia",))]
        libc::ENOMEDIUM => "No medium found",

        #[cfg(any(target_os = "linux", target_os = "android", target_os = "fuchsia",))]
        libc::EMEDIUMTYPE => "Wrong medium type",

        #[cfg(any(target_os = "linux", target_os = "android", target_os = "fuchsia"))]
        libc::ECANCELED => "Operation canceled",

        #[cfg(any(target_os = "linux", target_os = "android", target_os = "fuchsia"))]
        libc::ENOKEY => "Required key not available",

        #[cfg(any(target_os = "linux", target_os = "android", target_os = "fuchsia"))]
        libc::EKEYEXPIRED => "Key has expired",

        #[cfg(any(target_os = "linux", target_os = "android", target_os = "fuchsia"))]
        libc::EKEYREVOKED => "Key has been revoked",

        #[cfg(any(target_os = "linux", target_os = "android", target_os = "fuchsia"))]
        libc::EKEYREJECTED => "Key was rejected by service",

        #[cfg(any(target_os = "linux", target_os = "android", target_os = "fuchsia"))]
        libc::EOWNERDEAD => "Owner died",

        #[cfg(any(target_os = "linux", target_os = "android", target_os = "fuchsia"))]
        libc::ENOTRECOVERABLE => "State not recoverable",

        #[cfg(any(
            all(target_os = "linux", not(target_arch = "mips")),
            target_os = "fuchsia"
        ))]
        libc::ERFKILL => "Operation not possible due to RF-kill",

        #[cfg(any(
            all(target_os = "linux", not(target_arch = "mips")),
            target_os = "fuchsia"
        ))]
        libc::EHWPOISON => "Memory page has hardware error",

        #[cfg(any(target_os = "macos", target_os = "ios",))]
        libc::ENEEDAUTH => "Need authenticator",

        #[cfg(any(target_os = "macos", target_os = "ios",))]
        libc::EOVERFLOW => "Value too large to be stored in data type",

        #[cfg(any(target_os = "macos", target_os = "ios",))]
        libc::EILSEQ => "Illegal byte sequence",

        #[cfg(any(target_os = "macos", target_os = "ios",))]
        libc::ENOATTR => "Attribute not found",

        #[cfg(any(target_os = "macos", target_os = "ios",))]
        libc::EBADMSG => "Bad message",

        #[cfg(any(target_os = "macos", target_os = "ios",))]
        libc::EPROTO => "Protocol error",

        #[cfg(any(target_os = "macos", target_os = "ios",))]
        libc::ENOTRECOVERABLE => "State not recoverable",

        #[cfg(any(target_os = "macos", target_os = "ios",))]
        libc::EOWNERDEAD => "Previous owner died",

        #[cfg(any(target_os = "macos", target_os = "ios",))]
        libc::ENOTSUP => "Operation not supported",

        #[cfg(any(target_os = "macos", target_os = "ios",))]
        libc::EPROCLIM => "Too many processes",

        #[cfg(any(target_os = "macos", target_os = "ios",))]
        libc::EUSERS => "Too many users",

        #[cfg(any(target_os = "macos", target_os = "ios",))]
        libc::EDQUOT => "Disc quota exceeded",

        #[cfg(any(target_os = "macos", target_os = "ios",))]
        libc::ESTALE => "Stale NFS file handle",

        #[cfg(any(target_os = "macos", target_os = "ios",))]
        libc::EREMOTE => "Too many levels of remote in path",

        #[cfg(any(target_os = "macos", target_os = "ios",))]
        EBADRPC => "RPC struct is bad",

        #[cfg(any(target_os = "macos", target_os = "ios",))]
        ERPCMISMATCH => "RPC version wrong",

        #[cfg(any(target_os = "macos", target_os = "ios",))]
        EPROGUNAVAIL => "RPC prog. not avail",

        #[cfg(any(target_os = "macos", target_os = "ios",))]
        EPROGMISMATCH => "Program version wrong",

        #[cfg(any(target_os = "macos", target_os = "ios",))]
        EPROCUNAVAIL => "Bad procedure for program",

        #[cfg(any(target_os = "macos", target_os = "ios",))]
        EFTYPE => "Inappropriate file type or format",

        #[cfg(any(target_os = "macos", target_os = "ios",))]
        EAUTH => "Authentication error",

        #[cfg(any(target_os = "macos", target_os = "ios",))]
        ECANCELED => "Operation canceled",

        #[cfg(any(target_os = "macos", target_os = "ios"))]
        EPWROFF => "Device power is off",

        #[cfg(any(target_os = "macos", target_os = "ios"))]
        EDEVERR => "Device error, e.g. paper out",

        #[cfg(any(target_os = "macos", target_os = "ios"))]
        libc::EBADEXEC => "Bad executable",

        #[cfg(any(target_os = "macos", target_os = "ios"))]
        libc::EBADARCH => "Bad CPU type in executable",

        #[cfg(any(target_os = "macos", target_os = "ios"))]
        libc::ESHLIBVERS => "Shared library version mismatch",

        #[cfg(any(target_os = "macos", target_os = "ios"))]
        libc::EBADMACHO => "Malformed Macho file",

        #[cfg(any(target_os = "macos", target_os = "ios"))]
        libc::EMULTIHOP => "Reserved",

        #[cfg(any(target_os = "macos", target_os = "ios",))]
        libc::ENODATA => "No message available on STREAM",

        #[cfg(any(target_os = "macos", target_os = "ios"))]
        libc::ENOLINK => "Reserved",

        #[cfg(any(target_os = "macos", target_os = "ios",))]
        libc::ENOSR => "No STREAM resources",

        #[cfg(any(target_os = "macos", target_os = "ios",))]
        libc::ENOSTR => "Not a STREAM",

        #[cfg(any(target_os = "macos", target_os = "ios",))]
        libc::ETIME => "STREAM ioctl timeout",

        #[cfg(any(target_os = "macos", target_os = "ios",))]
        libc::EOPNOTSUPP => "Operation not supported on socket",

        #[cfg(any(target_os = "macos", target_os = "ios"))]
        libc::ENOPOLICY => "No such policy registered",

        #[cfg(any(target_os = "macos", target_os = "ios"))]
        libc::EQFULL => "Interface output queue is full",

        _ => "unknown",
    }
}

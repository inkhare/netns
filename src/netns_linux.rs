use super::errno;
use libc::{c_char, c_uint};

pub const CLONE_NEWUTS: i32 = 0x04000000;
pub const CLONE_NEWIPC: i32 = 0x08000000;
pub const CLONE_NEWUSER: i32 = 0x10000000;
pub const CLONE_NEWPID: i32 = 0x20000000;
pub const CLONE_NEWNET: i32 = 0x40000000;

pub const O_RDONLY: i32 = 0x000000000;
pub const O_WRONLY: i32 = 0x000000001;
pub const O_RDWR: i32 = 0x000000002;
pub const O_CREAT: i32 = 0x000000100;

pub fn gettid() -> i32 {
    unsafe { libc::syscall(libc::SYS_gettid) as i32 }
}

pub fn getpid() -> i32 {
    unsafe { libc::getpid() }
}

pub fn unshare(flags: i32) -> errno::Result<i32> {
    let res = unsafe { libc::unshare(flags) };
    errno::Errno::result(res)
}

pub fn setns(fd: i32, nstype: i32) -> errno::Result<i32> {
    let res = unsafe { libc::setns(fd, nstype) };
    errno::Errno::result(res)
}

pub fn close(fd: i32) -> errno::Result<i32> {
    let res = unsafe { libc::close(fd) };
    errno::Errno::result(res)
}

pub fn open(path: String, oflag: i32, mode: i32) -> errno::Result<i32> {
    let fd = unsafe { libc::open(path.as_ptr() as *const c_char, oflag, mode as c_uint) };
    errno::Errno::result(fd)
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NetNS {
    fd: i32,
    path: String,
}

impl Drop for NetNS {
    fn drop(&mut self) {
        let _ = close(self.fd);
    }
}

impl NetNS {
    #[allow(dead_code)]
    pub fn new() -> errno::Result<NetNS> {
        match unshare(CLONE_NEWNET) {
            Ok(_v) => {}
            Err(e) => {
                return Err(e);
            }
        }
        NetNS::get()
    }

    pub fn get() -> errno::Result<NetNS> {
        return NetNS::get_from_thread(getpid(), gettid());
    }

    pub fn set(ns: &NetNS) -> Result<(), errno::Error> {
        match setns(ns.fd, CLONE_NEWNET) {
            Ok(_v) => Ok(()),
            Err(e) => {
                return Err(e);
            }
        }
    }

    pub fn get_from_thread(pid: i32, tid: i32) -> errno::Result<NetNS> {
        return NetNS::get_from_path(&format!("/proc/{}/task/{}/ns/net", pid, tid));
    }

    pub fn get_from_process(pid: i32) -> errno::Result<NetNS> {
        return NetNS::get_from_path(&format!("/proc/{}/ns/net", pid));
    }

    pub fn get_from_path(path: &String) -> errno::Result<NetNS> {
        let fd = open(path.clone(), O_RDONLY, 0);
        match fd {
            Ok(v) => {
                return Ok(NetNS {
                    fd: v,
                    path: path.clone(),
                });
            }
            Err(e) => {
                return Err(e);
            }
        }
    }
}

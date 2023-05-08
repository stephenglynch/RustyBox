use std::os::unix::prelude::OsStrExt;
use std::path::Path;
use std::ptr::null_mut;
use std::io;
use std::mem::MaybeUninit;
use libc;

pub fn getgroups() -> Vec<u32> {
    let n = 32;
    let mut v = Vec::with_capacity(n);
    let p: *mut u32 = v.as_mut_ptr();

    loop {
        let len = unsafe { libc::getgroups(n as i32, p) };
        if len > 0 {
            unsafe { v.set_len(len as usize) };
            break;
        }

        // Find out true size
        let new_n = unsafe { libc::getgroups(0, null_mut()) } as usize;
        let n_extend = n - new_n;
        v.reserve(n_extend);
    }

    return v;
}

pub fn geteuid() -> u32 {
    unsafe { libc::geteuid() }
}

pub fn getegid() -> u32 {
    unsafe { libc::getegid() }
}

pub fn stat(path: &Path) -> io::Result<libc::stat> {
    let mut st = MaybeUninit::uninit();
    let path = path.as_os_str().as_bytes().as_ptr() as *const i8;
    let ret = unsafe { libc::stat(path, st.as_mut_ptr()) };
    if ret == 0 {
        Ok(unsafe { st.assume_init() })
    } else {
        Err(io::Error::last_os_error())
    }
}

pub fn isatty(fd: i32) -> io::Result<bool> {
    let ret = unsafe { libc::isatty(fd) };
    if ret == 1 {
        Ok(true)
    } else {
        let err = io::Error::last_os_error();
        let errno = err.raw_os_error().unwrap();
        if errno == libc::ENOTTY {
            Ok(false)
        } else {
            Err(err)
        }
    }
}
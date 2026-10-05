use std::ffi::CString;
use std::io;

pub fn enter_new_uts() -> io::Result<()> {
    let ret = unsafe { libc::unshare(libc::CLONE_NEWUTS) };
    if ret == -1 {
        Err(io::Error::last_os_error())
    } else {
        Ok(())
    }
}

pub fn set_hostname(hostname: &str) -> io::Result<()> {
    let c_string = CString::new(hostname)
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "hostname contains null byte"))?;
    let ret = unsafe { libc::sethostname(c_string.as_ptr(), hostname.len()) };
    if ret == -1 {
        Err(io::Error::last_os_error())
    } else {
        Ok(())
    }
}

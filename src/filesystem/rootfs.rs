use std::ffi::CString;
use std::io;
use std::path::Path;

use std::os::unix::ffi::OsStrExt;

pub fn enter_rootfs(path: &Path) -> io::Result<()> {
    let c_path = CString::new(path.as_os_str().as_bytes())
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "path contains null byte"))?;
    let ret: i32 = unsafe { libc::chroot(c_path.as_ptr()) };
    if ret == -1 {
        return Err(io::Error::last_os_error());
    }
    let ret: i32 = unsafe { libc::chdir(CString::new("/").unwrap().as_ptr()) };
    if ret == -1 {
        return Err(io::Error::last_os_error());
    }
    Ok(())
}

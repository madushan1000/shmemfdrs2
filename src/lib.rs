use std::ffi::CStr;
use std::fs::File;
use std::io;
use std::os::fd::FromRawFd;

/// Create a new empty file for shared memory.
///
/// # Implementation
///
/// - On Linux `memfd_create` is used.
/// - On FreeBSD `shm_open` with `SHM_ANON` is used.
/// - Other platform use `shm_open` followed by `shm_unlink`.
pub fn create_shmem<T: AsRef<CStr>>(name: T) -> io::Result<File> {
    let fd;

    #[cfg(any(target_os = "linux", target_os = "android"))]
    unsafe {
        fd = make_fd(libc::memfd_create(
            name.as_ref().as_ptr(),
            libc::MFD_CLOEXEC,
        ))?;
    }

    #[cfg(target_os = "freebsd")]
    unsafe {
        let _ = name;
        fd = make_fd(libc::shm_open(
            libc::SHM_ANON,
            libc::O_CREAT | libc::O_RDWR | libc::O_EXCL,
            0o600,
        ))?;
    }

    #[cfg(not(any(target_os = "freebsd", target_os = "linux", target_os = "android")))]
    unsafe {
        fd = make_fd(libc::shm_open(
            name.as_ref().as_ptr(),
            libc::O_CREAT | libc::O_RDWR | libc::O_EXCL,
            0o600,
        ))?;

        if libc::shm_unlink(name.as_ref().as_ptr()) == -1 {
            return Err(io::Error::last_os_error());
        }
    }

    Ok(fd)
}

unsafe fn make_fd(fd: libc::c_int) -> io::Result<File> {
    if fd == -1 {
        Err(io::Error::last_os_error())
    } else {
        Ok(unsafe { File::from_raw_fd(fd) })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;

    #[test]
    fn test_create_shmem() {
        create_shmem(CString::new("/helloworld").unwrap()).unwrap();
    }
}

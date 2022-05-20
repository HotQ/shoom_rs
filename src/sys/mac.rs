use std::ffi;
use std::ptr;

pub type Error = crate::Error;
pub type Result<T> = std::result::Result<T, Error>;

pub(crate) unsafe fn create_or_open(
    create: bool,
    path: String,
    size: usize,
) -> Result<(*mut ffi::c_void,)> {
    let path = ffi::CString::new(path.clone()).map_err(|_| Error::kErrorFFIFailed)?;

    if create {
        // shm segments persist across runs, and macOS will refuse
        // to ftruncate an existing shm segment, so to be on the safe
        // side, we unlink it beforehand.
        // TODO(amos) check errno while ignoring ENOENT?
        let ret = libc::shm_unlink(path.as_ptr());
        if ret < 0 {
            if *(libc::__error()) != libc::ENOENT {
                return Err(Error::kErrorCreationFailed);
            }
        }
    }

    let flags = if create {
        libc::O_CREAT | libc::O_RDWR
    } else {
        libc::O_RDONLY
    };

    let fd = libc::shm_open(path.as_ptr() as _, flags, 0755);

    if fd < 0 {
        if create {
            return Err(Error::kErrorCreationFailed);
        } else {
            return Err(Error::kErrorOpeningFailed);
        }
    }

    if create {
        // this is the only way to specify the size of a
        // newly-created POSIX shared memory object
        let ret = libc::ftruncate(fd, size as _);
        if ret != 0 {
            return Err(Error::kErrorCreationFailed);
        }
    }

    let prot = if create {
        libc::PROT_READ | libc::PROT_WRITE
    } else {
        libc::PROT_READ
    };

    let memory = libc::mmap(ptr::null_mut(), size as _, prot, libc::MAP_SHARED, fd, 0);

    if memory == libc::MAP_FAILED {
        return Err(Error::kErrorMappingFailed);
    };

    Ok((memory,))
}

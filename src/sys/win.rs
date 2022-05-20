use std::ffi;
use std::ptr;
use winapi::shared::minwindef::{DWORD, FALSE};
use winapi::um::handleapi;
use winapi::um::memoryapi;
use winapi::um::winbase::{CreateFileMappingA, OpenFileMappingA};
use winapi::um::winnt::{HANDLE, PAGE_READWRITE};

pub type Error = crate::Error;
pub type Result<T> = std::result::Result<T, Error>;

pub(crate) unsafe fn create_or_open(
    create: bool,
    path: String,
    size: usize,
) -> Result<(*mut ffi::c_void, HANDLE)> {
    let path = ffi::CString::new(path.clone()).map_err(|_| Error::kErrorFFIFailed)?;
    let handle: HANDLE;
    if create {
        let size_high_order: DWORD = 0;
        let size_low_order: DWORD = size as _;

        handle = CreateFileMappingA(
            handleapi::INVALID_HANDLE_VALUE,
            ptr::null_mut(),
            PAGE_READWRITE,
            size_high_order,
            size_low_order,
            path.as_ptr(),
        );

        if handle.is_null() {
            return Err(Error::kErrorCreationFailed);
        }
    } else {
        handle = OpenFileMappingA(memoryapi::FILE_MAP_READ, FALSE, path.as_ptr());
        if handle.is_null() {
            return Err(Error::kErrorOpeningFailed);
        }
    }

    let access: DWORD = if create {
        memoryapi::FILE_MAP_ALL_ACCESS
    } else {
        memoryapi::FILE_MAP_READ
    };

    let memory = memoryapi::MapViewOfFile(handle, access, 0, 0, size);
    if memory.is_null() {
        return Err(Error::kErrorMappingFailed);
    }
    Ok((memory, handle))
}

pub(crate) unsafe fn unmap(data: *mut ffi::c_void, handle: HANDLE) {
    if !data.is_null() {
        memoryapi::UnmapViewOfFile(data);
    }
    handleapi::CloseHandle(handle);
}

#![feature(stmt_expr_attributes)]

pub mod error;
pub mod sys;

use std::ffi;
use std::ptr;
#[cfg(target_os = "windows")]
use winapi::um::winnt::HANDLE;

pub type Error = error::ShoomError;
pub type ErrorType = error::ShoomErrorType;
pub type Result<T> = std::result::Result<T, Error>;
pub struct Shoom {
    data: *mut ffi::c_void,
    size: usize,
    path: String,
    #[cfg(target_os = "windows")]
    handle: HANDLE,
    #[cfg(target_os = "macos")]
    fd: libc::c_int,
}

impl Shoom {
    pub fn new(path: &str, size: usize) -> Self {
        Self {
            data: ptr::null_mut(),
            size,
            #[cfg(target_os = "windows")]
            path: path.to_string(),
            #[cfg(target_os = "macos")]
            path: format!("/{}", path),
            #[cfg(target_os = "windows")]
            handle: ptr::null_mut(),
            #[cfg(target_os = "macos")]
            fd: -1,
        }
    }
    pub unsafe fn create_or_open(&mut self, create: bool) -> Result<*mut ffi::c_void> {
        let rs = sys::create_or_open(create, self.path.clone(), self.size)?;

        self.data = rs.0;
        #[cfg(target_os = "windows")]
        {
            self.handle = rs.1;
        }
        #[cfg(target_os = "macos")]
        {
            self.fd = rs.1;
        }

        Ok(rs.0)
    }

    pub unsafe fn create(&mut self) -> Result<*mut ffi::c_void> {
        self.create_or_open(true)
    }
    pub unsafe fn open(&mut self) -> Result<*mut ffi::c_void> {
        self.create_or_open(false)
    }

    pub fn path(&mut self) -> String {
        self.path.clone()
    }
    pub fn size(&mut self) -> usize {
        self.size
    }
    pub fn data(&mut self) -> *mut ffi::c_void {
        self.data
    }
}

impl Drop for Shoom {
    fn drop(&mut self) {
        unsafe {
            sys::unmap(
                self.data,
                #[cfg(target_os = "windows")]
                self.handle,
                #[cfg(target_os = "macos")]
                self.fd,
                #[cfg(target_os = "macos")]
                self.size,
                #[cfg(target_os = "macos")]
                self.path.clone(),
            )
        }
    }
}

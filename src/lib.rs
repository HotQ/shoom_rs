pub mod error;
pub mod sys;

use std::ffi;
use std::ptr;

pub type Error = error::ShoomError;
pub type Result<T> = std::result::Result<T, Error>;
pub struct Shoom {
    data: *mut ffi::c_void,
    size: usize,
    path: String,
}

impl Shoom {
    pub fn new<T: std::fmt::Display>(path: T, size: usize) -> Self {
        Self {
            data: ptr::null_mut(),
            size,
            path: format!("/{}", path),
        }
    }
    pub unsafe fn create_or_open(&mut self, create: bool) -> Result<*mut ffi::c_void> {
        let memory = sys::create_or_open(create, self.path.clone(), self.size)?;
        self.data = memory;
        Ok(memory)
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

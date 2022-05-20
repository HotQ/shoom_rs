#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub enum ShoomError {
    kOK = 0,
    kErrorCreationFailed = 100,
    kErrorMappingFailed = 110,
    kErrorOpeningFailed = 120,
    kErrorFFIFailed = 130,
}

use std::fmt;

impl fmt::Display for ShoomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let info: &'static str = match *self {
            Self::kOK => "ok",
            Self::kErrorCreationFailed => "creation failed",
            Self::kErrorMappingFailed => "mapping failed",
            Self::kErrorOpeningFailed => "opening failed",
            Self::kErrorFFIFailed => "ffi failed",
        };

        write!(f, "{}", info)
    }
}

impl std::error::Error for ShoomError {}

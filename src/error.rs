#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub enum ShoomErrorType {
    kOK = 0,
    kErrorCreationFailed = 100,
    kErrorMappingFailed = 110,
    kErrorOpeningFailed = 120,
    kErrorFFIFailed = 130,
}

impl ShoomErrorType {
    pub fn context<T: ToString>(self, context: T) -> ShoomError {
        ShoomError {
            error_type: self,
            context: context.to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ShoomError {
    pub error_type: ShoomErrorType,
    pub context: String,
}

impl From<ShoomErrorType> for ShoomError {
    fn from(error_type: ShoomErrorType) -> Self {
        error_type.context("")
    }
}

use std::fmt;

impl fmt::Display for ShoomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let error_type: &'static str = match self.error_type {
            ShoomErrorType::kOK => "ok",
            ShoomErrorType::kErrorCreationFailed => "creation failed",
            ShoomErrorType::kErrorMappingFailed => "mapping failed",
            ShoomErrorType::kErrorOpeningFailed => "opening failed",
            ShoomErrorType::kErrorFFIFailed => "ffi failed",
        };

        write!(
            f,
            "{}({})",
            error_type,
            if self.context.is_empty() {
                ""
            } else {
                self.context.as_str()
            }
        )
    }
}

impl std::error::Error for ShoomError {}

#[cfg(target_os = "macos")]
pub(crate) mod mac;
#[cfg(target_os = "macos")]
pub(crate) use mac::*;

#[cfg(target_os = "windows")]
pub(crate) mod win;

#[cfg(target_os = "windows")]
pub(crate) use win::*;

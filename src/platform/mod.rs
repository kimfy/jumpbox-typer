use crate::types::{KeyboardLayout, SystemCheck};

#[cfg(target_os = "linux")]
mod linux;
#[cfg(not(target_os = "linux"))]
mod unsupported;

#[cfg(target_os = "linux")]
use linux as backend;
#[cfg(not(target_os = "linux"))]
use unsupported as backend;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccessRequest {
    CheckOnly,
    RequestIfNeeded,
}

pub fn check_system(access_request: AccessRequest) -> SystemCheck {
    backend::check_system(access_request)
}

pub fn prepare_typing() -> Result<(), String> {
    backend::prepare_typing()
}

pub fn type_character(ch: char, layout: KeyboardLayout) -> Result<(), String> {
    backend::type_character(ch, layout)
}

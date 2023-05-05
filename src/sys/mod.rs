#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(clippy::useless_transmute)]
#![allow(clippy::too_many_arguments)]

mod version;

#[cfg(target_os = "linux")]
mod linux_sys;
#[cfg(target_os = "linux")]
pub use linux_sys::*;

#[cfg(target_os = "windows")]
mod windows_sys;
#[cfg(target_os = "windows")]
pub use windows_sys::*;

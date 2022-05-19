#[macro_use]
pub mod dictionary;
pub mod channel_layout;
pub mod chroma;
pub mod color;
pub mod error;
pub mod format;
pub mod frame;
pub mod image;
pub mod interrupt;
pub mod mathematics;
pub mod media;
pub mod option;
pub mod picture;
pub mod range;
pub mod rational;
pub mod time;

#[cfg(feature = "log")]
pub mod log;

use std::{
	ffi::{CStr, CString, OsStr},
	str::from_utf8_unchecked,
};

use crate::ffi::*;

#[inline(always)]
pub fn version() -> u32 {
	unsafe { avutil_version() }
}

#[inline(always)]
pub fn configuration() -> &'static str {
	unsafe { from_utf8_unchecked(CStr::from_ptr(avutil_configuration()).to_bytes()) }
}

#[inline(always)]
pub fn license() -> &'static str {
	unsafe { from_utf8_unchecked(CStr::from_ptr(avutil_license()).to_bytes()) }
}

#[cfg(unix)]
pub fn from_os_str(path_or_url: impl AsRef<OsStr>) -> CString {
	use std::os::unix::ffi::OsStrExt;
	CString::new(path_or_url.as_ref().as_bytes()).unwrap()
}

#[cfg(not(unix))]
pub fn from_os_str(path_or_url: impl AsRef<OsStr>) -> CString {
	CString::new(path_or_url.as_ref().to_str().unwrap()).unwrap()
}

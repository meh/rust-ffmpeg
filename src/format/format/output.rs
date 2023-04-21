use std::{
	ffi::{CStr, CString},
	path::Path,
	ptr,
	str::from_utf8_unchecked,
};

use libc::c_void;

use super::Flags;
use crate::{codec, ffi::*, media};

#[derive(Copy, Clone)]
pub struct Output {
	ptr: *const AVOutputFormat,
}

unsafe impl Send for Output {}
unsafe impl Sync for Output {}

impl Output {
	pub unsafe fn wrap(ptr: *const AVOutputFormat) -> Self {
		Output { ptr }
	}

	pub unsafe fn as_ptr(&self) -> *const AVOutputFormat {
		self.ptr
	}
}

impl Output {
	pub fn name(&self) -> &str {
		unsafe { from_utf8_unchecked(CStr::from_ptr((*self.as_ptr()).name).to_bytes()) }
	}

	pub fn description(&self) -> &str {
		unsafe { from_utf8_unchecked(CStr::from_ptr((*self.as_ptr()).long_name).to_bytes()) }
	}

	pub fn extensions(&self) -> Vec<&str> {
		unsafe {
			let ptr = (*self.as_ptr()).extensions;

			if ptr.is_null() {
				Vec::new()
			}
			else {
				from_utf8_unchecked(CStr::from_ptr(ptr).to_bytes()).split(',').collect()
			}
		}
	}

	pub fn mime_types(&self) -> Vec<&str> {
		unsafe {
			let ptr = (*self.as_ptr()).mime_type;

			if ptr.is_null() {
				Vec::new()
			}
			else {
				from_utf8_unchecked(CStr::from_ptr(ptr).to_bytes()).split(',').collect()
			}
		}
	}

	pub fn codec<P: AsRef<Path>>(&self, path: &P, kind: media::Type) -> codec::Id {
		// XXX: use to_cstring when stable
		let path = CString::new(path.as_ref().as_os_str().to_str().unwrap()).unwrap();

		unsafe {
			codec::Id::from(av_guess_codec(
				self.as_ptr(),
				ptr::null(),
				path.as_ptr(),
				ptr::null(),
				kind.into(),
			))
		}
	}

	pub fn flags(&self) -> Flags {
		unsafe { Flags::from_bits_truncate((*self.as_ptr()).flags) }
	}
}

pub struct Iter {
	state: *mut c_void,
}

impl Iter {
	pub fn new() -> Self {
		Iter { state: ptr::null_mut() }
	}
}

impl Default for Iter {
	fn default() -> Self {
		Self::new()
	}
}

impl Iterator for Iter {
	type Item = Output;

	fn next(&mut self) -> Option<<Self as Iterator>::Item> {
		unsafe {
			let ptr = av_muxer_iterate(&mut self.state as *mut _);

			if ptr.is_null() {
				None
			}
			else {
				Some(Output::wrap(ptr))
			}
		}
	}
}

pub fn all() -> Iter {
	Iter::new()
}

pub fn by_name(name: impl Into<String>) -> impl Iterator<Item = Output> {
	let name = name.into();
	all().filter(move |i| i.name() == name)
}

pub fn by_mime(mime: impl Into<String>) -> impl Iterator<Item = Output> {
	let mime = mime.into();
	all().filter(move |i| i.mime_types().contains(&mime.as_ref()))
}

pub fn by_extension(ext: impl Into<String>) -> impl Iterator<Item = Output> {
	let ext = ext.into();
	all().filter(move |i| i.extensions().contains(&ext.as_ref()))
}
